// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Tegne is a Vulkan rendering engine

#![feature(proc_macro_hygiene, drain_filter, thread_id_value)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
)]

// should be imported first
mod macros;

mod buffer;
mod camera;
mod color;
mod device;
mod error;
mod font;
mod image;
mod instance;
mod math;
mod mesh;
mod pipeline;
mod profiler;
mod renderer;
mod resource;
mod surface;
mod sync;
mod tegne;

pub use self::tegne::Tegne;
pub use self::tegne::TegneOptions;
pub use camera::Camera;
pub use camera::CameraType;
pub use color::colors;
pub use color::Color;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::MeshOptions;
pub use pipeline::MaterialOptions;
pub use pipeline::SamplerAddress;
pub use pipeline::SamplerFilter;
pub use pipeline::SamplerMipmaps;
pub use pipeline::ShaderOptions;
pub use profiler::begin_profile;
pub use profiler::end_profile;
pub use renderer::SamplerOptions;
pub use renderer::Target;

pub type Mesh = resource::Ref<mesh::Mesh>;
pub type Material = resource::Ref<pipeline::Material>;
pub type Texture = resource::Ref<image::Texture>;
pub type Shader = resource::Ref<pipeline::Shader>;
pub type Framebuffer = resource::Ref<image::Framebuffer>;

#[cfg(feature = "controller")]
pub use camera::Controller;
#[cfg(feature = "ui")]
pub use surface::ui;
#[cfg(feature = "window")]
pub use surface::Events;
#[cfg(feature = "window")]
pub use surface::Key;
#[cfg(feature = "window")]
pub use surface::Window;
#[cfg(feature = "window")]
pub use surface::WindowOptions;
