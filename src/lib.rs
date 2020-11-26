// Oliver Berzs
// https://github.com/oberzs/duku

// Duku is a Vulkan rendering engine

//! This Rust crate makes it easy to render 2D and 3D graphics.
//!
//! # Example
//!
//! ```ignore
//! use duku::Color;
//! use duku::Camera;
//! use duku::Duku;
//! use duku::Result;
//!
//! fn main() -> Result<()> {
//!     // initialize duku and OS window with a size of 500x500
//!     let (mut duku, window) = Duku::builder().build_window(500, 500).build()?;
//!
//!     // create a 3D perspective camera with an FOV of 90
//!     let mut camera = Camera::perspective_autosized(90);
//!
//!     // move the camera to some location
//!     // and make it look at the center of the world
//!     camera.transform.move_by([2.0, 1.5, -2.0]);
//!     camera.transform.look_at([0.0, 0.0, 0.0]);
//!
//!     // start up the main event loop
//!     window.main_loop(move |_| {
//!       // start drawing on the window using our camera
//!       duku.draw_on_window(Some(&camera), |target| {
//!             // set the background color to sky blue
//!             target.clear = Color::SKY_BLUE;
//!
//!             // draw a cube at the center of the world
//!             target.draw_cube();
//!         });
//!     });
//!
//!     Ok(())
//! }
//! ```
//!
//! More usage examples can be found [here](https://github.com/oberzs/duku/tree/release/examples).

#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    missing_docs,
    missing_doc_code_examples,
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
    clippy::unused_self
)]

// should be imported first
mod macros;

mod buffer;
mod device;
mod duku;
mod error;
mod features;
mod font;
mod image;
mod instance;
mod math;
mod mesh;
mod pipeline;
mod renderer;
mod resources;
mod surface;
mod vk;

// normal exports
pub use self::duku::Duku;
pub use self::duku::DukuBuilder;
pub use device::Stats;
pub use error::Error;
pub use error::Result;
pub use image::Canvas;
pub use image::ColorSpace;
pub use image::Cubemap;
pub use image::CubemapSides;
pub use image::Filter;
pub use image::Format;
pub use image::Mips;
pub use image::Msaa;
pub use image::Texture;
pub use image::Wrap;
pub use math::Matrix4;
pub use math::Quaternion;
pub use math::Transform;
pub use math::Vector2;
pub use math::Vector3;
pub use math::Vector4;
pub use mesh::Mesh;
pub use mesh::Model;
pub use mesh::ModelNode;
pub use pipeline::Material;
pub use pipeline::Shader;
pub use renderer::BorderMode;
pub use renderer::Camera;
pub use renderer::Color;
pub use renderer::Light;
pub use renderer::LightType;
pub use renderer::Pcf;
pub use renderer::Projection;
pub use renderer::ShapeMode;
pub use renderer::Target;
pub use resources::Handle;
pub use surface::VSync;
pub use surface::WindowHandle;

// optional feature exports
#[cfg(feature = "glsl")]
pub use features::glsl;
#[cfg(feature = "window")]
pub use features::window;
