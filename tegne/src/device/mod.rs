// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Device - struct to access GPU API layer

mod commands;
mod extension;
mod pick;
mod properties;

use ash::extensions::khr::Swapchain as SwapchainExt;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::Device as VkDevice;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub(crate) use commands::Commands;
pub(crate) use commands::LayoutChangeOptions;
pub(crate) use pick::pick_gpu;
pub(crate) use properties::DeviceProperties;

use crate::error::Result;
use crate::instance::layer;
use crate::instance::Instance;
use crate::sync::fence;
use crate::sync::semaphore;
use crate::window::SurfaceProperties;
use crate::window::Swapchain;

pub(crate) const IN_FLIGHT_FRAME_COUNT: usize = 2;

pub(crate) struct Device {
    handle: VkDevice,
    swapchain_ext: SwapchainExt,
    graphics_queue: (u32, vk::Queue),
    present_queue: (u32, vk::Queue),
    memory_types: Vec<vk::MemoryType>,
    sync_acquire_image: Vec<vk::Semaphore>,
    sync_release_image: Vec<vk::Semaphore>,
    sync_queue_submit: Vec<vk::Fence>,
    current_frame: AtomicUsize,
}

impl Device {
    pub(crate) fn new(
        instance: &Instance,
        surface_properties: &SurfaceProperties,
        device_properties: &DeviceProperties,
        gpu_index: usize,
    ) -> Result<Self> {
        // configure device features
        let features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .fill_mode_non_solid(true)
            .wide_lines(true);

        // configure queues
        let g_index = surface_properties.graphics_index();
        let p_index = surface_properties.present_index();
        let queue_priorities = [1.0];

        let g_queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(g_index)
            .queue_priorities(&queue_priorities)
            .build();
        let p_queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(p_index)
            .queue_priorities(&queue_priorities)
            .build();

        let mut queue_infos = vec![g_queue_info];
        if g_index != p_index {
            queue_infos.push(p_queue_info);
        }

        let extension_list = extension::list()?;
        let extensions = extension::to_i8(&extension_list);
        let layer_list = layer::list()?;
        let layers = layer::to_i8(&layer_list);

        // open GPU
        let info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_features(&features)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);

        let handle = instance.create_device(gpu_index, &info)?;

        // create swapchain extension
        let swapchain_ext = instance.create_swapchain_extension(&handle);

        // get device queues
        let graphics_queue = unsafe { handle.get_device_queue(g_index, 0) };
        let present_queue = unsafe { handle.get_device_queue(p_index, 0) };

        // create synchronization objects
        let mut sync_acquire_image = vec![];
        let mut sync_release_image = vec![];
        let mut sync_queue_submit = vec![];
        for _ in 0..IN_FLIGHT_FRAME_COUNT {
            sync_acquire_image.push(semaphore::create(&handle)?);
            sync_release_image.push(semaphore::create(&handle)?);
            sync_queue_submit.push(fence::create(&handle)?);
        }

        let memory_types = device_properties.memory.memory_types.to_vec();

        Ok(Self {
            handle,
            swapchain_ext,
            graphics_queue: (g_index, graphics_queue),
            present_queue: (p_index, present_queue),
            memory_types,
            sync_acquire_image,
            sync_release_image,
            sync_queue_submit,
            current_frame: AtomicUsize::new(0),
        })
    }

    pub(crate) fn current_frame(&self) -> usize {
        self.current_frame.load(Ordering::Relaxed)
    }

    pub(crate) fn next_frame(&self, swapchain: &Swapchain) -> Result<()> {
        let mut current = self.current_frame();
        current = (current + 1) % IN_FLIGHT_FRAME_COUNT;

        swapchain.next(self.sync_acquire_image[current])?;

        // wait for queue
        let wait = self.sync_queue_submit[current];
        fence::wait_for(&self.handle, wait)?;
        fence::reset(&self.handle, wait)?;

        self.current_frame.store(current, Ordering::Release);

        Ok(())
    }

    pub(crate) fn submit_and_wait(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let buffers = [buffer];
        let info = vk::SubmitInfo::builder().command_buffers(&buffers).build();
        let infos = [info];

        unsafe {
            self.handle
                .queue_submit(self.graphics_queue.1, &infos, vk::Fence::null())?;
            self.handle.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn submit(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let current = self.current_frame();
        let wait = [self.sync_acquire_image[current]];
        let signal = [self.sync_release_image[current]];
        let done = self.sync_queue_submit[current];
        let buffers = [buffer];
        let stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let info = [vk::SubmitInfo::builder()
            .wait_semaphores(&wait)
            .signal_semaphores(&signal)
            .wait_dst_stage_mask(&stage_mask)
            .command_buffers(&buffers)
            .build()];
        unsafe {
            self.handle
                .queue_submit(self.graphics_queue.1, &info, done)?
        };
        Ok(())
    }

    pub(crate) fn present(&self, swapchain: &Swapchain) -> Result<()> {
        let current = self.current_frame();
        let wait = self.sync_release_image[current];
        swapchain.present(wait)?;
        Ok(())
    }

    pub(crate) fn wait_for_idle(&self) -> Result<()> {
        for fen in self.sync_queue_submit.iter() {
            fence::wait_for(&self.handle, *fen)?;
        }

        unsafe {
            self.handle.queue_wait_idle(self.graphics_queue.1)?;
            self.handle.queue_wait_idle(self.present_queue.1)?;
            self.handle.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn create_swapchain(
        &self,
        info: &vk::SwapchainCreateInfoKHR,
    ) -> Result<vk::SwapchainKHR> {
        Ok(unsafe { self.swapchain_ext.create_swapchain(info, None)? })
    }

    pub(crate) fn destroy_swapchain(&self, handle: vk::SwapchainKHR) {
        unsafe {
            self.swapchain_ext.destroy_swapchain(handle, None);
        }
    }

    pub(crate) fn get_swapchain_images(&self, handle: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
        Ok(unsafe { self.swapchain_ext.get_swapchain_images(handle)? })
    }

    pub(crate) fn get_next_swapchain_image(
        &self,
        handle: vk::SwapchainKHR,
        signal: vk::Semaphore,
    ) -> Result<u32> {
        Ok(unsafe {
            self.swapchain_ext
                .acquire_next_image(handle, u64::max_value(), signal, Default::default())?
                .0
        })
    }

    pub(crate) fn present_queue(&self, info: &vk::PresentInfoKHR) -> Result<()> {
        unsafe {
            self.swapchain_ext
                .queue_present(self.present_queue.1, info)?;
        }
        Ok(())
    }

    pub(crate) fn graphics_index(&self) -> u32 {
        self.graphics_queue.0
    }

    pub(crate) fn find_memory_type(
        &self,
        type_filter: u32,
        props: vk::MemoryPropertyFlags,
    ) -> Option<u32> {
        self.memory_types
            .iter()
            .enumerate()
            .find(|(i, mem_type)| {
                let byte: u32 = 1 << i;
                (type_filter & byte != 0) && (mem_type.property_flags & props) == props
            })
            .map(|t| t.0 as u32)
    }

    pub(crate) fn logical(&self) -> &VkDevice {
        &self.handle
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.sync_acquire_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.handle, *s));
            self.sync_release_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.handle, *s));
            self.sync_queue_submit
                .iter()
                .for_each(|f| fence::destroy(&self.handle, *f));
            self.handle.destroy_device(None);
        }
    }
}
