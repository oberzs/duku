// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Vulkan instance extensions

use std::ffi::CStr;
use std::ffi::CString;

use crate::error::ErrorKind;
use crate::error::Result;

pub(crate) struct Extensions(&'static [&'static CStr]);

pub(crate) const INSTANCE_EXTENSIONS: Extensions = Extensions(cslice![
    "VK_KHR_surface",
    #[cfg(target_os = "windows")]
    "VK_KHR_win32_surface",
    #[cfg(target_os = "linux")]
    "VK_KHR_xlib_surface",
    #[cfg(target_os = "macos")]
    "VK_EXT_metal_surface",
    #[cfg(debug_assertions)]
    "VK_EXT_debug_utils"
]);

pub(crate) const DEVICE_EXTENSIONS: Extensions = Extensions(cslice!["VK_KHR_swapchain"]);

pub(crate) const VALIDATION_LAYERS: Extensions = Extensions(cslice![
    #[cfg(debug_assertions)]
    "VK_LAYER_KHRONOS_validation"
]);

impl Extensions {
    pub(crate) fn assert_missing(&self, available: &[CString]) -> Result<()> {
        for ext in self.0 {
            if !available.iter().any(|a| a.as_c_str() == *ext) {
                return Err(ErrorKind::UnsupportedExtension(format!("{:?}", ext)).into());
            }
        }
        Ok(())
    }

    pub(crate) fn as_ptr(&self) -> Vec<*const i8> {
        self.0.iter().map(|e| e.as_ptr()).collect()
    }
}
