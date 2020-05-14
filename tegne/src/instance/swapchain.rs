use ash::extensions::khr::Swapchain as Extension;
use ash::vk::ColorSpaceKHR;
use ash::vk::CompositeAlphaFlagsKHR;
use ash::vk::Image;
use ash::vk::PresentInfoKHR;
use ash::vk::Queue;
use ash::vk::Semaphore;
use ash::vk::SharingMode;
use ash::vk::SwapchainCreateInfoKHR;
use ash::vk::SwapchainKHR;
use log::debug;
use std::cell::Cell;
use std::sync::Arc;

use super::Device;
use super::Surface;
use super::Vulkan;
use crate::error::Result;
use crate::images::ImageFormat;
use crate::images::ImageUsage;

pub(crate) struct Swapchain {
    ext: Extension,
    vk: SwapchainKHR,
    current_image: Cell<u32>,
}

impl Swapchain {
    pub(crate) fn new(vulkan: &Vulkan, device: &Arc<Device>, surface: &Surface) -> Result<Self> {
        debug!("creating window swapchain");

        let props = device.properties();
        let transform = props.surface_capabilities.current_transform;

        let mut create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface.vk())
            .image_format(ImageFormat::Bgra.flag())
            .image_color_space(ColorSpaceKHR::SRGB_NONLINEAR)
            .image_extent(props.extent)
            .image_array_layers(1)
            .image_usage(ImageUsage::Color.flag())
            .pre_transform(transform)
            .min_image_count(props.image_count)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(props.present_mode)
            .clipped(true);

        let indices = device.indices();
        if device.are_indices_unique() {
            create_info = create_info
                .image_sharing_mode(SharingMode::CONCURRENT)
                .queue_family_indices(&indices);
        } else {
            create_info = create_info.image_sharing_mode(SharingMode::EXCLUSIVE);
        }

        let ext = Extension::new(vulkan.instance_ref(), device.logical());
        let vk = unsafe { ext.create_swapchain(&create_info, None)? };

        Ok(Self {
            ext,
            vk,
            current_image: Cell::new(0),
        })
    }

    pub(crate) fn iter_images(&self) -> Result<impl Iterator<Item = Image>> {
        Ok(unsafe { self.ext.get_swapchain_images(self.vk)?.into_iter() })
    }

    pub(crate) fn next(&self, signal: Semaphore) -> Result<()> {
        self.current_image.set(unsafe {
            self.ext
                .acquire_next_image(self.vk, u64::max_value(), signal, Default::default())?
                .0
        });
        Ok(())
    }

    pub(crate) fn current(&self) -> usize {
        self.current_image.get() as usize
    }

    pub(crate) fn present(&self, queue: Queue, wait: Semaphore) -> Result<()> {
        let waits = [wait];
        let swapchains = [self.vk];
        let image = [self.current_image.get()];
        let info = PresentInfoKHR::builder()
            .wait_semaphores(&waits)
            .swapchains(&swapchains)
            .image_indices(&image);

        unsafe {
            self.ext.queue_present(queue, &info)?;
        }
        Ok(())
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.ext.destroy_swapchain(self.vk, None);
        }
    }
}
