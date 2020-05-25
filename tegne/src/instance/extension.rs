// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Vulkan instance extensions

use std::ffi::CString;

use crate::error::Result;

pub(crate) fn to_i8(exts: &[CString]) -> Vec<*const i8> {
    exts.iter().map(|e| e.as_ptr()).collect()
}

pub(crate) fn list() -> Result<Vec<CString>> {
    Ok(vec![
        CString::new("VK_KHR_surface")?,
        #[cfg(target_os = "windows")]
        CString::new("VK_KHR_win32_surface")?,
        #[cfg(target_os = "linux")]
        CString::new("VK_KHR_xlib_surface")?,
        #[cfg(target_os = "macos")]
        CString::new("VK_EXT_metal_surface")?,
        #[cfg(debug_assertions)]
        CString::new("VK_EXT_debug_utils")?,
    ])
}
