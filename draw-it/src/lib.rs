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
    // clippy::missing_const_for_fn,
    // clippy::redundant_pub_crate,
    // clippy::unwrap_used
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
mod storage;
mod surface;
mod sync;
mod watch;

// normal exports
pub use color::Color;
pub use context::Context;
pub use context::ContextOptions;
pub use device::Stats;
pub use error::Result;
pub use image::Framebuffer;
pub use image::Texture;
pub use image::TextureFilter;
pub use image::TextureWrap;
pub use math::Quaternion;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::Mesh;
pub use pipeline::Material;
pub use pipeline::Shader;
pub use quality::Quality;
pub use quality::QualityOptions;
pub use renderer::Camera;
pub use renderer::CameraType;
pub use renderer::Light;
pub use renderer::LightType;
pub use renderer::Pcf;
pub use renderer::Target;
pub use surface::VSync;
pub use surface::WindowHandle;

// optional feature exports
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "window")]
pub mod window;
