// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// a drawable surface that connects to an OS window

mod handle;
mod properties;
mod swapchain;

use std::ptr;
use std::rc::Rc;

use crate::instance::Instance;
use crate::vk;

pub(crate) use swapchain::Swapchain;

pub use handle::WindowHandle;
pub use properties::VSync;

pub(crate) struct Surface {
    handle: vk::SurfaceKHR,
    width: u32,
    height: u32,
    instance: Rc<Instance>,
}

impl Surface {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(instance: &Rc<Instance>, window: WindowHandle) -> Self {
        use std::os::raw::c_void;
        use winapi::um::libloaderapi::GetModuleHandleW;

        let hinstance = unsafe { GetModuleHandleW(ptr::null()) } as *const c_void;
        let info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: 0,
            hwnd: window.hwnd,
            hinstance,
        };

        let handle = instance.create_win32_surface(&info);

        Self {
            handle,
            width: window.width,
            height: window.height,
            instance: Rc::clone(instance),
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn new(instance: &Rc<Instance>, window: WindowHandle) -> Self {
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
            instance: Rc::clone(instance),
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(instance: &Rc<Instance>, window: WindowHandle) -> Self {
        use cocoa::appkit::NSView;
        use cocoa::appkit::NSWindow;
        use cocoa::base::id as cocoa_id;
        use metal::CoreAnimationLayer;
        use std::mem;

        let wnd: cocoa_id = unsafe { mem::transmute(window.ns_window) };
        let layer = CoreAnimationLayer::new();

        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        let view = unsafe { wnd.contentView() };

        layer.set_contents_scale(unsafe { view.backingScaleFactor() });
        unsafe { view.setLayer(mem::transmute(layer.as_ref())) };
        unsafe { view.setWantsLayer(1) };

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
            instance: Rc::clone(instance),
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

impl Drop for Surface {
    fn drop(&mut self) {
        self.instance.destroy_surface(self.handle);
    }
}
