use ash::extensions::ext::DebugUtils as Extension;
use ash::vk::DebugUtilsMessageSeverityFlagsEXT as Severity;
use ash::vk::DebugUtilsMessageTypeFlagsEXT as MessageType;
use ash::vk::DebugUtilsMessengerCallbackDataEXT as CallbackData;
use ash::vk::DebugUtilsMessengerCreateInfoEXT;
use ash::vk::DebugUtilsMessengerEXT as Messenger;
use ash::vk::FALSE;
use log::debug;
use log::error;
use log::warn;
use std::ffi::c_void;
use std::ffi::CStr;

use super::Vulkan;
use crate::error::Result;

pub(crate) struct Validator {
    messenger: Messenger,
    ext: Extension,
}

impl Validator {
    #[allow(dead_code)]
    pub(crate) fn new(vulkan: &Vulkan) -> Result<Self> {
        debug!("creating validator");

        let ext = Extension::new(vulkan.entry_ref(), vulkan.instance_ref());

        let info = DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(Severity::ERROR | Severity::WARNING)
            .message_type(MessageType::all())
            .pfn_user_callback(Some(callback));

        let messenger = unsafe { ext.create_debug_utils_messenger(&info, None)? };

        Ok(Self { ext, messenger })
    }
}

impl Drop for Validator {
    fn drop(&mut self) {
        unsafe {
            self.ext.destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}

#[allow(dead_code)]
extern "system" fn callback(
    severity: Severity,
    _: MessageType,
    debug_data: *const CallbackData,
    _: *mut c_void,
) -> u32 {
    let msg = unsafe {
        let message = debug_data.as_ref().unwrap().p_message;
        CStr::from_ptr(message).to_str().unwrap()
    };

    if severity.contains(Severity::ERROR) {
        error!("{}", msg);
        std::process::exit(1);
    } else {
        warn!("{}", msg);
    }

    FALSE
}
