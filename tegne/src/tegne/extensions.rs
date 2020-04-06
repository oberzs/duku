use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::PhysicalDevice;
use ash::Entry;
use std::ffi::CStr;
use std::ffi::CString;

use super::Instance;
use crate::utils::cstring;
use crate::utils::unwrap_error;

#[derive(Default)]
pub struct Extensions {
    instance: Vec<CString>,
    device: Vec<CString>,
    layers: Vec<CString>,
}

impl Extensions {
    pub fn new() -> Self {
        let mut instance = vec![cstring("VK_KHR_surface")];
        #[cfg(target_os = "windows")]
        instance.push(cstring("VK_KHR_win32_surface"));
        #[cfg(target_os = "linux")]
        instance.push(cstring("VK_KHR_xlib_surface"));
        #[cfg(target_os = "macos")]
        instance.push(cstring("VK_EXT_metal_surface"));
        #[cfg(debug_assertions)]
        instance.push(cstring("VK_EXT_debug_utils"));

        let device = vec![cstring("VK_KHR_swapchain")];

        #[cfg(debug_assertions)]
        let layers = vec![cstring("VK_LAYER_KHRONOS_validation")];
        #[cfg(not(debug_assertions))]
        let layers = vec![];

        Self {
            instance,
            device,
            layers,
        }
    }

    pub fn supports_instance(&self, entry: &Entry) -> bool {
        let available = unwrap_error(
            entry.enumerate_instance_extension_properties(),
            "cannot enumerate instance extensions",
        )
        .iter()
        .map(|e| {
            let ptr = e.extension_name.as_ptr();
            unsafe { CStr::from_ptr(ptr).to_owned() }
        })
        .collect::<Vec<_>>();

        self.instance.iter().all(|e| available.contains(e))
    }

    pub fn supports_device(&self, instance: &Instance, device: PhysicalDevice) -> bool {
        let available = unsafe {
            unwrap_error(
                instance
                    .vk_ref()
                    .enumerate_device_extension_properties(device),
                "cannot enumerate device extensions",
            )
            .iter()
            .map(|e| {
                let ptr = e.extension_name.as_ptr();
                CStr::from_ptr(ptr).to_owned()
            })
            .collect::<Vec<_>>()
        };

        self.device.iter().all(|e| available.contains(e))
    }

    pub fn supports_layers(&self, entry: &Entry) -> bool {
        let available = unwrap_error(
            entry.enumerate_instance_layer_properties(),
            "cannot enumerate layers",
        )
        .iter()
        .map(|l| {
            let ptr = l.layer_name.as_ptr();
            unsafe { CStr::from_ptr(ptr).to_owned() }
        })
        .collect::<Vec<_>>();

        self.layers.iter().all(|l| available.contains(l))
    }

    pub fn instance(&self) -> Vec<*const i8> {
        self.instance.iter().map(|e| e.as_ptr()).collect()
    }

    pub fn device(&self) -> Vec<*const i8> {
        self.device.iter().map(|e| e.as_ptr()).collect()
    }

    pub fn layers(&self) -> Vec<*const i8> {
        self.layers.iter().map(|l| l.as_ptr()).collect()
    }
}
