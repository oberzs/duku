// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// validates Vulkan API calls

#![cfg(debug_assertions)]

use ash::vk;
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
        let message = debug_data.as_ref().expect("bad message").p_message;
        CStr::from_ptr(message).to_str().expect("bad message")
    };

    // remove beginning ramble
    let rest = msg.split('|').nth(2).expect("bad message").trim();

    // extract final link if exists
    let (link, rest) = {
        let link_index = rest.find("(https:");
        if let Some(index) = link_index {
            let (r, l) = rest.split_at(index);
            (l.trim_start_matches('(').trim_end_matches(')'), r)
        } else {
            ("", rest)
        }
    };

    // extract detailed explanation if exists
    let (states, rest) = {
        let states_index = rest.find("The Vulkan spec states:");
        if let Some(index) = states_index {
            let (r, s) = rest.split_at(index);
            (s.trim(), r)
        } else {
            ("", rest)
        }
    };

    let mut formatted_msg = rest.to_string();
    if !states.is_empty() {
        formatted_msg = format!("{}\n\n\x1b[93m{}\x1b[0m", formatted_msg, states);
    }
    if !link.is_empty() {
        formatted_msg = format!("{}\n\n{}\n", formatted_msg, link);
    }

    if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
        panic!("{}", formatted_msg);
    } else {
        warn!("{}", formatted_msg);
    }

    vk::FALSE
}
