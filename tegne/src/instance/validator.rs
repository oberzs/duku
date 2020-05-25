// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// validates Vulkan API calls

#![cfg(debug_assertions)]

use ash::vk;
use log::error;
use log::warn;
use std::ffi::c_void;
use std::ffi::CStr;

pub(crate) fn messenger_config() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        )
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(callback))
        .build()
}

extern "system" fn callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    debug_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> u32 {
    let msg = unsafe {
        let message = debug_data.as_ref().unwrap().p_message;
        CStr::from_ptr(message).to_str().unwrap()
    };

    if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
        panic!(error!("{}", msg));
    } else {
        warn!("{}", msg);
    }

    vk::FALSE
}
