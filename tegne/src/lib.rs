#![feature(proc_macro_hygiene)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
)]

mod buffer;
mod error;
mod images;
mod instance;
mod memory;
mod mesh;
mod objects;
mod renderer;
mod shaders;
mod sync;
mod utils;

pub use images::Texture;
pub use instance::Target;
pub use instance::Tegne;
pub use instance::TegneOptions;
pub use mesh::Mesh;
pub use mesh::MeshOptions;
pub use objects::Id;
pub use shaders::Material;
pub use shaders::MaterialOptions;
pub use shaders::Shader;

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
