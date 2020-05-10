#![feature(proc_macro_hygiene)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    missing_debug_implementations,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    box_pointers
)]

mod buffer;
mod builtins;
mod images;
mod instance;
mod memory;
mod mesh;
mod renderer;
mod shaders;
mod sync;
mod utils;

pub use images::Texture;
pub use instance::Target;
pub use instance::Tegne;
pub use mesh::Mesh;
pub use shaders::Material;
pub use shaders::Shader;

#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Controller;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Events;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Key;
#[cfg(feature = "tegne-utils")]
pub use tegne_utils::Window;

pub use tegne_math::Camera;
pub use tegne_math::Transform;
pub use tegne_math::Vector2;
pub use tegne_math::Vector3;
