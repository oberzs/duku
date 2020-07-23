// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// DeviceProperties - properties for the specific GPU

use ash::vk;

use super::extension;
use crate::error::Result;
use crate::image::Msaa;
use crate::instance::Instance;

#[derive(Clone)]
pub(crate) struct DeviceProperties {
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) features: vk::PhysicalDeviceFeatures,
    pub(crate) memory: vk::PhysicalDeviceMemoryProperties,
    pub(crate) msaa: Msaa,
    pub(crate) supports_extensions: bool,
}

impl DeviceProperties {
    pub(crate) fn new(instance: &Instance, msaa: Msaa) -> Result<Vec<Self>> {
        profile_scope!("new");

        let properties = instance.get_device_properties().into_iter();
        let features = instance.get_device_features().into_iter();
        let memory = instance.get_device_memory().into_iter();
        let available_extensions = instance.get_device_extensions()?.into_iter();

        // check extension support
        let mut supports = vec![];
        for available in available_extensions {
            let mut s = true;
            for extension in extension::list() {
                if !available.contains(&extension) {
                    s = false;
                }
            }
            supports.push(s);
        }

        Ok(properties
            .zip(features.zip(memory.zip(supports.into_iter())))
            .map(|(p, (f, (m, s)))| Self {
                properties: p,
                features: f,
                memory: m,
                msaa: pick_msaa(p, msaa),
                supports_extensions: s,
            })
            .collect())
    }
}

fn pick_msaa(properties: vk::PhysicalDeviceProperties, msaa: Msaa) -> Msaa {
    let counts = properties.limits.framebuffer_color_sample_counts
        & properties.limits.framebuffer_depth_sample_counts;

    if !counts.contains(msaa.flag()) {
        warn!("unsupported MSAA value: {:?}", msaa);
        Msaa::Disabled
    } else {
        info!("using MSAA level {:?}", msaa);
        msaa
    }
}
