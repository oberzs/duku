use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::make_version;
use ash::vk::version_major;
use ash::vk::version_minor;
use ash::vk::version_patch;
use ash::vk::ApplicationInfo;
use ash::vk::InstanceCreateInfo;
use ash::Entry;
use ash::Instance;
use log::info;

use super::Extensions;
use crate::error::Result;

pub(crate) struct Vulkan {
    instance: Instance,
    entry: Entry,
}

impl Vulkan {
    pub(crate) fn new(exts: &Extensions) -> Result<Self> {
        info!("initializing the Vulkan API");

        let entry = Entry::new()?;

        match entry.try_enumerate_instance_version()? {
            Some(version) => {
                let major = version_major(version);
                let minor = version_minor(version);
                let patch = version_patch(version);
                info!("using Vulkan {}.{}.{}", major, minor, patch);
            }
            None => info!("using Vulkan 1.0"),
        }

        exts.supports_instance(&entry)?;
        exts.supports_layers(&entry)?;

        let layers = exts.layers();
        let extensions = exts.instance();
        let app_info = ApplicationInfo::builder().api_version(make_version(1, 2, 0));

        let info = InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);

        let instance = unsafe { entry.create_instance(&info, None)? };

        Ok(Self { instance, entry })
    }

    pub(crate) fn instance_ref(&self) -> &Instance {
        &self.instance
    }

    pub(crate) fn entry_ref(&self) -> &Entry {
        &self.entry
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}
