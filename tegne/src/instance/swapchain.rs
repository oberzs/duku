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
use super::Vulkan;
use super::WindowSurface;
use crate::images::ImageFormat;
use crate::images::ImageUsage;
use crate::utils::OrError;

pub(crate) struct Swapchain {
    ext: Extension,
    vk: SwapchainKHR,
    current_image: Cell<u32>,
}

impl Swapchain {
    pub(crate) fn new(
        vulkan: &Vulkan,
        device: &Arc<Device>,
        window_surface: &WindowSurface,
        width: u32,
        height: u32,
    ) -> Self {
        debug!("creating window swapchain");

        let image_count = device.pick_image_count();
        let extent = device.pick_extent(width, height);
        let present_mode = device.pick_present_mode();
        let transform = device.properties().surface_capabilities.current_transform;

        let mut create_info = SwapchainCreateInfoKHR::builder()
            .surface(window_surface.vk())
            .image_format(ImageFormat::Bgra.flag())
            .image_color_space(ColorSpaceKHR::SRGB_NONLINEAR)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(ImageUsage::Color.flag())
            .pre_transform(transform)
            .min_image_count(image_count)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
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
        let vk = unsafe {
            ext.create_swapchain(&create_info, None)
                .or_error("cannot create swapchain")
        };

        Self {
            ext,
            vk,
            current_image: Cell::new(0),
        }
    }

    pub(crate) fn iter_images(&self) -> impl Iterator<Item = Image> {
        unsafe {
            self.ext
                .get_swapchain_images(self.vk)
                .or_error("cannot get swapchain images")
                .into_iter()
        }
    }

    pub(crate) fn next(&self, signal: Semaphore) {
        self.current_image.set(unsafe {
            self.ext
                .acquire_next_image(self.vk, u64::max_value(), signal, Default::default())
                .or_error("cannot acquire next image")
                .0
        });
    }

    pub(crate) fn current(&self) -> usize {
        self.current_image.get() as usize
    }

    pub(crate) fn present(&self, queue: Queue, wait: Semaphore) {
        let waits = [wait];
        let swapchains = [self.vk];
        let image = [self.current_image.get()];
        let info = PresentInfoKHR::builder()
            .wait_semaphores(&waits)
            .swapchains(&swapchains)
            .image_indices(&image);

        unsafe {
            self.ext
                .queue_present(queue, &info)
                .or_error("cannot present queue");
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.ext.destroy_swapchain(self.vk, None);
        }
    }
}
