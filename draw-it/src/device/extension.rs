// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Vulkan device extensions

use std::ffi::CString;

pub(crate) fn to_i8(exts: &[CString]) -> Vec<*const i8> {
    exts.iter().map(|e| e.as_ptr()).collect()
}

pub(crate) fn list() -> Vec<CString> {
    let exts = &["VK_KHR_swapchain"];

    exts.iter()
        .map(|e| CString::new(*e).expect("bad string"))
        .collect()
}
