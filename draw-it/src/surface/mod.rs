// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// a drawable surface that connects to an OS window

mod handle;
mod properties;
mod swapchain;

use ash::vk;
use std::sync::Arc;

use crate::error::Result;
use crate::instance::Instance;

pub(crate) use properties::ColorSpace;
pub(crate) use properties::PresentMode;
pub(crate) use properties::SurfaceProperties;
pub(crate) use swapchain::Swapchain;

pub use handle::WindowHandle;

pub(crate) struct Surface {
    handle: vk::SurfaceKHR,
    width: u32,
    height: u32,
    instance: Arc<Instance>,
}

impl Surface {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(instance: &Arc<Instance>, window: WindowHandle) -> Result<Self> {
        use std::os::raw::c_void;
        use std::ptr;
        use winapi::um::libloaderapi::GetModuleHandleW;

        let hinstance = unsafe { GetModuleHandleW(ptr::null()) } as *const c_void;
        let info = vk::Win32SurfaceCreateInfoKHR::builder()
            .hwnd(window.hwnd)
            .hinstance(hinstance);

        let handle = instance.create_surface(&info)?;

        Ok(Self {
            handle,
            width: window.width,
            height: window.height,
            instance: instance.clone(),
        })
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn new(instance: &Arc<Instance>, window: WindowHandle) -> Result<Self> {
        let info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(window.xlib_window as u64)
            .dpy(window.xlib_display as *mut vk::Display);

        let handle = instance.create_surface(&info)?;

        Ok(Self {
            handle,
            width: window.width,
            height: window.height,
            instance: instance.clone(),
        })
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(instance: &Arc<Instance>, window: WindowHandle) -> Result<Self> {
        use cocoa::appkit::NSView;
        use cocoa::appkit::NSWindow;
        use cocoa::base::id as cocoa_id;
        use metal::CoreAnimationLayer;
        use std::mem;
        use std::os::raw::c_void;

        let wnd: cocoa_id = unsafe { mem::transmute(window.ns_window) };
        let layer = CoreAnimationLayer::new();

        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        let view = unsafe { wnd.contentView() };

        layer.set_contents_scale(unsafe { view.backingScaleFactor() });
        unsafe { view.setLayer(mem::transmute(layer.as_ref())) };
        unsafe { view.setWantsLayer(1) };

        let info = vk::MacOSSurfaceCreateInfoMVK::builder().view(view as &c_void);

        let handle = instance.create_surface(&info)?;

        Ok(Self {
            handle,
            width: window.width,
            height: window.height,
            instance: instance.clone(),
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.instance.destroy_surface(self.handle);
    }
}
