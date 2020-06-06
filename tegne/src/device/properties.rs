// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// DeviceProperties - properties for the specific GPU

use ash::vk;
use log::info;
use log::warn;

use super::extension;
use crate::error::Result;
use crate::image::ImageSamples;
use crate::instance::Instance;
use crate::profile_scope;

#[derive(Clone)]
pub(crate) struct DeviceProperties {
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) features: vk::PhysicalDeviceFeatures,
    pub(crate) memory: vk::PhysicalDeviceMemoryProperties,
    pub(crate) samples: ImageSamples,
    pub(crate) supports_extensions: bool,
}

impl DeviceProperties {
    pub(crate) fn new(instance: &Instance, msaa: u8) -> Result<Vec<Self>> {
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
                samples: pick_samples(p, msaa),
                supports_extensions: s,
            })
            .collect())
    }
}

fn pick_samples(properties: vk::PhysicalDeviceProperties, msaa: u8) -> ImageSamples {
    let counts = properties.limits.framebuffer_color_sample_counts
        & properties.limits.framebuffer_depth_sample_counts;

    let samples = ImageSamples(msaa);

    if samples.flag() == vk::SampleCountFlags::TYPE_1 && msaa != 1 {
        warn!("invalid MSAA value: {}", msaa);
        ImageSamples(1)
    } else if !counts.contains(samples.flag()) {
        warn!("unsupported MSAA value: {}", msaa);
        ImageSamples(1)
    } else {
        info!("using MSAA level {}", msaa);
        samples
    }
}
