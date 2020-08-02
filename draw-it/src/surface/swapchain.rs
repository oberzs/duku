// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Swapchain - struct that holds images for specific surface

use ash::vk;
use std::sync::Arc;

use super::ColorSpace;
use super::Surface;
use super::VSync;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageUsage;
use crate::instance::GPUProperties;

pub(crate) struct Swapchain {
    handle: vk::SwapchainKHR,
    current_image: usize,
    width: u32,
    height: u32,
    device: Arc<Device>,
}

impl Swapchain {
    pub(crate) fn new(
        device: &Arc<Device>,
        surface: &Surface,
        gpu_properties: &GPUProperties,
        vsync: VSync,
    ) -> Result<Self> {
        let info = swapchain_info(surface, &gpu_properties, vsync);
        let handle = device.create_swapchain(&info)?;
        let width = gpu_properties.extent.width;
        let height = gpu_properties.extent.height;

        Ok(Self {
            device: Arc::clone(device),
            current_image: 0,
            handle,
            width,
            height,
        })
    }

    pub(crate) fn recreate(
        &mut self,
        surface: &Surface,
        gpu_properties: &GPUProperties,
        vsync: VSync,
    ) -> Result<()> {
        self.device.destroy_swapchain(self.handle);
        let info = swapchain_info(surface, gpu_properties, vsync);
        self.handle = self.device.create_swapchain(&info)?;
        self.current_image = 0;
        Ok(())
    }

    pub(crate) fn iter_images(&self) -> Result<impl Iterator<Item = vk::Image>> {
        Ok(self.device.get_swapchain_images(self.handle)?.into_iter())
    }

    pub(crate) fn next(&mut self, signal: vk::Semaphore) -> Result<()> {
        self.current_image = self.device.get_next_swapchain_image(self.handle, signal)?;
        Ok(())
    }

    pub(crate) fn current(&self) -> usize {
        self.current_image as usize
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn handle(&self) -> vk::SwapchainKHR {
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

    let mut info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface.handle())
        .image_format(ImageFormat::Sbgra.flag())
        .image_color_space(ColorSpace::Srgb.flag())
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(ImageUsage::Color.flag())
        .pre_transform(transform)
        .min_image_count(image_count)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vsync.flag())
        .clipped(true);

    let indices = [
        gpu_properties.graphics_index.expect("bad graphics index"),
        gpu_properties.present_index.expect("bad present index"),
    ];
    if indices[0] != indices[1] {
        info = info
            .image_sharing_mode(vk::SharingMode::CONCURRENT)
            .queue_family_indices(&indices);
    } else {
        info = info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
    }

    info.build()
}
