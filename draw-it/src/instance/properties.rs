// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// GPUProperties - properties for the specific GPU

use crate::image::Msaa;
use crate::surface::VSync;
use crate::vk;

pub(crate) struct GPUProperties {
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) features: vk::PhysicalDeviceFeatures,
    pub(crate) memory: vk::PhysicalDeviceMemoryProperties,
    pub(crate) capabilities: vk::SurfaceCapabilitiesKHR,
    pub(crate) formats: Vec<vk::SurfaceFormatKHR>,
    pub(crate) present_modes: Vec<vk::PresentModeKHR>,

    pub(crate) queue_index: Option<u32>,

    pub(crate) extent: vk::Extent2D,
    pub(crate) image_count: u32,

    pub(crate) supports_extensions: bool,
}

impl GPUProperties {
    pub(crate) const fn supports_msaa(&self, msaa: Msaa) -> bool {
        let counts = self.properties.limits.framebuffer_color_sample_counts
            & self.properties.limits.framebuffer_depth_sample_counts;
        (counts & msaa.flag()) != 0
    }

    pub(crate) fn supports_present_mode(&self, vsync: VSync) -> bool {
        self.present_modes.contains(&vsync.flag())
    }
}
