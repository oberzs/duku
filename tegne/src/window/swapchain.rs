// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Swapchain - struct that holds images for specific surface

use ash::vk;
use log::debug;
use std::cell::Cell;
use std::sync::Arc;

use super::Surface;
use super::SurfaceProperties;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::ImageUsage;

pub(crate) struct Swapchain {
    handle: vk::SwapchainKHR,
    current_image: Cell<u32>,
    device: Arc<Device>,
}

impl Swapchain {
    pub(crate) fn new(
        device: &Arc<Device>,
        surface: &Surface,
        surface_properties: &SurfaceProperties,
    ) -> Result<Self> {
        debug!("creating window swapchain");

        let info = swapchain_info(surface, surface_properties);
        let handle = device.create_swapchain(&info)?;

        Ok(Self {
            handle,
            current_image: Cell::new(0),
            device: device.clone(),
        })
    }

    pub(crate) fn recreate(
        &mut self,
        surface: &Surface,
        surface_properties: &SurfaceProperties,
    ) -> Result<()> {
        self.device.destroy_swapchain(self.handle);
        let info = swapchain_info(surface, surface_properties);
        self.handle = self.device.create_swapchain(&info)?;
        self.current_image.set(0);
        Ok(())
    }

    pub(crate) fn iter_images(&self) -> Result<impl Iterator<Item = vk::Image>> {
        Ok(self.device.get_swapchain_images(self.handle)?.into_iter())
    }

    pub(crate) fn next(&self, signal: vk::Semaphore) -> Result<()> {
        self.current_image
            .set(self.device.get_next_swapchain_image(self.handle, signal)?);
        Ok(())
    }

    pub(crate) fn current(&self) -> usize {
        self.current_image.get() as usize
    }

    pub(crate) fn present(&self, wait: vk::Semaphore) -> Result<()> {
        let waits = [wait];
        let swapchains = [self.handle];
        let image = [self.current_image.get()];
        let info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&waits)
            .swapchains(&swapchains)
            .image_indices(&image);

        self.device.present_queue(&info)?;
        Ok(())
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
        .image_format(ImageFormat::Bgra.flag())
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
