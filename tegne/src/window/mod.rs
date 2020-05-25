// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// window represents a drawable surface
// that connects to a OS window manager

mod handle;
mod properties;
mod surface;
mod swapchain;

pub(crate) use properties::SurfaceProperties;
pub(crate) use surface::Surface;
pub(crate) use swapchain::Swapchain;

pub use handle::WindowHandle;
