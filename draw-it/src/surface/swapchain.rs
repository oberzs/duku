// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Swapchain - struct that holds images for specific surface

use std::ptr;
use std::rc::Rc;

use super::ColorSpace;
use super::Surface;
use super::VSync;
use crate::device::Device;
use crate::image::ImageFormat;
use crate::image::ImageUsage;
use crate::instance::GPUProperties;
use crate::vk;

pub(crate) struct Swapchain {
    handle: vk::SwapchainKHR,
    current_image: usize,
    width: u32,
    height: u32,
    device: Rc<Device>,
}

impl Swapchain {
    pub(crate) fn new(
        device: &Rc<Device>,
        surface: &Surface,
        gpu_properties: &GPUProperties,
        vsync: VSync,
    ) -> Self {
        let info = swapchain_info(surface, &gpu_properties, vsync);
        let handle = device.create_swapchain(&info);
        let width = gpu_properties.extent.width;
        let height = gpu_properties.extent.height;

        Self {
            device: Rc::clone(device),
            current_image: 0,
            handle,
            width,
            height,
        }
    }

    pub(crate) fn recreate(
        &mut self,
        surface: &Surface,
        gpu_properties: &GPUProperties,
        vsync: VSync,
    ) {
        self.device.destroy_swapchain(self.handle);
        let info = swapchain_info(surface, gpu_properties, vsync);
        self.handle = self.device.create_swapchain(&info);
        self.width = gpu_properties.extent.width;
        self.height = gpu_properties.extent.height;
        self.current_image = 0;
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = vk::Image> {
        self.device.get_swapchain_images(self.handle).into_iter()
    }

    pub(crate) fn next(&mut self, signal: vk::Semaphore) {
        self.current_image = self.device.get_next_swapchain_image(self.handle, signal);
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

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.device.destroy_swapchain(self.handle);
    }
}

fn swapchain_info(
    surface: &Surface,
    gpu_properties: &GPUProperties,
    vsync: VSync,
) -> vk::SwapchainCreateInfoKHR {
    let transform = gpu_properties.capabilities.current_transform;
    let image_count = gpu_properties.image_count;
    let extent = gpu_properties.extent;
    let indices = [gpu_properties.queue_index.expect("bad queue index")];

    vk::SwapchainCreateInfoKHR {
        s_type: vk::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: 0,
        surface: surface.handle(),
        min_image_count: image_count,
        image_format: ImageFormat::Sbgra.flag(),
        image_color_space: ColorSpace::Srgb.flag(),
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
    }
}
