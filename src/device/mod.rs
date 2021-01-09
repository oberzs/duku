// Oliver Berzs
// https://github.com/oberzs/duku

mod commands;
mod pick;
mod stats;

use std::cell::RefCell;
use std::ffi::c_void;
use std::ffi::CString;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem;
use std::ptr;
use std::slice;

pub(crate) use commands::Commands;
pub(crate) use pick::pick_gpu;
pub use stats::Stats;

use crate::buffer::MemoryAccess;
use crate::error::Error;
use crate::error::Result;
use crate::instance::GpuProperties;
use crate::instance::Instance;
use crate::instance::DEVICE_EXTENSIONS;
use crate::surface::Swapchain;
use crate::vk;

pub(crate) const FRAMES_IN_FLIGHT: usize = 2;

pub(crate) struct Device {
    handle: vk::Device,
    properties: GpuProperties,
    queue: (u32, vk::Queue),

    commands: [Commands; FRAMES_IN_FLIGHT],
    sync_acquire: [vk::Semaphore; FRAMES_IN_FLIGHT],
    sync_release: [vk::Semaphore; FRAMES_IN_FLIGHT],
    sync_submit: [vk::Fence; FRAMES_IN_FLIGHT],
    current_frame: usize,

    destroyed_pipelines: RefCell<[Vec<vk::Pipeline>; FRAMES_IN_FLIGHT]>,
    destroyed_buffers: RefCell<[Vec<(vk::Buffer, vk::DeviceMemory)>; FRAMES_IN_FLIGHT]>,
    destroyed_images: RefCell<[Vec<(vk::Image, vk::DeviceMemory)>; FRAMES_IN_FLIGHT]>,
}

impl Device {
    pub(crate) fn new(instance: &Instance, properties: GpuProperties, gpu_index: usize) -> Self {
        // configure device features
        let mut features: &mut [vk::PhysicalDeviceFeatures] = unsafe { &mut [mem::zeroed()] };
        features[0].sampler_anisotropy = vk::TRUE;
        features[0].fill_mode_non_solid = vk::TRUE;
        features[0].wide_lines = vk::TRUE;

        // configure queues
        let queue_index = properties.queue_index.expect("bad queue index");
        let queue_priorities = [1.0f32];
        let queue_infos = [vk::DeviceQueueCreateInfo {
            s_type: vk::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            queue_family_index: queue_index,
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
        }];

        // open GPU
        let c_strings: Vec<_> = DEVICE_EXTENSIONS
            .iter()
            .map(|e| CString::new(*e).expect("bad c string"))
            .collect();
        let extensions: Vec<_> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let info = vk::DeviceCreateInfo {
            s_type: vk::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            queue_create_info_count: queue_infos.len() as u32,
            p_queue_create_infos: queue_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            p_enabled_features: features.as_ptr(),
        };

        let handle = instance.create_device(gpu_index, &info);

        // get device queue
        let mut queue = 0;
        unsafe {
            vk::get_device_queue(handle, queue_index, 0, &mut queue);
        }

        // create synchronization semaphores
        let mut sync_acquire = [0; FRAMES_IN_FLIGHT];
        let mut sync_release = [0; FRAMES_IN_FLIGHT];
        let sem_info = vk::SemaphoreCreateInfo {
            s_type: vk::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
        };
        unsafe {
            vk::check(vk::create_semaphore(
                handle,
                &sem_info,
                ptr::null(),
                &mut sync_acquire[0],
            ));
            vk::check(vk::create_semaphore(
                handle,
                &sem_info,
                ptr::null(),
                &mut sync_acquire[1],
            ));
            vk::check(vk::create_semaphore(
                handle,
                &sem_info,
                ptr::null(),
                &mut sync_release[0],
            ));
            vk::check(vk::create_semaphore(
                handle,
                &sem_info,
                ptr::null(),
                &mut sync_release[1],
            ));
        }

        // create synchronization fences
        let mut sync_submit = [0; FRAMES_IN_FLIGHT];
        let fence_info = vk::FenceCreateInfo {
            s_type: vk::STRUCTURE_TYPE_FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FENCE_CREATE_SIGNALED_BIT,
        };
        unsafe {
            vk::check(vk::create_fence(
                handle,
                &fence_info,
                ptr::null(),
                &mut sync_submit[0],
            ));
            vk::check(vk::create_fence(
                handle,
                &fence_info,
                ptr::null(),
                &mut sync_submit[1],
            ));
        }

        // create command pools and buffers
        let commands = [
            Commands::new(handle, queue_index),
            Commands::new(handle, queue_index),
        ];

        // create destroyed resource storage
        let destroyed_pipelines = [vec![], vec![]];
        let destroyed_buffers = [vec![], vec![]];
        let destroyed_images = [vec![], vec![]];

        Self {
            destroyed_pipelines: RefCell::new(destroyed_pipelines),
            destroyed_buffers: RefCell::new(destroyed_buffers),
            destroyed_images: RefCell::new(destroyed_images),
            queue: (queue_index, queue),
            current_frame: 0,
            properties,
            commands,
            sync_release,
            sync_acquire,
            sync_submit,
            handle,
        }
    }

