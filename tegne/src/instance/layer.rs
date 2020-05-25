// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Vulkan instance validation layers

use std::ffi::CString;

use crate::error::Result;

pub(crate) fn to_i8(layers: &[CString]) -> Vec<*const i8> {
    layers.iter().map(|v| v.as_ptr()).collect()
}

pub(crate) fn list() -> Result<Vec<CString>> {
    Ok(vec![
        #[cfg(debug_assertions)]
        CString::new("VK_LAYER_KHRONOS_validation")?,
    ])
}
