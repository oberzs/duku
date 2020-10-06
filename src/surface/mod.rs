// Oliver Berzs
// https://github.com/oberzs/draw-it

// a drawable surface that connects to an OS window

mod handle;
mod properties;
mod swapchain;

use std::ptr;

use crate::instance::Instance;
use crate::vk;

pub(crate) use swapchain::Swapchain;

pub use handle::WindowHandle;
pub use properties::VSync;

pub(crate) struct Surface {
    handle: vk::SurfaceKHR,
    width: u32,
    height: u32,
}

impl Surface {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(instance: &Instance, window: WindowHandle) -> Self {
        let info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: 0,
            hwnd: window.hwnd,
            hinstance: ptr::null(),
        };

        let handle = instance.create_win32_surface(&info);

        Self {
            handle,
            width: window.width,
            height: window.height,
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn new(instance: &Instance, window: WindowHandle) -> Self {
        let info = vk::XlibSurfaceCreateInfoKHR {
            s_type: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: 0,
            dpy: window.xlib_display.cast(),
            window: window.xlib_window as u64,
        };

        let handle = instance.create_linux_surface(&info);

        Self {
            handle,
            width: window.width,
            height: window.height,
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(instance: &Instance, window: WindowHandle) -> Self {
        unimplemented!();

        let info = vk::MacOSSurfaceCreateInfoMVK {
            s_type: vk::STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: ptr::null(),
            flags: 0,
            p_view: ptr::null(), // TODO: implement
        };

        let handle = instance.create_macos_surface(&info);

        Self {
            handle,
            width: window.width,
            height: window.height,
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub(crate) const fn width(&self) -> u32 {
        self.width
    }

    pub(crate) const fn height(&self) -> u32 {
        self.height
    }

    pub(crate) const fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }
}
