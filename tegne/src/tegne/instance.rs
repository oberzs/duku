use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::make_version;
use ash::vk::ApplicationInfo;
use ash::vk::InstanceCreateInfo;
use ash::Entry;
use ash::Instance as VkInstance;

use crate::utils::error;
use crate::utils::OrError;

use super::Extensions;

pub struct Instance {
    vk: VkInstance,
    entry: Entry,
}

impl Instance {
    pub fn new(exts: &Extensions) -> Self {
        let entry = Entry::new().or_error("cannot init Vulkan");

        if !exts.supports_instance(&entry) {
            error("requested instance extensions not available");
        }
        if !exts.supports_layers(&entry) {
            error("validation layers requested, but not available");
        }

        let layers = exts.layers();
        let extensions = exts.instance();
        let app_info = ApplicationInfo::builder()
            .application_version(make_version(1, 2, 0))
            .engine_version(make_version(1, 2, 0))
            .api_version(make_version(1, 2, 0));

        let info = InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);

        let vk = unsafe {
            entry
                .create_instance(&info, None)
                .or_error("cannot create instance")
        };

        Self { vk, entry }
    }

    pub fn vk_ref(&self) -> &VkInstance {
        &self.vk
    }

    pub fn entry_ref(&self) -> &Entry {
        &self.entry
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.vk.destroy_instance(None) };
    }
}
