// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Vulkan instance extensions

use std::ffi::CString;

pub(crate) fn to_i8(exts: &[CString]) -> Vec<*const i8> {
    exts.iter().map(|e| e.as_ptr()).collect()
}

pub(crate) fn list() -> Vec<CString> {
    let exts = &[
        "VK_KHR_surface",
        #[cfg(target_os = "windows")]
        "VK_KHR_win32_surface",
        #[cfg(target_os = "linux")]
        "VK_KHR_xlib_surface",
        #[cfg(target_os = "macos")]
        "VK_EXT_metal_surface",
        #[cfg(debug_assertions)]
        "VK_EXT_debug_utils",
    ];

    exts.iter()
        .map(|e| CString::new(*e).expect("bad string"))
        .collect()
}
