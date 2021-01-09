// Oliver Berzs
// https://github.com/oberzs/duku

// Swapchain - struct that holds images for specific surface

use std::ptr;

use super::Surface;
use super::VSync;
use crate::device::Device;
use crate::image::Format;
use crate::image::ImageUsage;
use crate::instance::SurfaceProperties;
use crate::vk;

pub(crate) struct Swapchain {
    handle: vk::SwapchainKHR,
    current_image: usize,
    width: u32,
    height: u32,
}

impl Swapchain {
    pub(crate) fn new(
        device: &Device,
        surface: &Surface,
        properties: SurfaceProperties,
        vsync: VSync,
    ) -> Self {
        let transform = properties.capabilities.current_transform;
        let image_count = properties.image_count;
        let extent = properties.extent;
        let indices = [device.queue_index()];

        let info = vk::SwapchainCreateInfoKHR {
            s_type: vk::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: 0,
            surface: surface.handle(),
            min_image_count: image_count,
            image_format: Format::Bgra.flag(),
            image_color_space: vk::COLOR_SPACE_SRGB_NONLINEAR_KHR,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: ImageUsage::Color.flag(),
            image_sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 1,
            p_queue_family_indices: indices.as_ptr(),
            pre_transform: transform,
            composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            present_mode: vsync.flag(),
            clipped: vk::TRUE,
            old_swapchain: 0,
        };

        let handle = device.create_swapchain(&info);

        Self {
            width: extent.width,
            height: extent.height,
            current_image: 0,
            handle,
        }
    }

    pub(crate) fn recreate(
        &mut self,
        device: &Device,
        surface: &Surface,
        properties: SurfaceProperties,
        vsync: VSync,
    ) {
        device.destroy_swapchain(self);

        let transform = properties.capabilities.current_transform;
        let image_count = properties.image_count;
        let extent = properties.extent;
        let indices = [device.queue_index()];

        let info = vk::SwapchainCreateInfoKHR {
            s_type: vk::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: 0,
            surface: surface.handle(),
            min_image_count: image_count,
            image_format: Format::Bgra.flag(),
            image_color_space: vk::COLOR_SPACE_SRGB_NONLINEAR_KHR,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: ImageUsage::Color.flag(),
            image_sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 1,
            p_queue_family_indices: indices.as_ptr(),
            pre_transform: transform,
            composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            present_mode: vsync.flag(),
            clipped: vk::TRUE,
            old_swapchain: 0,
        };

        self.handle = device.create_swapchain(&info);
        self.width = extent.width;
        self.height = extent.height;
        self.current_image = 0;
    }

    pub(crate) fn next(&mut self, device: &Device) {
        let next_image = device.get_next_swapchain_image(self.handle);
        self.current_image = next_image;
    }

    pub(crate) const fn current(&self) -> usize {
        self.current_image
    }

    pub(crate) const fn width(&self) -> u32 {
        self.width
    }

    pub(crate) const fn height(&self) -> u32 {
        self.height
    }

    pub(crate) const fn handle(&self) -> vk::SwapchainKHR {
        self.handle
    }
}