    pub(crate) fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;
    }

    pub(crate) fn reset_buffers(&mut self) {
        // wait for queue
        let wait = self.sync_submit[self.current_frame];
        unsafe {
            let fences = [wait];
            vk::check(vk::wait_for_fences(
                self.handle,
                1,
                fences.as_ptr(),
                vk::TRUE,
                u64::max_value(),
            ));
            vk::check(vk::reset_fences(self.handle, 1, fences.as_ptr()));
        }

        // reset command buffer
        self.commands[self.current_frame].free(self.handle);

        // cleanup destroyed storage
        self.cleanup_resources(self.current_frame);

        // create new command buffer
        self.commands[self.current_frame].recreate(self.handle);
        self.commands[self.current_frame].reset_stats();

        // begin new command buffer
        self.commands[self.current_frame].begin();
    }

    pub(crate) fn submit_and_wait(&self, buffer: vk::CommandBuffer) {
        let buffers = [buffer];
        let infos = [vk::SubmitInfo {
            s_type: vk::STRUCTURE_TYPE_SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: ptr::null(),
            p_wait_dst_stage_mask: ptr::null(),
            command_buffer_count: 1,
            p_command_buffers: buffers.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores: ptr::null(),
        }];

        unsafe {
            vk::check(vk::queue_submit(
                self.queue.1,
                infos.len() as u32,
                infos.as_ptr(),
                0,
            ));
            vk::check(vk::device_wait_idle(self.handle));
        }
    }

    pub(crate) fn submit(&self, for_swapchain: bool) {
        // end command buffer
        self.commands[self.current_frame].end();

        // submit
        let wait = if for_swapchain {
            vec![self.sync_acquire[self.current_frame]]
        } else {
            vec![]
        };
        let signal = if for_swapchain {
            vec![self.sync_release[self.current_frame]]
        } else {
            vec![]
        };
        let done = self.sync_submit[self.current_frame];
        let buffers = [self.commands[self.current_frame].buffer()];
        let stage_mask = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];

        let infos = [vk::SubmitInfo {
            s_type: vk::STRUCTURE_TYPE_SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait.len() as u32,
            p_wait_semaphores: wait.as_ptr(),
            p_wait_dst_stage_mask: stage_mask.as_ptr(),
            command_buffer_count: buffers.len() as u32,
            p_command_buffers: buffers.as_ptr(),
            signal_semaphore_count: signal.len() as u32,
            p_signal_semaphores: signal.as_ptr(),
        }];

        unsafe {
            vk::check(vk::queue_submit(
                self.queue.1,
                infos.len() as u32,
                infos.as_ptr(),
                done,
            ));
        }
    }

    // returns 'true' if swapchain should be resized
    pub(crate) fn present(&self, swapchain: &Swapchain) -> bool {
        let wait = [self.sync_release[self.current_frame]];
        let image = [swapchain.current() as u32];
        let handle = [swapchain.handle()];

        let info = vk::PresentInfoKHR {
            s_type: vk::STRUCTURE_TYPE_PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: wait.as_ptr(),
            swapchain_count: 1,
            p_swapchains: handle.as_ptr(),
            p_image_indices: image.as_ptr(),
            p_results: ptr::null(),
        };

        let present_result = unsafe { vk::queue_present_khr(self.queue.1, &info) };

        // check if should resize
        if present_result == vk::ERROR_OUT_OF_DATE_KHR {
            true
        } else {
            vk::check(present_result);
            false
        }
    }

    pub(crate) const fn commands(&self) -> &Commands {
        &self.commands[self.current_frame]
    }

    pub(crate) fn wait_idle(&self) {
        unsafe {
            vk::check(vk::device_wait_idle(self.handle));
        }
    }

    pub(crate) fn create_swapchain(&self, info: &vk::SwapchainCreateInfoKHR) -> vk::SwapchainKHR {
        let mut swapchain = 0;
        unsafe {
            vk::check(vk::create_swapchain_khr(
                self.handle,
                info,
                ptr::null(),
                &mut swapchain,
            ));
        }
        swapchain
    }

    pub(crate) fn destroy_swapchain(&self, swapchain: &Swapchain) {
        unsafe {
            vk::destroy_swapchain_khr(self.handle, swapchain.handle(), ptr::null());
        }
    }

    pub(crate) fn get_swapchain_images(&self, swapchain: &Swapchain) -> Vec<vk::Image> {
        unsafe {
            let mut count = 0;
            vk::check(vk::get_swapchain_images_khr(
                self.handle,
                swapchain.handle(),
                &mut count,
                ptr::null_mut(),
            ));
            let mut images: Vec<vk::Image> = Vec::with_capacity(count as usize);
            vk::check(vk::get_swapchain_images_khr(
                self.handle,
                swapchain.handle(),
                &mut count,
                images.as_mut_ptr(),
            ));
            images.set_len(count as usize);
            images
        }
    }

    pub(crate) fn get_next_swapchain_image(&self, swapchain: vk::SwapchainKHR) -> usize {
        let mut index = 0;
        unsafe {
            vk::check(vk::acquire_next_image_khr(
                self.handle,
                swapchain,
                u64::max_value(),
                self.sync_acquire[self.current_frame],
                0,
                &mut index,
            ));
        }
        index as usize
    }

    pub(crate) const fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub(crate) fn stats(&self) -> Stats {
        self.commands[self.current_frame()].stats()
    }

    pub(crate) const fn queue_index(&self) -> u32 {
        self.queue.0
    }

    pub(crate) fn allocate_buffer(
        &self,
        info: &vk::BufferCreateInfo,
        access: MemoryAccess,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        // create buffer handle
        let mut buffer = 0;
        unsafe {
            vk::check(vk::create_buffer(
                self.handle,
                info,
                ptr::null(),
                &mut buffer,
            ));
        }

        // get memory type
        let mut requirements = unsafe { mem::zeroed() };
        unsafe {
            vk::get_buffer_memory_requirements(self.handle, buffer, &mut requirements);
        }
        let mem_type = self.find_memory_type(&requirements, access);

        // allocate memory
        let alloc_info = vk::MemoryAllocateInfo {
            s_type: vk::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: requirements.size,
            memory_type_index: mem_type,
        };
        let mut memory = 0;
        unsafe {
            vk::check(vk::allocate_memory(
                self.handle,
                &alloc_info,
                ptr::null(),
                &mut memory,
            ));
            vk::check(vk::bind_buffer_memory(self.handle, buffer, memory, 0));
        }

        (buffer, memory)
    }

    pub(crate) fn free_buffer(&self, handle: vk::Buffer, memory: vk::DeviceMemory) {
        self.destroyed_buffers.borrow_mut()[self.current_frame].push((handle, memory));
    }

    pub(crate) fn allocate_image(
        &self,
        info: &vk::ImageCreateInfo,
    ) -> (vk::Image, vk::DeviceMemory) {
        // create image handle
        let mut image = 0;
        unsafe {
            vk::check(vk::create_image(self.handle, info, ptr::null(), &mut image));
        }

        // get memory type
        let mut requirements = unsafe { mem::zeroed() };
        unsafe {
            vk::get_image_memory_requirements(self.handle, image, &mut requirements);
        }
        let mem_type = self.find_memory_type(&requirements, MemoryAccess::Gpu);

        // allocate memory
        let alloc_info = vk::MemoryAllocateInfo {
            s_type: vk::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: requirements.size,
            memory_type_index: mem_type,
        };
        let mut memory = 0;
        unsafe {
            vk::check(vk::allocate_memory(
                self.handle,
                &alloc_info,
                ptr::null(),
                &mut memory,
            ));
            vk::check(vk::bind_image_memory(self.handle, image, memory, 0));
        }

        (image, memory)
    }

    pub(crate) fn free_image(&self, image: vk::Image, memory: vk::DeviceMemory) {
        unsafe {
            vk::destroy_image(self.handle, image, ptr::null());
            vk::free_memory(self.handle, memory, ptr::null());
        }
    }

    pub(crate) fn create_image_view(&self, info: &vk::ImageViewCreateInfo) -> vk::ImageView {
        let mut view = 0;
        unsafe {
            vk::check(vk::create_image_view(
                self.handle,
                info,
                ptr::null(),
                &mut view,
            ));
        }
        view
    }

    pub(crate) fn destroy_image_view(&self, view: vk::ImageView) {
        unsafe {
            vk::destroy_image_view(self.handle, view, ptr::null());
        }
    }

    pub(crate) fn map_memory(&self, memory: vk::DeviceMemory, size: usize) -> *mut c_void {
        let mut data = ptr::null_mut();
        unsafe {
            vk::check(vk::map_memory(
                self.handle,
                memory,
                0,
                size as u64,
                0,
                &mut data,
            ));
        }
        data
    }

    pub(crate) fn unmap_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            vk::unmap_memory(self.handle, memory);
        }
    }

    pub(crate) fn create_framebuffer(&self, info: &vk::FramebufferCreateInfo) -> vk::Framebuffer {
        let mut framebuffer = 0;
        unsafe {
            vk::check(vk::create_framebuffer(
                self.handle,
                info,
                ptr::null(),
                &mut framebuffer,
            ));
        }
        framebuffer
    }

    pub(crate) fn destroy_framebuffer(&self, framebuffer: vk::Framebuffer) {
        unsafe {
            vk::destroy_framebuffer(self.handle, framebuffer, ptr::null());
        }
    }

    pub(crate) fn create_descriptor_set_layout(
        &self,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> vk::DescriptorSetLayout {
        let info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
        };
        let mut layout = 0;
        unsafe {
            vk::check(vk::create_descriptor_set_layout(
                self.handle,
                &info,
                ptr::null(),
                &mut layout,
            ));
        }
        layout
    }

    pub(crate) fn destroy_descriptor_set_layout(&self, layout: vk::DescriptorSetLayout) {
        unsafe {
            vk::destroy_descriptor_set_layout(self.handle, layout, ptr::null());
        }
    }

    pub(crate) fn create_descriptor_pool(
        &self,
        pool_sizes: &[vk::DescriptorPoolSize],
    ) -> vk::DescriptorPool {
        let max_sets = pool_sizes.iter().fold(0, |acc, x| acc + x.descriptor_count);
        let info = vk::DescriptorPoolCreateInfo {
            s_type: vk::STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
            max_sets,
        };
        let mut pool = 0;
        unsafe {
            vk::check(vk::create_descriptor_pool(
                self.handle,
                &info,
                ptr::null(),
                &mut pool,
            ));
        }
        pool
    }

    pub(crate) fn destroy_descriptor_pool(&self, pool: vk::DescriptorPool) {
        unsafe {
            vk::destroy_descriptor_pool(self.handle, pool, ptr::null());
        }
    }

    pub(crate) fn create_pipeline_layout(
        &self,
        info: &vk::PipelineLayoutCreateInfo,
    ) -> vk::PipelineLayout {
        let mut layout = 0;
        unsafe {
            vk::check(vk::create_pipeline_layout(
                self.handle,
                info,
                ptr::null(),
                &mut layout,
            ));
        }
        layout
    }

    pub(crate) fn destroy_pipeline_layout(&self, layout: vk::PipelineLayout) {
        unsafe {
            vk::destroy_pipeline_layout(self.handle, layout, ptr::null());
        }
    }

    pub(crate) fn allocate_descriptor_set(
        &self,
        layout: vk::DescriptorSetLayout,
        pool: vk::DescriptorPool,
    ) -> vk::DescriptorSet {
        let layouts = [layout];
        let info = vk::DescriptorSetAllocateInfo {
            s_type: vk::STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool: pool,
            descriptor_set_count: 1,
            p_set_layouts: layouts.as_ptr(),
        };
        let mut set = 0;
        unsafe {
            vk::check(vk::allocate_descriptor_sets(self.handle, &info, &mut set));
        }
        set
    }

    pub(crate) fn update_descriptor_sets(&self, writes: &[vk::WriteDescriptorSet]) {
        unsafe {
            vk::update_descriptor_sets(
                self.handle,
                writes.len() as u32,
                writes.as_ptr(),
                0,
                ptr::null(),
            );
        }
    }

    pub(crate) fn create_render_pass(&self, info: &vk::RenderPassCreateInfo) -> vk::RenderPass {
        let mut pass = 0;
        unsafe {
            vk::check(vk::create_render_pass(
                self.handle,
                info,
                ptr::null(),
                &mut pass,
            ));
        }
        pass
    }

    pub(crate) fn destroy_render_pass(&self, pass: vk::RenderPass) {
        unsafe {
            vk::destroy_render_pass(self.handle, pass, ptr::null());
        }
    }

    pub(crate) fn create_sampler(&self, info: &vk::SamplerCreateInfo) -> vk::Sampler {
        let mut sampler = 0;
        unsafe {
            vk::check(vk::create_sampler(
                self.handle,
                info,
                ptr::null(),
                &mut sampler,
            ));
        }
        sampler
    }

    pub(crate) fn destroy_sampler(&self, sampler: vk::Sampler) {
        unsafe {
            vk::destroy_sampler(self.handle, sampler, ptr::null());
        }
    }

    pub(crate) fn create_pipeline(&self, info: vk::GraphicsPipelineCreateInfo) -> vk::Pipeline {
        let infos = [info];
        let mut pipeline = 0;
        unsafe {
            vk::check(vk::create_graphics_pipelines(
                self.handle,
                0,
                1,
                infos.as_ptr(),
                ptr::null(),
                &mut pipeline,
            ));
        }
        pipeline
    }

    pub(crate) fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        self.destroyed_pipelines.borrow_mut()[self.current_frame].push(pipeline);
    }

    pub(crate) fn create_shader_module(&self, source: &[u8]) -> Result<vk::ShaderModule> {
        let mut cursor = Cursor::new(&source[..]);

        // check data size
        let size = cursor.seek(SeekFrom::End(0)).expect("bad index");
        if size % 4 != 0 {
            return Err(Error::InvalidSpirv);
        }
        if size > usize::max_value() as u64 {
            return Err(Error::InvalidSpirv);
        }

        // read data
        let words = (size / 4) as usize;
        let mut code = Vec::<u32>::with_capacity(words);
        cursor.seek(SeekFrom::Start(0)).expect("bad index");
        unsafe {
            cursor
                .read_exact(slice::from_raw_parts_mut(
                    code.as_mut_ptr() as *mut u8,
                    words * 4,
                ))
                .expect("bad read");
            code.set_len(words);
        }

        // check magic number
        let magic_number = 0x0723_0203u32;
        if !code.is_empty() && code[0] == magic_number.swap_bytes() {
            for word in &mut code {
                *word = word.swap_bytes();
            }
        }
        if code.is_empty() || code[0] != magic_number {
            return Err(Error::InvalidSpirv);
        }

        // create module
        let info = vk::ShaderModuleCreateInfo {
            s_type: vk::STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            code_size: code.len() * 4,
            p_code: code.as_ptr(),
        };
        let mut module = 0;
        unsafe {
            vk::check(vk::create_shader_module(
                self.handle,
                &info,
                ptr::null(),
                &mut module,
            ));
        }
        Ok(module)
    }

    pub(crate) fn destroy_shader_module(&self, module: vk::ShaderModule) {
        unsafe {
            vk::destroy_shader_module(self.handle, module, ptr::null());
        }
    }

    pub(crate) fn do_commands(&self, mut fun: impl FnMut(&Commands)) {
        // create single use commands
        let cmd = Commands::new(self.handle, self.queue.0);

        // do commands
        cmd.begin();
        fun(&cmd);
        cmd.end();
        self.submit_and_wait(cmd.buffer());
        cmd.destroy(self.handle);
    }

    pub(crate) fn destroy(&self) {
        for i in 0..FRAMES_IN_FLIGHT {
            self.cleanup_resources(i);
        }
        unsafe {
            for s in &self.sync_acquire {
                vk::destroy_semaphore(self.handle, *s, ptr::null());
            }
            for s in &self.sync_release {
                vk::destroy_semaphore(self.handle, *s, ptr::null());
            }
            for f in &self.sync_submit {
                vk::destroy_fence(self.handle, *f, ptr::null());
            }
            for c in &self.commands {
                c.destroy(self.handle);
            }
            vk::destroy_device(self.handle, ptr::null());
        }
    }

    fn cleanup_resources(&self, frame: usize) {
        // cleanup pipelines
        let destroyed_pipelines = &mut self.destroyed_pipelines.borrow_mut()[frame];
        for p in destroyed_pipelines.iter() {
            unsafe {
                vk::destroy_pipeline(self.handle, *p, ptr::null());
            }
        }
        destroyed_pipelines.clear();

        // cleanup buffers
        let destroyed_buffers = &mut self.destroyed_buffers.borrow_mut()[frame];
        for (b, m) in destroyed_buffers.iter() {
            unsafe {
                vk::destroy_buffer(self.handle, *b, ptr::null());
                vk::free_memory(self.handle, *m, ptr::null());
            }
        }
        destroyed_buffers.clear();

        // cleanup images
        let destroyed_images = &mut self.destroyed_images.borrow_mut()[frame];
        for (i, m) in destroyed_images.iter() {
            unsafe {
                vk::destroy_image(self.handle, *i, ptr::null());
                vk::free_memory(self.handle, *m, ptr::null());
            }
        }
        destroyed_images.clear();
    }

    fn find_memory_type(&self, requirements: &vk::MemoryRequirements, access: MemoryAccess) -> u32 {
        self.properties
            .memory
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, mem_type)| {
                let has_type = (requirements.memory_type_bits & (1 << i)) != 0;
                let has_properties = (mem_type.property_flags & access.flag()) == access.flag();
                has_type && has_properties
            })
            .expect("bad memory type")
            .0 as u32
    }
}
