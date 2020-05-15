use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::PhysicalDevice;
use ash::Entry;
use std::ffi::CStr;
use std::ffi::CString;

use super::Vulkan;
use crate::error::ErrorKind;
use crate::error::Result;

#[derive(Default)]
pub(crate) struct Extensions {
    instance: Vec<CString>,
    device: Vec<CString>,
    layers: Vec<CString>,
}

impl Extensions {
    pub(crate) fn new() -> Result<Self> {
        let mut instance = vec![CString::new("VK_KHR_surface")?];
        #[cfg(target_os = "windows")]
        instance.push(CString::new("VK_KHR_win32_surface")?);
        #[cfg(target_os = "linux")]
        instance.push(CString::new("VK_KHR_xlib_surface")?);
        #[cfg(target_os = "macos")]
        instance.push(CString::new("VK_EXT_metal_surface")?);
        #[cfg(debug_assertions)]
        instance.push(CString::new("VK_EXT_debug_utils")?);

        let device = vec![CString::new("VK_KHR_swapchain")?];

        #[cfg(debug_assertions)]
        let layers = vec![CString::new("VK_LAYER_KHRONOS_validation")?];
        #[cfg(not(debug_assertions))]
        let layers = vec![];

        Ok(Self {
            instance,
            device,
            layers,
        })
    }

    pub(crate) fn supports_instance(&self, entry: &Entry) -> Result<()> {
        let available = entry
            .enumerate_instance_extension_properties()?
            .iter()
            .map(|e| {
                let ptr = e.extension_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();

        if self.instance.iter().all(|e| available.contains(e)) {
            Ok(())
        } else {
            Err(ErrorKind::UnsupportedExtension.into())
        }
    }

    pub(crate) fn supports_device(&self, vulkan: &Vulkan, device: PhysicalDevice) -> Result<()> {
        let available = unsafe {
            vulkan
                .instance_ref()
                .enumerate_device_extension_properties(device)?
                .iter()
                .map(|e| {
                    let ptr = e.extension_name.as_ptr();
                    CStr::from_ptr(ptr).to_owned()
                })
                .collect::<Vec<_>>()
        };

        if self.device.iter().all(|e| available.contains(e)) {
            Ok(())
        } else {
            Err(ErrorKind::UnsupportedExtension.into())
        }
    }

    pub(crate) fn supports_layers(&self, entry: &Entry) -> Result<()> {
        let available = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| {
                let ptr = l.layer_name.as_ptr();
                unsafe { CStr::from_ptr(ptr).to_owned() }
            })
            .collect::<Vec<_>>();

        if self.layers.iter().all(|l| available.contains(l)) {
            Ok(())
        } else {
            Err(ErrorKind::UnsupportedExtension.into())
        }
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
