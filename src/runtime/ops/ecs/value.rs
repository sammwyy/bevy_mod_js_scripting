use std::{any::TypeId, cell::RefCell, rc::Rc};

use anyhow::{format_err, Context};
use bevy::prelude::default;
use bevy_ecs_dynamic::reflect_value_ref::ReflectValueRef;
use bevy_reflect::{ReflectRef, TypeRegistryArc};
use bevy_reflect_fns::{PassMode, ReflectArg, ReflectMethods};
use type_map::TypeMap;

use super::{
    types::{
        JsValueRef, JsValueRefs, Primitive, ReflectArgIntermediate, ReflectArgIntermediateValue,
    },
    WithValueRefs,
};

macro_rules! try_downcast_leaf_get {
    ($value:ident for $($ty:ty $(,)?),*) => {
        $(if let Some(value) = $value.downcast_ref::<$ty>() {
            let value = serde_json::to_value(value)?;
            return Ok(value);
        })*
    };
}

macro_rules! try_downcast_leaf_set {
    ($value:ident <- $new_value:ident for $($ty:ty $(,)?),*) => {
        $(if let Some(value) = $value.downcast_mut::<$ty>() {
            *value = serde_json::from_value($new_value)?;
            return Ok(serde_json::Value::Null);
        })*
    };
}

pub fn ecs_value_ref_get(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    op_state.with_refs_and_funcs(|_, value_refs, reflect_functions| {
        // Parse args
        let (value_ref, path): (JsValueRef, String) =
            serde_json::from_value(args).context("parse args")?;

        // Load the type registry
        let type_registry = world.resource::<TypeRegistryArc>();
        let type_registry = type_registry.read();

        // Get the reflect value ref from the JS argument
        let value_ref = value_refs
            .get(value_ref.key)
            .ok_or_else(|| format_err!("Value ref doesn't exist"))?
            .clone();

        // See if we can find any reflect methods for this type in the type registry
        let reflect_methods =
            type_registry.get_type_data::<ReflectMethods>(value_ref.get(world)?.type_id());

        // If we found methods for this type
        if let Some(reflect_methods) = reflect_methods {
            let method_name = path.trim_start_matches('.');
            // If the path we are accessing is a method on the type
            if let Some(reflect_function) = reflect_methods.get(method_name.trim_start_matches('.'))
            {
                // Return a method reference
                let value = JsValueRef {
                    key: value_refs.insert(value_ref),
                    function: Some(reflect_functions.insert(reflect_function.clone())),
                };

                return Ok(serde_json::to_value(&value)?);
            }
        }

        // If we didn't find a method, add the path to our value ref
        let value_ref = value_ref.append_path(&path, world)?;

        // Try to downcast the value to a primitive
        {
            let value = value_ref.get(world)?;

            try_downcast_leaf_get!(value for
                u8, u16, u32, u64, u128, usize,
                i8, i16, i32, i64, i128, isize,
                String, char, bool, f32, f64
            );
        }

        // If not a primitive, just return a new value ref
        let object = JsValueRef {
            key: value_refs.insert(value_ref),
            function: None,
        };

        Ok(serde_json::to_value(object)?)
    })
}

pub fn ecs_value_ref_set(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Parse args
    let (value_ref, path, new_value): (JsValueRef, String, serde_json::Value) =
        serde_json::from_value(args).context("parse args")?;

    let value_refs = op_state.entry::<JsValueRefs>().or_insert_with(default);

    // Get the value ref from the JS arg
    let value_ref = value_refs
        .get(value_ref.key)
        .ok_or_else(|| format_err!("Value ref doesn't exist"))?
        .clone();

    // Access the provided path on the value ref
    let mut value_ref = value_ref.append_path(&path, world).unwrap();

    // Get the reflect value
    let mut reflect = value_ref.get_mut(world).unwrap();

    // Try to store a primitive in the value
    try_downcast_leaf_set!(reflect <- new_value for
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        String, char, bool, f32, f64
    );

    anyhow::bail!(
        "could not set value reference: type `{}` is not a primitive type",
        reflect.type_name(),
    );
}

pub fn ecs_value_ref_keys(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Parse args
    let (value_ref,): (JsValueRef,) = serde_json::from_value(args).context("parse args")?;

    let value_refs = op_state.entry::<JsValueRefs>().or_insert_with(default);

    // Get the value ref from the JS arg
    let value_ref = value_refs
        .get(value_ref.key)
        .ok_or_else(|| format_err!("Value ref doesn't exist"))?
        .clone();
    let reflect = value_ref.get(world).unwrap();

    // Enumerate the fields of the reflected object
    let fields = match reflect.reflect_ref() {
        ReflectRef::Struct(s) => (0..s.field_len())
            .map(|i| {
                let name = s.name_at(i).ok_or_else(|| {
                    format_err!("misbehaving Reflect impl on `{}`", s.type_name())
                })?;
                Ok(name.to_owned())
            })
            .collect::<anyhow::Result<_>>()?,
        ReflectRef::Tuple(tuple) => (0..tuple.field_len()).map(|i| i.to_string()).collect(),
        ReflectRef::TupleStruct(tuple_struct) => (0..tuple_struct.field_len())
            .map(|i| i.to_string())
            .collect(),
        _ => Vec::new(),
    };

    Ok(serde_json::to_value(fields)?)
}

