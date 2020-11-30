// Oliver Berzs
// https://github.com/oberzs/duku

// Duku is a Vulkan rendering engine

//! This Rust crate makes it easy to render 2D and 3D graphics.
//!
//! # Example
//!
//! ```no_run
//! // This example draws a cube in the center of the window,
//! // rotating and coloring it based on the time that has passed.
//!
//! use duku::Camera;
//! use duku::Duku;
//! use duku::Hsb;
//! use duku::Light;
//! use duku::Result;
//! use std::time::Instant;
//!
//! fn main() -> Result<()> {
//!     // create duku context and window
//!     let (mut duku, window) = Duku::windowed(500, 500)?;
//!
//!     // create 3D camera with 90 fov
//!     let camera = Camera::perspective(90);
//!
//!     // create directional light
//!     let light = Light::directional("#ffffff", [-1.0, -1.0, 1.0]);
//!
//!     // start timer for rotation and color
//!     let timer = Instant::now();
//!
//!     // start window loop
//!     window.while_open(move |_| {
//!         // start drawing on window
//!         duku.draw(Some(&camera), |t| {
//!             // setup scene
//!             t.background("#ababab");
//!             t.light(light);
//!
//!             // get elapsed time since start
//!             let elapsed = timer.elapsed().as_secs_f32();
//!
//!             // transform scene
//!             let angle = elapsed * 45.0;
//!             t.rotate_x(angle);
//!             t.rotate_y(angle);
//!             t.translate_z(2.0);
//!
//!             // draw cube
//!             let hue = (elapsed * 60.0) as u16;
//!             t.tint(Hsb::new(hue, 70, 80));
//!             t.cube([1.0, 1.0, 1.0]);
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
mod color;
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
pub use color::Gradient;
pub use color::Hsb;
pub use color::Mix;
pub use color::Rgb;
pub use color::Rgbf;
pub use device::Stats;
pub use error::Error;
pub use error::Result;
pub use font::Font;
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
pub use math::Mat4;
pub use math::Quat;
pub use math::Vec2;
pub use math::Vec3;
pub use math::Vec4;
pub use mesh::Mesh;
pub use mesh::Model;
pub use mesh::ModelNode;
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
pub use resources::Handle;
pub use surface::VSync;
pub use surface::WindowHandle;

// optional feature exports
#[cfg(feature = "glsl")]
pub use features::glsl;
#[cfg(feature = "window")]
pub use features::window;
