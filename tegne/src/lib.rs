// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Tegne is a Vulkan rendering engine

#![feature(proc_macro_hygiene, drain_filter)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
)]

mod buffer;
mod device;
mod error;
mod font;
mod image;
mod instance;
mod mesh;
mod objects;
mod pipeline;
mod renderer;
mod sync;
mod tegne;
mod thread_kill;
mod window;

pub use image::Framebuffer;
pub use image::Texture;
pub use mesh::Mesh;
pub use mesh::MeshOptions;
pub use objects::Id;
pub use pipeline::Material;
pub use pipeline::MaterialOptions;
pub use pipeline::Shader;
pub use renderer::Target;
pub use tegne::Tegne;
pub use tegne::TegneOptions;

#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Controller;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Events;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Key;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Window;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::WindowOptions;

pub use tegne_math::Camera;
pub use tegne_math::Transform;
pub use tegne_math::Vector2;
pub use tegne_math::Vector3;
pub use tegne_math::Vector4;
