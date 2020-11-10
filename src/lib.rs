// Oliver Berzs
// https://github.com/oberzs/duku

// Duku is a Vulkan rendering engine

#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    clippy::missing_const_for_fn,
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::if_not_else,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::unused_self,
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
mod renderer;
mod storage;
mod surface;
mod vk;

#[cfg(feature = "glsl")]
mod watch;

// normal exports
pub use color::Color;
pub use context::Context;
pub use device::Stats;
pub use error::Result;
pub use image::ColorSpace;
pub use image::Cubemap;
pub use image::CubemapSides;
pub use image::Filter;
pub use image::Framebuffer;
pub use image::Mips;
pub use image::Msaa;
pub use image::Texture;
pub use image::Wrap;
pub use math::Matrix3;
pub use math::Matrix4;
pub use math::Quaternion;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::Mesh;
pub use pipeline::Material;
pub use pipeline::Shader;
pub use renderer::BorderMode;
pub use renderer::Camera;
pub use renderer::Light;
pub use renderer::LightType;
pub use renderer::Pcf;
pub use renderer::Projection;
pub use renderer::ShapeMode;
pub use renderer::Target;
pub use storage::Handle;
pub use surface::VSync;
pub use surface::WindowHandle;

// optional feature exports
#[cfg(feature = "window")]
pub mod window;