pub fn ecs_value_ref_to_string(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Parse args
    let (value_ref,): (JsValueRef,) = serde_json::from_value(args).context("parse args")?;

    let value_refs = op_state.entry::<JsValueRefs>().or_insert_with(default);

    // Get the value ref from the JS arg
    let value_ref = value_refs
        .get(value_ref.key)
        .ok_or_else(|| format_err!("Value ref doesn't exist"))?
        .clone();
    let reflect = value_ref.get(world).unwrap();

    Ok(serde_json::Value::String(format!("{reflect:?}")))
}

pub fn ecs_value_ref_call(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Parse args
    let (receiver, args): (JsValueRef, Vec<serde_json::Value>) =
        serde_json::from_value(args).context("parse args")?;

    let ref_not_exist_err = || format_err!("Ref does not exist");

    op_state.with_refs_and_funcs(|_, value_refs, reflect_functions| {
        // Get the receiver's reflect_function
        let method_key = receiver
            .function
            .ok_or_else(|| format_err!("Cannot call non-function ref"))?;
        let method = reflect_functions
            .get_mut(method_key)
            .ok_or_else(ref_not_exist_err)?;

        // Get the receiver's reflect ref
        let receiver = value_refs.get(receiver.key).ok_or_else(ref_not_exist_err)?;

        // Collect the receiver intermediate value
        let receiver_pass_mode = method.signature[0].0;
        let receiver_intermediate = match receiver_pass_mode {
            PassMode::Ref => ReflectArgIntermediateValue::Ref(receiver.get(world).unwrap()),
            PassMode::RefMut => {
                unimplemented!("values passed by mutable reference in reflect fn call")
            }
            PassMode::Owned => ReflectArgIntermediateValue::Owned(receiver.get(world).unwrap()),
        };
        let mut receiver_intermediate = ReflectArgIntermediate::Value(receiver_intermediate);

        // Collect the intermediate values for the arguments
        let mut arg_intermediates = args
            .iter()
            .zip(method.signature.iter().skip(1))
            .map(|(arg, &(pass_mode, type_id))| {
                // Try to cast the arg as a primitive
                let downcast_primitive = match type_id {
                    type_id if type_id == TypeId::of::<f32>() => {
                        Some(Primitive::f32(serde_json::from_value(arg.clone())?))
                    }
                    type_id if type_id == TypeId::of::<f64>() => {
                        Some(Primitive::f64(serde_json::from_value(arg.clone())?))
                    }
                    type_id if type_id == TypeId::of::<i32>() => {
                        Some(Primitive::i32(serde_json::from_value(arg.clone())?))
                    }
                    type_id if type_id == TypeId::of::<u32>() => {
                        Some(Primitive::u32(serde_json::from_value(arg.clone())?))
                    }
                    _ => None,
                };
                // If the arg cast worked, return a primitive arg
                if let Some(primitive) = downcast_primitive {
                    return Ok(ReflectArgIntermediate::Primitive(primitive, pass_mode));
                }

                // Otherwise, try get the arg as a value ref
                let value_ref: JsValueRef = serde_json::from_value(arg.clone())?;
                let value_ref = value_refs
                    .get(value_ref.key)
                    .ok_or_else(|| format_err!("Value ref doesn't exist"))?;

                let value_ref = match pass_mode {
                    PassMode::Ref => {
                        ReflectArgIntermediateValue::Ref(value_ref.get(world).unwrap())
                    }
                    PassMode::RefMut => {
                        unimplemented!("values passed by mutable reference in reflect fn call")
                    }
                    PassMode::Owned => {
                        ReflectArgIntermediateValue::Owned(value_ref.get(world).unwrap())
                    }
                };

                Ok(ReflectArgIntermediate::Value(value_ref))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        // Collect references to our intermediates as [`ReflectArg`]s
        let mut args: Vec<ReflectArg> = std::iter::once(&mut receiver_intermediate)
            .chain(arg_intermediates.iter_mut())
            .map(|intermediate| intermediate.as_arg())
            .collect();

        // Finally call the method
        let ret = method.call(args.as_mut_slice()).unwrap();
        // And package it's return value as a standalone reflect ref
        let ret = Rc::new(RefCell::new(ret));
        let ret = ReflectValueRef::free(ret);

        // Drop our intermediates and args so that we can use `value_refs` again, below.
        drop(args);
        drop(arg_intermediates);
        drop(receiver_intermediate);

        // Return our resulting value ref
        let ret = JsValueRef {
            key: value_refs.insert(ret),
            function: None,
        };

        Ok(serde_json::to_value(ret)?)
    })
}

pub fn ecs_value_ref_free(
    op_state: &mut TypeMap,
    _script_info: &crate::runtime::ScriptInfo,
    _world: &mut bevy::prelude::World,
    args: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Parse args
    let (value_ref,): (JsValueRef,) = serde_json::from_value(args).context("parse args")?;

    op_state.with_refs_and_funcs(|_, value_refs, reflect_functions| {
        value_refs.remove(value_ref.key);
        if let Some(func) = value_ref.function {
            reflect_functions.remove(func);
        }
    });

    Ok(serde_json::Value::Null)
}