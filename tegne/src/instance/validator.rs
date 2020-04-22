use ash::extensions::ext::DebugUtils as Extension;
use ash::vk::DebugUtilsMessageSeverityFlagsEXT as Severity;
use ash::vk::DebugUtilsMessageTypeFlagsEXT as MessageType;
use ash::vk::DebugUtilsMessengerCallbackDataEXT as CallbackData;
use ash::vk::DebugUtilsMessengerCreateInfoEXT;
use ash::vk::DebugUtilsMessengerEXT as Messenger;
use ash::vk::FALSE;
use log::warn;
use std::ffi::c_void;
use std::ffi::CStr;

use super::Vulkan;
use crate::utils::error;
use crate::utils::OrError;

pub(crate) struct Validator {
    messenger: Messenger,
    ext: Extension,
}

impl Validator {
    pub(crate) fn new(vulkan: &Vulkan) -> Self {
        let ext = Extension::new(vulkan.entry_ref(), vulkan.instance_ref());

        let info = DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(Severity::ERROR | Severity::WARNING)
            .message_type(MessageType::all())
            .pfn_user_callback(Some(callback));

        let messenger = unsafe {
            ext.create_debug_utils_messenger(&info, None)
                .or_error("cannot create validator")
        };

        Self { ext, messenger }
    }
}

impl Drop for Validator {
    fn drop(&mut self) {
        unsafe {
            self.ext.destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}

extern "system" fn callback(
    severity: Severity,
    _: MessageType,
    debug_data: *const CallbackData,
    _: *mut c_void,
) -> u32 {
    let msg = unsafe {
        let message = debug_data.as_ref().or_error("no debug data").p_message;
        CStr::from_ptr(message)
            .to_str()
            .or_error("cannot convert cstr to str")
    };

    if severity.contains(Severity::ERROR) {
        error(msg);
    } else {
        warn!("{}", msg);
    }

    FALSE
}
