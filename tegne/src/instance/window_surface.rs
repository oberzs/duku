use ash::extensions::khr::Surface as Extension;
use ash::vk::PhysicalDevice;
use ash::vk::PresentModeKHR;
use ash::vk::SurfaceCapabilitiesKHR;
use ash::vk::SurfaceFormatKHR;
use ash::vk::SurfaceKHR;
use log::debug;
use std::os::raw::c_void;

use super::Vulkan;
use crate::utils::OrError;

#[cfg(target_os = "windows")]
#[derive(Copy, Clone)]
pub struct WindowArgs {
    pub hwnd: *const c_void,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "linux")]
#[derive(Copy, Clone)]
pub struct WindowArgs {
    pub xlib_window: std::os::raw::c_ulong,
    pub xlib_display: *mut c_void,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "macos")]
#[derive(Copy, Clone)]
pub struct WindowArgs {
    pub ns_window: *mut c_void,
    pub ns_view: *mut c_void,
    pub width: u32,
    pub height: u32,
}

pub(crate) struct WindowSurface {
    vk: SurfaceKHR,
    ext: Extension,
}

impl WindowSurface {
    #[cfg(target_os = "windows")]
    pub(crate) fn new(vulkan: &Vulkan, args: WindowArgs) -> Self {
        debug!("creating Windows window surface");

        use ash::extensions::khr::Win32Surface;
        use ash::vk::StructureType;
        use ash::vk::Win32SurfaceCreateInfoKHR;
        use std::ptr;
        use winapi::um::libloaderapi::GetModuleHandleW;

        let hinstance = unsafe { GetModuleHandleW(ptr::null()) } as *const c_void;
        let info = Win32SurfaceCreateInfoKHR {
            s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            hwnd: args.hwnd,
            hinstance,
        };

        let ext = Extension::new(vulkan.entry_ref(), vulkan.instance_ref());
        let loader = Win32Surface::new(vulkan.entry_ref(), vulkan.instance_ref());
        let vk = unsafe {
            loader
                .create_win32_surface(&info, None)
                .or_error("cannot create window surface")
        };

        Self { vk, ext }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn new(vulkan: &Vulkan, args: WindowArgs) -> Self {
        debug!("creating Linux window surface");

        use ash::extensions::khr::XlibSurface;
        use ash::vk::Display;
        use ash::vk::XlibSurfaceCreateInfoKHR;

        let info = XlibSurfaceCreateInfoKHR::builder()
            .window(args.xlib_window)
            .dpy(args.xlib_display as *mut Display);

        let ext = Extension::new(vulkan.entry_ref(), vulkan.instance_ref());
        let loader = XlibSurface::new(vulkan.entry_ref(), vulkan.instance_ref());
        let vk = unsafe {
            loader
                .create_xlib_surface(&info, None)
                .or_error("cannot create window surface")
        };

        Self { vk, ext }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(vulkan: &Vulkan, args: WindowArgs) -> Self {
        debug!("creating MacOS window surface");

        use ash::extensions::mvk::MacOSSurface;
        use ash::vk::MacOSSurfaceCreateInfoMVK;
        use ash::vk::StructureType;
        use cocoa::appkit::{NSView, NSWindow};
        use cocoa::base::id as cocoa_id;
        use metal::CoreAnimationLayer;
        use std::mem;
        use std::ptr;

        let wnd: cocoa_id = unsafe { mem::transmute(args.ns_window) };
        let layer = CoreAnimationLayer::new();

        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        let view = unsafe { wnd.contentView() };

        layer.set_contents_scale(unsafe { view.backingScaleFactor() });
        unsafe { view.setLayer(mem::transmute(layer.as_ref())) };
        unsafe { view.setWantsLayer(1) };

        let info = MacOSSurfaceCreateInfoMVK {
            s_type: StructureType::MACOS_SURFACE_CREATE_INFO_M,
            p_next: ptr::null(),
            flags: Default::default(),
            p_view: args.ns_view as *const c_void,
        };

        let ext = Extension::new(vulkan.entry_ref(), vulkan.instance_ref());
        let loader = MacOSSurface::new(vulkan.entry_ref(), vulkan.instance_ref());
        let vk = unsafe {
            loader
                .create_mac_os_surface_mvk(&info, None)
                .or_error("cannot create window surface")
        };

        Self { vk, ext }
    }

    pub(crate) fn gpu_formats(&self, device: PhysicalDevice) -> Vec<SurfaceFormatKHR> {
        unsafe {
            self.ext
                .get_physical_device_surface_formats(device, self.vk)
                .or_error("cannot get surface formats")
        }
    }

    pub(crate) fn gpu_capabilities(&self, device: PhysicalDevice) -> SurfaceCapabilitiesKHR {
        unsafe {
            self.ext
                .get_physical_device_surface_capabilities(device, self.vk)
                .or_error("cannot get surface capabilities")
        }
    }

    pub(crate) fn gpu_present_modes(&self, device: PhysicalDevice) -> Vec<PresentModeKHR> {
        unsafe {
            self.ext
                .get_physical_device_surface_present_modes(device, self.vk)
                .or_error("cannot get surface present modes")
        }
    }

    pub(crate) fn supports_device(&self, device: PhysicalDevice, index: u32) -> bool {
        unsafe {
            self.ext
                .get_physical_device_surface_support(device, index, self.vk)
                .or_error("cannot get surface support")
        }
    }

    pub(crate) fn vk(&self) -> SurfaceKHR {
        self.vk
    }
}

impl Drop for WindowSurface {
    fn drop(&mut self) {
        unsafe {
            self.ext.destroy_surface(self.vk, None);
        }
    }
}
