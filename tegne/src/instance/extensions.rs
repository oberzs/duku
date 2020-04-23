use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::PhysicalDevice;
use ash::Entry;
use std::ffi::CStr;
use std::ffi::CString;

use super::Vulkan;
use crate::utils::cstring;
use crate::utils::OrError;

#[derive(Default)]
pub(crate) struct Extensions {
    instance: Vec<CString>,
    device: Vec<CString>,
    layers: Vec<CString>,
}

impl Extensions {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn supports_instance(&self, entry: &Entry) -> bool {
        let available = entry
            .enumerate_instance_extension_properties()
            .or_error("cannot enumerate instance extensions")
            .iter()
            .map(|e| {
                let ptr = e.extension_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();

        self.instance.iter().all(|e| available.contains(e))
    }

    pub(crate) fn supports_device(&self, vulkan: &Vulkan, device: PhysicalDevice) -> bool {
        let available = unsafe {
            vulkan
                .instance_ref()
                .enumerate_device_extension_properties(device)
                .or_error("cannot enumerate device extensions")
                .iter()
                .map(|e| {
                    let ptr = e.extension_name.as_ptr();
                    CStr::from_ptr(ptr).to_owned()
                })
                .collect::<Vec<_>>()
        };

        self.device.iter().all(|e| available.contains(e))
    }

    pub(crate) fn supports_layers(&self, entry: &Entry) -> bool {
        let available = entry
            .enumerate_instance_layer_properties()
            .or_error("cannot enumerate layers")
            .iter()
            .map(|l| {
                let ptr = l.layer_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();

        self.layers.iter().all(|l| available.contains(l))
    }

    pub(crate) fn instance(&self) -> Vec<*const i8> {
        self.instance.iter().map(|e| e.as_ptr()).collect()
    }

    pub(crate) fn device(&self) -> Vec<*const i8> {
        self.device.iter().map(|e| e.as_ptr()).collect()
    }

    pub(crate) fn layers(&self) -> Vec<*const i8> {
        self.layers.iter().map(|l| l.as_ptr()).collect()
    }
}