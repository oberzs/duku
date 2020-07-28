// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Swapchain - struct that holds images for specific surface

use ash::vk;
use std::sync::Arc;

use super::Surface;
use super::SurfaceProperties;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageUsage;
use crate::instance::Instance;

pub(crate) struct Swapchain {
    handle: vk::SwapchainKHR,
    surface_properties: SurfaceProperties,
    current_image: usize,
    device: Arc<Device>,
}

impl Swapchain {
    pub(crate) fn new(
        device: &Arc<Device>,
        surface: &Surface,
        surface_properties: SurfaceProperties,
    ) -> Result<Self> {
        profile_scope!("new");

        let info = swapchain_info(surface, &surface_properties);
        let handle = device.create_swapchain(&info)?;

        Ok(Self {
            handle,
            surface_properties,
            current_image: 0,
            device: device.clone(),
        })
    }

    pub(crate) fn recreate(
        &mut self,
        instance: &Instance,
        surface: &Surface,
        gpu_index: usize,
    ) -> Result<()> {
        self.surface_properties
            .refresh(instance, surface, gpu_index)?;
        self.device.destroy_swapchain(self.handle);
        let info = swapchain_info(surface, &self.surface_properties);
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

    pub(crate) fn extent(&self) -> vk::Extent2D {
        self.surface_properties.extent
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
    surface_properties: &SurfaceProperties,
) -> vk::SwapchainCreateInfoKHR {
    let transform = surface_properties.capabilities.current_transform;
    let image_count = surface_properties.image_count;
    let present_mode = surface_properties.present_mode;
    let extent = surface_properties.extent;

    let mut info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface.handle())
        .image_format(ImageFormat::Sbgra.flag())
        .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(ImageUsage::Color.flag())
        .pre_transform(transform)
        .min_image_count(image_count)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    let indices = surface_properties.indices();
    if surface_properties.are_indices_unique() {
        info = info
            .image_sharing_mode(vk::SharingMode::CONCURRENT)
            .queue_family_indices(&indices);
    } else {
        info = info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
    }

    info.build()
}
