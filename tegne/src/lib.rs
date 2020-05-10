#![feature(proc_macro_hygiene)]

mod buffer;
mod builtins;
mod font;
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
