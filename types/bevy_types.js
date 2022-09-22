const AnimationPlayer = { typeName: "bevy_animation::AnimationPlayer" };
const HandleAnimationClip = { typeName: "bevy_asset::handle::Handle<bevy_animation::AnimationClip>" };
const HandleAudioSink = { typeName: "bevy_asset::handle::Handle<bevy_audio::audio_output::AudioSink>" };
const HandleAudioSource = { typeName: "bevy_asset::handle::Handle<bevy_audio::audio_source::AudioSource>" };
const HandleGltf = { typeName: "bevy_asset::handle::Handle<bevy_gltf::Gltf>" };
const HandleGltfMesh = { typeName: "bevy_asset::handle::Handle<bevy_gltf::GltfMesh>" };
const HandleGltfNode = { typeName: "bevy_asset::handle::Handle<bevy_gltf::GltfNode>" };
const HandleGltfPrimitive = { typeName: "bevy_asset::handle::Handle<bevy_gltf::GltfPrimitive>" };
const HandleStandardMaterial = { typeName: "bevy_asset::handle::Handle<bevy_pbr::pbr_material::StandardMaterial>" };
const HandleMesh = { typeName: "bevy_asset::handle::Handle<bevy_render::mesh::mesh::Mesh>" };
const HandleSkinnedMeshInverseBindposes = { typeName: "bevy_asset::handle::Handle<bevy_render::mesh::mesh::skinning::SkinnedMeshInverseBindposes>" };
const HandleShader = { typeName: "bevy_asset::handle::Handle<bevy_render::render_resource::shader::Shader>" };
const HandleImage = { typeName: "bevy_asset::handle::Handle<bevy_render::texture::image::Image>" };
const HandleDynamicScene = { typeName: "bevy_asset::handle::Handle<bevy_scene::dynamic_scene::DynamicScene>" };
const HandleScene = { typeName: "bevy_asset::handle::Handle<bevy_scene::scene::Scene>" };
const HandleColorMaterial = { typeName: "bevy_asset::handle::Handle<bevy_sprite::mesh2d::color_material::ColorMaterial>" };
const HandleTextureAtlas = { typeName: "bevy_asset::handle::Handle<bevy_sprite::texture_atlas::TextureAtlas>" };
const HandleFont = { typeName: "bevy_asset::handle::Handle<bevy_text::font::Font>" };
const HandleFontAtlasSet = { typeName: "bevy_asset::handle::Handle<bevy_text::font_atlas_set::FontAtlasSet>" };
const HandleId = { typeName: "bevy_asset::handle::HandleId" };
const Name = { typeName: "bevy_core::name::Name" };
const ClearColor = { typeName: "bevy_core_pipeline::clear_color::ClearColor" };
const ClearColorConfig = { typeName: "bevy_core_pipeline::clear_color::ClearColorConfig" };
const Camera2d = { typeName: "bevy_core_pipeline::core_2d::camera_2d::Camera2d" };
const Camera3d = { typeName: "bevy_core_pipeline::core_3d::camera_3d::Camera3d" };
const Camera3dDepthLoadOp = { typeName: "bevy_core_pipeline::core_3d::camera_3d::Camera3dDepthLoadOp" };
const GltfExtras = { typeName: "bevy_gltf::GltfExtras" };
const Children = { typeName: "bevy_hierarchy::components::children::Children" };
const Parent = { typeName: "bevy_hierarchy::components::parent::Parent" };
const CubemapVisibleEntities = { typeName: "bevy_pbr::bundle::CubemapVisibleEntities" };
const AmbientLight = { typeName: "bevy_pbr::light::AmbientLight" };
const DirectionalLight = { typeName: "bevy_pbr::light::DirectionalLight" };
const DirectionalLightShadowMap = { typeName: "bevy_pbr::light::DirectionalLightShadowMap" };
const PointLight = { typeName: "bevy_pbr::light::PointLight" };
const PointLightShadowMap = { typeName: "bevy_pbr::light::PointLightShadowMap" };
const SpotLight = { typeName: "bevy_pbr::light::SpotLight" };
const Camera = { typeName: "bevy_render::camera::camera::Camera" };
const CameraRenderGraph = { typeName: "bevy_render::camera::camera::CameraRenderGraph" };
const DepthCalculation = { typeName: "bevy_render::camera::camera::DepthCalculation" };
const Viewport = { typeName: "bevy_render::camera::camera::Viewport" };
const OrthographicProjection = { typeName: "bevy_render::camera::projection::OrthographicProjection" };
const PerspectiveProjection = { typeName: "bevy_render::camera::projection::PerspectiveProjection" };
const Projection = { typeName: "bevy_render::camera::projection::Projection" };
const ScalingMode = { typeName: "bevy_render::camera::projection::ScalingMode" };
const WindowOrigin = { typeName: "bevy_render::camera::projection::WindowOrigin" };
const Color = { typeName: "bevy_render::color::Color" };
const SkinnedMesh = { typeName: "bevy_render::mesh::mesh::skinning::SkinnedMesh" };
const Aabb = { typeName: "bevy_render::primitives::Aabb" };
const CubemapFrusta = { typeName: "bevy_render::primitives::CubemapFrusta" };
const Frustum = { typeName: "bevy_render::primitives::Frustum" };
const Msaa = { typeName: "bevy_render::view::Msaa" };
const ComputedVisibility = { typeName: "bevy_render::view::visibility::ComputedVisibility" };
const Visibility = { typeName: "bevy_render::view::visibility::Visibility" };
const VisibleEntities = { typeName: "bevy_render::view::visibility::VisibleEntities" };
const Mesh2dHandle = { typeName: "bevy_sprite::mesh2d::mesh::Mesh2dHandle" };
const Anchor = { typeName: "bevy_sprite::sprite::Anchor" };
const Sprite = { typeName: "bevy_sprite::sprite::Sprite" };
const HorizontalAlign = { typeName: "bevy_text::text::HorizontalAlign" };
const TextAlignment = { typeName: "bevy_text::text::TextAlignment" };
const TextSection = { typeName: "bevy_text::text::TextSection" };
const TextStyle = { typeName: "bevy_text::text::TextStyle" };
const VerticalAlign = { typeName: "bevy_text::text::VerticalAlign" };
const Stopwatch = { typeName: "bevy_time::stopwatch::Stopwatch" };
const Time = { typeName: "bevy_time::time::Time" };
const Timer = { typeName: "bevy_time::timer::Timer" };
const GlobalTransform = { typeName: "bevy_transform::components::global_transform::GlobalTransform" };
const Transform = { typeName: "bevy_transform::components::transform::Transform" };
const FocusPolicy = { typeName: "bevy_ui::focus::FocusPolicy" };
const Interaction = { typeName: "bevy_ui::focus::Interaction" };
const Size = { typeName: "bevy_ui::geometry::Size" };
const SizeVal = { typeName: "bevy_ui::geometry::Size<bevy_ui::ui_node::Val>" };
const UiRectVal = { typeName: "bevy_ui::geometry::UiRect<bevy_ui::ui_node::Val>" };
const AlignContent = { typeName: "bevy_ui::ui_node::AlignContent" };
const AlignItems = { typeName: "bevy_ui::ui_node::AlignItems" };
const AlignSelf = { typeName: "bevy_ui::ui_node::AlignSelf" };
const CalculatedSize = { typeName: "bevy_ui::ui_node::CalculatedSize" };
const Direction = { typeName: "bevy_ui::ui_node::Direction" };
const Display = { typeName: "bevy_ui::ui_node::Display" };
const FlexDirection = { typeName: "bevy_ui::ui_node::FlexDirection" };
const FlexWrap = { typeName: "bevy_ui::ui_node::FlexWrap" };
const JustifyContent = { typeName: "bevy_ui::ui_node::JustifyContent" };
const Overflow = { typeName: "bevy_ui::ui_node::Overflow" };
const PositionType = { typeName: "bevy_ui::ui_node::PositionType" };
const Style = { typeName: "bevy_ui::ui_node::Style" };
const UiColor = { typeName: "bevy_ui::ui_node::UiColor" };
const UiImage = { typeName: "bevy_ui::ui_node::UiImage" };
const Val = { typeName: "bevy_ui::ui_node::Val" };
const Button = { typeName: "bevy_ui::widget::button::Button" };
const ImageMode = { typeName: "bevy_ui::widget::image::ImageMode" };
const Rangef32 = { typeName: "core::ops::range::Range<f32>" };
const OptionString = { typeName: "core::option::Option<alloc::string::String>" };
const Optionf32 = { typeName: "core::option::Option<f32>" };
const Duration = { typeName: "core::time::Duration" };
const Mat3 = { typeName: "glam::f32::mat3::Mat3" };
const Mat2 = { typeName: "glam::f32::sse2::mat2::Mat2" };
const Mat4 = { typeName: "glam::f32::sse2::mat4::Mat4" };
const Quat = { typeName: "glam::f32::sse2::quat::Quat" };
const Vec4 = { typeName: "glam::f32::sse2::vec4::Vec4" };
const Vec2 = { typeName: "glam::f32::vec2::Vec2" };
const Vec3 = { typeName: "glam::f32::vec3::Vec3" };
const IVec2 = { typeName: "glam::i32::ivec2::IVec2" };
const IVec3 = { typeName: "glam::i32::ivec3::IVec3" };
const IVec4 = { typeName: "glam::i32::ivec4::IVec4" };
const UVec2 = { typeName: "glam::u32::uvec2::UVec2" };
const UVec3 = { typeName: "glam::u32::uvec3::UVec3" };
const UVec4 = { typeName: "glam::u32::uvec4::UVec4" };
const HashSetString = { typeName: "hashbrown::set::HashSet<alloc::string::String>" };
const Instant = { typeName: "std::time::Instant" };