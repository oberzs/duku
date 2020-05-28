// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Tegne is a Vulkan rendering engine

#![feature(proc_macro_hygiene, drain_filter, trait_alias)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
)]

mod buffer;
mod camera;
mod device;
mod error;
mod font;
mod image;
mod instance;
mod math;
mod mesh;
mod pipeline;
mod renderer;
mod resource;
mod surface;
mod sync;
mod tegne;

pub use self::tegne::Tegne;
pub use self::tegne::TegneOptions;
pub use camera::Camera;
pub use image::Framebuffer;
pub use image::Texture;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::Mesh;
pub use mesh::MeshOptions;
pub use pipeline::Material;
pub use pipeline::MaterialOptions;
pub use pipeline::Shader;
pub use pipeline::ShaderOptions;
pub use renderer::Target;
pub use resource::Id;

#[cfg(feature = "controller")]
pub use camera::Controller;
#[cfg(feature = "ui")]
pub use imgui as ui;
#[cfg(feature = "window")]
pub use surface::Events;
#[cfg(feature = "window")]
pub use surface::Key;
#[cfg(feature = "window")]
pub use surface::Window;
#[cfg(feature = "window")]
pub use surface::WindowOptions;
