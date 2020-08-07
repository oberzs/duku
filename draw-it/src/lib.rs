// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Draw-it is a Vulkan rendering engine

#![feature(drain_filter, const_fn, const_cstr_unchecked)]
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
mod color;
mod context;
mod device;
mod error;
mod font;
mod image;
mod instance;
mod math;
mod mesh;
mod pipeline;
mod quality;
mod renderer;
mod resource;
mod stats;
mod surface;
mod sync;

// normal exports
pub use color::Color;
pub use context::Context;
pub use context::ContextOptions;
pub use error::Result;
pub use math::Quaternion;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::MeshOptions;
pub use pipeline::SamplerAddress;
pub use pipeline::SamplerFilter;
pub use pipeline::SamplerMipmaps;
pub use quality::Quality;
pub use quality::QualityOptions;
pub use renderer::Camera;
pub use renderer::CameraType;
pub use renderer::Light;
pub use renderer::LightType;
pub use renderer::Pcf;
pub use renderer::SamplerOptions;
pub use renderer::Target;
pub use stats::Stats;
pub use surface::VSync;
pub use surface::WindowHandle;

// optional feature exports
#[cfg(feature = "controller")]
pub mod controller;
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "window")]
pub mod window;

// special types
pub type Mesh = resource::Ref<mesh::Mesh>;
pub type Material = resource::Ref<pipeline::Material>;
pub type Texture = resource::Ref<image::Texture>;
pub type Shader = resource::Ref<pipeline::Shader>;
pub type Framebuffer = resource::Ref<image::Framebuffer>;
