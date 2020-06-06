// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Vulkan instance validation layers

use std::ffi::CString;

pub(crate) fn to_i8(layers: &[CString]) -> Vec<*const i8> {
    layers.iter().map(|v| v.as_ptr()).collect()
}

pub(crate) fn list() -> Vec<CString> {
    let lay = &[
        #[cfg(debug_assertions)]
        "VK_LAYER_KHRONOS_validation",
    ];

    lay.iter()
        .map(|s| CString::new(*s).expect("bad string"))
        .collect()
}
