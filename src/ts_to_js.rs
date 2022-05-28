use std::{
    io::{Cursor, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use swc_common::{
    self,
    comments::SingleThreadedComments,
    errors::{EmitterWriter, Handler},
    sync::Lrc,
    BytePos, Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

struct SharedWriter<W>(Arc<Mutex<W>>);

impl<W: Write> Write for SharedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        (*self.0.lock().unwrap()).write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        (*self.0.lock().unwrap()).flush()
    }
}

pub fn ts_to_js(path: &Path, js: &str) -> Result<String, anyhow::Error> {
    let cm: Lrc<SourceMap> = Default::default();

    cm.new_source_file(path.to_owned().into(), js.to_owned());

    let error_output = Arc::new(Mutex::new(Cursor::new(Vec::new())));
    let emitter = EmitterWriter::new(
        Box::new(SharedWriter(Arc::clone(&error_output))),
        None,
        true,
        false,
    );
    let handler = Handler::with_emitter(true, false, Box::new(emitter));

    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            ..Default::default()
        }),
        Default::default(),
        StringInput::new(js, BytePos(0), BytePos(0)),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    let mut had_error = false;
    for e in parser.take_errors() {
        had_error = true;
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit());

    let module = match module {
        Ok(module) if !had_error => module,
        _ => {
            let error_msg =
                String::from_utf8(std::mem::take(&mut *error_output.lock().unwrap()).into_inner())?;
            return Err(anyhow::anyhow!(
                "Failed to transpile js to ts:\n{error_msg}"
            ));
        }
    };

    let globals = Globals::default();
    let ts = GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // Optionally transforms decorators here before the resolver pass
        // as it might produce runtime declarations.

        // Conduct identifier scope analysis
        let module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        let module = module.fold_with(&mut strip(top_level_mark));

        // Fix up any identifiers with the same name, but different contexts
        let module = module.fold_with(&mut hygiene());

        // Ensure that we have enough parenthesis.
        let module = module.fold_with(&mut fixer(Some(&comments)));

        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: swc_ecma_codegen::Config {
                    minify: false,
                    ..Default::default()
                },
                cm: cm.clone(),
                comments: Some(&comments),
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_module(&module).unwrap();
        }

        String::from_utf8(buf)
    })?;

    Ok(ts)
}
