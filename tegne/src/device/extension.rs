// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Vulkan device extensions

use std::ffi::CString;

use crate::error::Result;

pub(crate) fn to_i8(exts: &[CString]) -> Vec<*const i8> {
    exts.iter().map(|e| e.as_ptr()).collect()
}

pub(crate) fn list() -> Result<Vec<CString>> {
    Ok(vec![CString::new("VK_KHR_swapchain")?])
}
