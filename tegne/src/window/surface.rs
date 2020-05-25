// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Surface - OS window abstraction

use ash::vk;
use log::debug;
use std::os::raw::c_void;
use std::sync::Arc;

use super::WindowHandle;
use crate::error::Result;
use crate::instance::Instance;

pub(crate) struct Surface {
    handle: vk::SurfaceKHR,
    width: u32,
    height: u32,
    instance: Arc<Instance>,
}

impl Surface {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(instance: &Arc<Instance>, window: WindowHandle) -> Result<Self> {
        debug!("creating Windows window surface");

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
        debug!("creating Linux window surface");

        let info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(window.xlib_window)
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
        debug!("creating MacOS window surface");

        use cocoa::appkit::NSView;
        use cocoa::appkit::NSWindow;
        use cocoa::base::id as cocoa_id;
        use metal::CoreAnimationLayer;
        use std::mem;
        use std::ptr;

        let wnd: cocoa_id = unsafe { mem::transmute(window.ns_window) };
        let layer = CoreAnimationLayer::new();

        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        let view = unsafe { wnd.contentView() };

        layer.set_contents_scale(unsafe { view.backingScaleFactor() });
        unsafe { view.setLayer(mem::transmute(layer.as_ref())) };
        unsafe { view.setWantsLayer(1) };

        let info = vk::MacOSSurfaceCreateInfoMVK::builder().p_view(window.ns_view as *const c_void);

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
