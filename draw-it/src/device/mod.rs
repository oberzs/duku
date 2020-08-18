// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Device - struct to access GPU API layer

mod pick;

use ash::extensions::khr::Swapchain as SwapchainExt;
use ash::util;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::Device as VkDevice;
use std::cell::Cell;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashSet;
use std::ffi::c_void;
use std::io::Cursor;
use std::mem;
use std::ops::Range;
use std::slice;

pub(crate) use pick::pick_gpu;

use crate::buffer::BufferAccess;
use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::image::ImageLayout;
use crate::image::ImageMemory;
use crate::instance::GPUProperties;
use crate::instance::Instance;
use crate::instance::DEVICE_EXTENSIONS;
use crate::mesh::CoreMesh;
use crate::pipeline::CoreMaterial;
use crate::pipeline::CoreShader;
use crate::pipeline::Descriptor;
use crate::pipeline::PushConstants;
use crate::pipeline::ShaderLayout;
use crate::pipeline::Uniform;
use crate::surface::Swapchain;
use crate::sync::fence;
use crate::sync::semaphore;

pub(crate) const IN_FLIGHT_FRAME_COUNT: usize = 2;

pub(crate) struct Device {
    handle: VkDevice,
    swapchain_ext: SwapchainExt,

    graphics_queue: (u32, vk::Queue),
    present_queue: (u32, vk::Queue),
    memory_types: Vec<vk::MemoryType>,

    command_pools: [vk::CommandPool; IN_FLIGHT_FRAME_COUNT],
    command_buffers: RefCell<[vk::CommandBuffer; IN_FLIGHT_FRAME_COUNT]>,
    sync_acquire_image: [vk::Semaphore; IN_FLIGHT_FRAME_COUNT],
    sync_release_image: [vk::Semaphore; IN_FLIGHT_FRAME_COUNT],
    sync_queue_submit: [vk::Fence; IN_FLIGHT_FRAME_COUNT],
    current_frame: Cell<usize>,

    destroyed_pipelines: RefCell<[Vec<vk::Pipeline>; IN_FLIGHT_FRAME_COUNT]>,
    destroyed_buffers: RefCell<[Vec<(vk::Buffer, vk::DeviceMemory)>; IN_FLIGHT_FRAME_COUNT]>,
    destroyed_images: RefCell<[Vec<(vk::Image, vk::DeviceMemory)>; IN_FLIGHT_FRAME_COUNT]>,

    stats: Cell<Stats>,
    used_materials: RefCell<HashSet<Descriptor>>,
    used_shaders: RefCell<HashSet<vk::Pipeline>>,
}

#[derive(Copy, Clone, Default)]
pub struct Stats {
    pub drawn_indices: u32,
    pub shaders_used: u32,
    pub shader_rebinds: u32,
    pub materials_used: u32,
    pub material_rebinds: u32,
    pub draw_calls: u32,
}

impl Device {
    pub(crate) fn new(
        instance: &Instance,
        gpu_properties: &GPUProperties,
        gpu_index: usize,
    ) -> Result<Self> {
        // configure device features
        let features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .fill_mode_non_solid(true)
            .wide_lines(true);

        // configure queues
        let g_index = gpu_properties.graphics_index.expect("bad graphics index");
        let p_index = gpu_properties.present_index.expect("bad present index");
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

        // open GPU
        let extensions = DEVICE_EXTENSIONS.as_ptr();
        let info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_features(&features)
            .enabled_extension_names(&extensions);

        let handle = instance.create_device(gpu_index, &info)?;

        // create swapchain extension
        let swapchain_ext = instance.create_swapchain_extension(&handle);

        // get device queues
        let graphics_queue = unsafe { handle.get_device_queue(g_index, 0) };
        let present_queue = unsafe { handle.get_device_queue(p_index, 0) };

        let memory_types = gpu_properties.memory.memory_types.to_vec();

        // create synchronization objects
        let sync_acquire_image = [semaphore::create(&handle)?, semaphore::create(&handle)?];
        let sync_release_image = [semaphore::create(&handle)?, semaphore::create(&handle)?];
        let sync_queue_submit = [fence::create(&handle)?, fence::create(&handle)?];

        // create command pools and buffers
        let pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(g_index);
        let command_pools = unsafe {
            [
                handle.create_command_pool(&pool_info, None)?,
                handle.create_command_pool(&pool_info, None)?,
            ]
        };
        let command_buffers = unsafe {
            [
                handle.allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pools[0])
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )?[0],
                handle.allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pools[1])
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )?[0],
            ]
        };

        // create destroyed resource storage
        let destroyed_pipelines = [vec![], vec![]];
        let destroyed_buffers = [vec![], vec![]];
        let destroyed_images = [vec![], vec![]];

        Ok(Self {
            destroyed_pipelines: RefCell::new(destroyed_pipelines),
            destroyed_buffers: RefCell::new(destroyed_buffers),
            destroyed_images: RefCell::new(destroyed_images),
            command_buffers: RefCell::new(command_buffers),
            graphics_queue: (g_index, graphics_queue),
            present_queue: (p_index, present_queue),
            current_frame: Cell::new(0),
            stats: Cell::new(Stats::default()),
            used_materials: RefCell::new(HashSet::new()),
            used_shaders: RefCell::new(HashSet::new()),
            memory_types,
            sync_release_image,
            sync_acquire_image,
            sync_queue_submit,
            command_pools,
            swapchain_ext,
            handle,
        })
    }

    pub(crate) fn next_frame(&self, swapchain: &mut Swapchain) -> Result<()> {
        let mut current = self.current_frame.get();
        current = (current + 1) % IN_FLIGHT_FRAME_COUNT;
        let mut buffers = self.command_buffers.borrow_mut();

        swapchain.next(self.sync_acquire_image[current])?;

        // wait for queue
        let wait = self.sync_queue_submit[current];
        fence::wait_for(&self.handle, wait)?;
        fence::reset(&self.handle, wait)?;

        // reset command buffer
        let pool = self.command_pools[current];
        self.free_command_buffer(pool, buffers[current])?;

        // cleanup destroyed storage
        self.cleanup_resources(current);

        // reset stats
        self.stats.set(Stats::default());
        self.used_materials.borrow_mut().clear();
        self.used_shaders.borrow_mut().clear();

        // create new command buffer
        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        buffers[current] = unsafe { self.handle.allocate_command_buffers(&buffer_info)?[0] };

        // begin new command buffer
        self.begin_command_buffer(buffers[current])?;

        self.current_frame.set(current);

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

    pub(crate) fn submit(&self) -> Result<()> {
        let current = self.current_frame.get();
        let buffers = self.command_buffers.borrow();

        // end command buffer
        self.end_command_buffer(buffers[current])?;

        // submit
        let wait = [self.sync_acquire_image[current]];
        let signal = [self.sync_release_image[current]];
        let done = self.sync_queue_submit[current];
        let buffers = [buffers[current]];
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
        let current = self.current_frame.get();
        let wait = [self.sync_release_image[current]];
        let image = [swapchain.current() as u32];
        let handle = [swapchain.handle()];

        let info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait)
            .swapchains(&handle)
            .image_indices(&image);

        unsafe {
            self.swapchain_ext
                .queue_present(self.present_queue.1, &info)?;
        }

        Ok(())
    }

    pub(crate) fn command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffers.borrow()[self.current_frame.get()]
    }

    pub(crate) fn wait_for_idle(&self) -> Result<()> {
        for fen in &self.sync_queue_submit {
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
    ) -> Result<usize> {
        Ok(unsafe {
            self.swapchain_ext
                .acquire_next_image(handle, u64::max_value(), signal, Default::default())?
                .0 as usize
        })
    }

    pub(crate) const fn graphics_index(&self) -> u32 {
        self.graphics_queue.0
    }

    pub(crate) fn current_frame(&self) -> usize {
        self.current_frame.get()
    }

    pub(crate) fn stats(&self) -> Stats {
        self.stats.get()
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

    pub(crate) fn allocate_buffer(
        &self,
        info: &vk::BufferCreateInfo,
        access: BufferAccess,
    ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
        // create buffer handle
        let buffer = unsafe { self.handle.create_buffer(info, None)? };

        // allocate memory
        let requirements = unsafe { self.handle.get_buffer_memory_requirements(buffer) };
        let mem_type = self
            .find_memory_type(requirements.memory_type_bits, access.flag())
            .ok_or(ErrorType::Internal(ErrorKind::UnsupportedMemoryType))?;
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(mem_type);
        let memory = unsafe { self.handle.allocate_memory(&alloc_info, None)? };

        // bind memory
        unsafe { self.handle.bind_buffer_memory(buffer, memory, 0)? };

        Ok((buffer, memory))
    }

    pub(crate) fn free_buffer(&self, handle: vk::Buffer, memory: vk::DeviceMemory) {
        self.destroyed_buffers.borrow_mut()[self.current_frame.get()].push((handle, memory));
    }

    pub(crate) fn allocate_image(
        &self,
        info: &vk::ImageCreateInfo,
    ) -> Result<(vk::Image, vk::DeviceMemory)> {
        // create image handle
        let image = unsafe { self.handle.create_image(info, None)? };

        // allocate memory
        let requirements = unsafe { self.handle.get_image_memory_requirements(image) };
        let mem_type = self
            .find_memory_type(
                requirements.memory_type_bits,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .ok_or(ErrorType::Internal(ErrorKind::UnsupportedMemoryType))?;
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(mem_type);
        let memory = unsafe { self.handle.allocate_memory(&alloc_info, None)? };

        // bind memory
        unsafe {
            self.handle.bind_image_memory(image, memory, 0)?;
        }

        Ok((image, memory))
    }

    pub(crate) fn free_image(&self, handle: vk::Image, memory: vk::DeviceMemory) {
        unsafe {
            self.handle.destroy_image(handle, None);
            self.handle.free_memory(memory, None);
        }
    }

    pub(crate) fn create_image_view(
        &self,
        info: &vk::ImageViewCreateInfo,
    ) -> Result<vk::ImageView> {
        Ok(unsafe { self.handle.create_image_view(info, None)? })
    }

    pub(crate) fn destroy_image_view(&self, view: vk::ImageView) {
        unsafe {
            self.handle.destroy_image_view(view, None);
        }
    }

    pub(crate) fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        size: usize,
        fun: impl Fn(*mut c_void),
    ) -> Result<()> {
        let mem = unsafe {
            self.handle
                .map_memory(memory, 0, (size as u32).into(), vk::MemoryMapFlags::empty())?
        };
        fun(mem);
        unsafe {
            self.handle.unmap_memory(memory);
        }
        Ok(())
    }

    pub(crate) fn create_framebuffer(
        &self,
        info: &vk::FramebufferCreateInfo,
    ) -> Result<vk::Framebuffer> {
        Ok(unsafe { self.handle.create_framebuffer(info, None)? })
    }

    pub(crate) fn destroy_framebuffer(&self, handle: vk::Framebuffer) {
        unsafe {
            self.handle.destroy_framebuffer(handle, None);
        }
    }

    pub(crate) fn create_descriptor_set_layout(
        &self,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> Result<vk::DescriptorSetLayout> {
        let info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);
        Ok(unsafe { self.handle.create_descriptor_set_layout(&info, None)? })
    }

    pub(crate) fn destroy_descriptor_set_layout(&self, layout: vk::DescriptorSetLayout) {
        unsafe {
            self.handle.destroy_descriptor_set_layout(layout, None);
        }
    }

    pub(crate) fn create_descriptor_pool(
        &self,
        pool_sizes: &[vk::DescriptorPoolSize],
        max_sets: u32,
    ) -> Result<vk::DescriptorPool> {
        let info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(max_sets);
        Ok(unsafe { self.handle.create_descriptor_pool(&info, None)? })
    }

    pub(crate) fn destroy_descriptor_pool(&self, pool: vk::DescriptorPool) {
        unsafe {
            self.handle.destroy_descriptor_pool(pool, None);
        }
    }

    pub(crate) fn create_pipeline_layout(
        &self,
        info: &vk::PipelineLayoutCreateInfo,
    ) -> Result<vk::PipelineLayout> {
        Ok(unsafe { self.handle.create_pipeline_layout(info, None)? })
    }

    pub(crate) fn destroy_pipeline_layout(&self, handle: vk::PipelineLayout) {
        unsafe {
            self.handle.destroy_pipeline_layout(handle, None);
        }
    }

    pub(crate) fn allocate_descriptor_set(
        &self,
        layout: vk::DescriptorSetLayout,
        pool: vk::DescriptorPool,
    ) -> Result<vk::DescriptorSet> {
        let layouts = [layout];
        let info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(pool)
            .set_layouts(&layouts);

        Ok(unsafe { self.handle.allocate_descriptor_sets(&info)?[0] })
    }

    pub(crate) fn update_descriptor_sets(&self, writes: &[vk::WriteDescriptorSet]) {
        unsafe {
            self.handle.update_descriptor_sets(writes, &[]);
        }
    }

    pub(crate) fn create_render_pass(
        &self,
        info: &vk::RenderPassCreateInfo,
    ) -> Result<vk::RenderPass> {
        Ok(unsafe { self.handle.create_render_pass(info, None)? })
    }

    pub(crate) fn destroy_render_pass(&self, handle: vk::RenderPass) {
        unsafe {
            self.handle.destroy_render_pass(handle, None);
        }
    }

    pub(crate) fn create_sampler(&self, info: &vk::SamplerCreateInfo) -> Result<vk::Sampler> {
        Ok(unsafe { self.handle.create_sampler(info, None)? })
    }

    pub(crate) fn destroy_sampler(&self, handle: vk::Sampler) {
        unsafe {
            self.handle.destroy_sampler(handle, None);
        }
    }

    pub(crate) fn create_pipeline(
        &self,
        info: vk::GraphicsPipelineCreateInfo,
    ) -> Result<vk::Pipeline> {
        let infos = [info];
        Ok(unsafe {
            self.handle
                .create_graphics_pipelines(vk::PipelineCache::null(), &infos, None)
                .map_err(|err| err.1)?[0]
        })
    }

    pub(crate) fn destroy_pipeline(&self, handle: vk::Pipeline) {
        self.destroyed_pipelines.borrow_mut()[self.current_frame.get()].push(handle);
    }

    pub(crate) fn create_shader_module(&self, source: &[u8]) -> Result<vk::ShaderModule> {
        let words = util::read_spv(&mut Cursor::new(&source[..]))?;
        let info = vk::ShaderModuleCreateInfo::builder().code(&words).build();
        Ok(unsafe { self.handle.create_shader_module(&info, None)? })
    }

    pub(crate) fn destroy_shader_module(&self, module: vk::ShaderModule) {
        unsafe {
            self.handle.destroy_shader_module(module, None);
        }
    }

    pub(crate) fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.handle.destroy_command_pool(pool, None);
        }
    }

    pub(crate) fn do_commands(
        &self,
        mut fun: impl FnMut(vk::CommandBuffer) -> Result<()>,
    ) -> Result<()> {
        let pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(self.graphics_index());
        let pool = unsafe { self.handle.create_command_pool(&pool_info, None)? };

        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let buffer = unsafe { self.handle.allocate_command_buffers(&buffer_info)?[0] };

        self.begin_command_buffer(buffer)?;
        fun(buffer)?;
        self.end_command_buffer(buffer)?;
        self.submit_and_wait(buffer)?;
        self.destroy_command_pool(pool);

        Ok(())
    }

    pub(crate) fn free_command_buffer(
        &self,
        pool: vk::CommandPool,
        buffer: vk::CommandBuffer,
    ) -> Result<()> {
        let buffers = [buffer];
        unsafe {
            self.handle
                .reset_command_pool(pool, vk::CommandPoolResetFlags::RELEASE_RESOURCES)?;
            self.handle.free_command_buffers(pool, &buffers);
        }
        Ok(())
    }

    pub(crate) fn begin_command_buffer(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.handle.begin_command_buffer(buffer, &info)?;
        }
        Ok(())
    }

    pub(crate) fn end_command_buffer(&self, buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            self.handle.end_command_buffer(buffer)?;
        }
        Ok(())
    }

    pub(crate) fn cmd_begin_render_pass(
        &self,
        buffer: vk::CommandBuffer,
        framebuffer: &CoreFramebuffer,
        clear: [f32; 4],
    ) {
        // create clear values based on framebuffer image formats
        let clear_values = framebuffer
            .iter_images()
            .map(|image| {
                if image.has_depth_format() {
                    vk::ClearValue {
                        depth_stencil: vk::ClearDepthStencilValue {
                            depth: 1.0,
                            stencil: 0,
                        },
                    }
                } else {
                    vk::ClearValue {
                        color: vk::ClearColorValue { float32: clear },
                    }
                }
            })
            .collect::<Vec<_>>();

        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(framebuffer.render_pass())
            .framebuffer(framebuffer.handle())
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: framebuffer.width(),
                    height: framebuffer.height(),
                },
            })
            .clear_values(&clear_values);
        unsafe {
            self.handle
                .cmd_begin_render_pass(buffer, &info, vk::SubpassContents::INLINE);
        }
    }

    pub(crate) fn cmd_end_render_pass(&self, buffer: vk::CommandBuffer) {
        unsafe {
            self.handle.cmd_end_render_pass(buffer);
        }
    }

    pub(crate) fn cmd_bind_shader(&self, buffer: vk::CommandBuffer, shader: &CoreShader) {
        // update stats
        let mut stats = self.stats.get();
        let mut used = self.used_shaders.borrow_mut();
        if !used.contains(&shader.handle()) {
            used.insert(shader.handle());
            stats.shaders_used += 1;
        }
        stats.shader_rebinds += 1;
        self.stats.set(stats);

        // bind shader
        unsafe {
            self.handle
                .cmd_bind_pipeline(buffer, vk::PipelineBindPoint::GRAPHICS, shader.handle());
        }
    }

    pub(crate) fn cmd_bind_material(
        &self,
        buffer: vk::CommandBuffer,
        layout: &ShaderLayout,
        material: &CoreMaterial,
    ) {
        // update stats
        let mut stats = self.stats.get();
        let mut used = self.used_materials.borrow_mut();
        if !used.contains(&material.descriptor()) {
            used.insert(material.descriptor());
            stats.materials_used += 1;
        }
        stats.material_rebinds += 1;
        self.stats.set(stats);

        // bind material
        self.cmd_bind_descriptor(buffer, layout, material.descriptor());
    }

    pub(crate) fn cmd_bind_descriptor(
        &self,
        buffer: vk::CommandBuffer,
        layout: &ShaderLayout,
        descriptor: Descriptor,
    ) {
        let sets = [descriptor.1];
        unsafe {
            self.handle.cmd_bind_descriptor_sets(
                buffer,
                vk::PipelineBindPoint::GRAPHICS,
                layout.handle(),
                descriptor.0,
                &sets,
                &[],
            );
        }
    }

    pub(crate) fn cmd_bind_uniform(
        &self,
        buffer: vk::CommandBuffer,
        layout: &ShaderLayout,
        uniform: &impl Uniform,
    ) {
        self.cmd_bind_descriptor(buffer, layout, uniform.descriptor());
    }

    pub(crate) fn cmd_bind_mesh(&self, buffer: vk::CommandBuffer, mesh: &CoreMesh) {
        self.cmd_bind_index_buffer(buffer, mesh.index_buffer());
        self.cmd_bind_vertex_buffer(buffer, mesh.vertex_buffer());
    }

    fn cmd_bind_vertex_buffer(&self, buffer: vk::CommandBuffer, v_buffer: vk::Buffer) {
        let buffers = [v_buffer];
        let offsets = [0];
        unsafe {
            self.handle
                .cmd_bind_vertex_buffers(buffer, 0, &buffers, &offsets);
        }
    }

    fn cmd_bind_index_buffer(&self, buffer: vk::CommandBuffer, i_buffer: vk::Buffer) {
        unsafe {
            self.handle
                .cmd_bind_index_buffer(buffer, i_buffer, 0, vk::IndexType::UINT16);
        }
    }

    pub(crate) fn cmd_push_constants(
        &self,
        buffer: vk::CommandBuffer,
        layout: &ShaderLayout,
        constants: PushConstants,
    ) {
        unsafe {
            let data: &[u8] = slice::from_raw_parts(
                &constants as *const PushConstants as *const u8,
                mem::size_of::<PushConstants>(),
            );

            self.handle.cmd_push_constants(
                buffer,
                layout.handle(),
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                0,
                data,
            );
        }
    }

    pub(crate) fn cmd_draw(&self, buffer: vk::CommandBuffer, count: usize, offset: usize) {
        // update stats
        let mut stats = self.stats.get();
        stats.drawn_indices += count as u32;
        stats.draw_calls += 1;
        self.stats.set(stats);

        // draw
        unsafe {
            self.handle
                .cmd_draw_indexed(buffer, count as u32, 1, offset as u32, 0, 0);
        }
    }

    pub(crate) fn cmd_copy_buffer(
        &self,
        buffer: vk::CommandBuffer,
        src: vk::Buffer,
        dst: vk::Buffer,
        size: usize,
    ) {
        let region = [vk::BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size((size as u32).into())
            .build()];
        unsafe {
            self.handle.cmd_copy_buffer(buffer, src, dst, &region);
        }
    }

    pub(crate) fn cmd_copy_buffer_to_image(
        &self,
        buffer: vk::CommandBuffer,
        src: vk::Buffer,
        dst: vk::Image,
        region: vk::BufferImageCopy,
    ) {
        let regions = [region];
        unsafe {
            self.handle.cmd_copy_buffer_to_image(
                buffer,
                src,
                dst,
                ImageLayout::TransferDst.flag(),
                &regions,
            );
        }
    }

    pub(crate) fn cmd_set_view(&self, buffer: vk::CommandBuffer, width: u32, height: u32) {
        let viewport = [vk::Viewport {
            x: 0.0,
            y: height as f32,
            width: width as f32,
            height: -(height as f32),
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissor = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D { width, height },
        }];

        unsafe {
            self.handle.cmd_set_viewport(buffer, 0, &viewport);
            self.handle.cmd_set_scissor(buffer, 0, &scissor);
        }
    }

    pub(crate) fn cmd_set_line_width(&self, buffer: vk::CommandBuffer, width: f32) {
        unsafe {
            self.handle.cmd_set_line_width(buffer, width);
        }
    }

    pub(crate) fn cmd_blit_image_mip(
        &self,
        buffer: vk::CommandBuffer,
        image: &ImageMemory,
        src: u32,
        dst: u32,
    ) {
        fn mip_offsets(image: &ImageMemory, mip: u32) -> [vk::Offset3D; 2] {
            let orig_width = image.width();
            let orig_height = image.height();
            let scale = 2u32.pow(mip);
            let width = cmp::max(orig_width / scale, 1);
            let height = cmp::max(orig_height / scale, 1);
            [
                vk::Offset3D::default(),
                vk::Offset3D {
                    x: width as i32,
                    y: height as i32,
                    z: 1,
                },
            ]
        }

        let src_offsets = mip_offsets(image, src);
        let dst_offsets = mip_offsets(image, dst);

        let src_subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(image.all_aspects())
            .mip_level(src)
            .base_array_layer(0)
            .layer_count(image.layer_count())
            .build();
        let dst_subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(image.all_aspects())
            .mip_level(dst)
            .base_array_layer(0)
            .layer_count(image.layer_count())
            .build();

        let blit = [vk::ImageBlit::builder()
            .src_subresource(src_subresource)
            .dst_subresource(dst_subresource)
            .src_offsets(src_offsets)
            .dst_offsets(dst_offsets)
            .build()];

        unsafe {
            self.handle.cmd_blit_image(
                buffer,
                image.handle(),
                ImageLayout::TransferSrc.flag(),
                image.handle(),
                ImageLayout::TransferDst.flag(),
                &blit,
                vk::Filter::LINEAR,
            );
        }
    }

    pub(crate) fn cmd_blit_image(
        &self,
        buffer: vk::CommandBuffer,
        src: &ImageMemory,
        dst: &ImageMemory,
    ) {
        debug_assert!(
            src.width() == dst.width() && src.height() == dst.height(),
            "cannot blit image, sizes are different"
        );
        debug_assert!(
            src.all_aspects() == dst.all_aspects(),
            "cannot blit image, aspects are different"
        );
        debug_assert!(
            src.layer_count() == dst.layer_count(),
            "cannot blit image, layer counts are different"
        );

        let offsets = [
            vk::Offset3D::default(),
            vk::Offset3D {
                x: src.width() as i32,
                y: src.height() as i32,
                z: 1,
            },
        ];
        let subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(src.all_aspects())
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(src.layer_count())
            .build();

        let blit = [vk::ImageBlit::builder()
            .src_subresource(subresource)
            .dst_subresource(subresource)
            .src_offsets(offsets)
            .dst_offsets(offsets)
            .build()];

        unsafe {
            self.handle.cmd_blit_image(
                buffer,
                src.handle(),
                ImageLayout::TransferSrc.flag(),
                dst.handle(),
                ImageLayout::TransferDst.flag(),
                &blit,
                vk::Filter::LINEAR,
            );
        }
    }

    pub(crate) fn cmd_change_image_layout(
        &self,
        buffer: vk::CommandBuffer,
        image: &ImageMemory,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
        mips: Range<u32>,
        layers: Range<u32>,
    ) {
        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(image.all_aspects())
            .base_array_layer(layers.start)
            .base_mip_level(mips.start)
            .layer_count(layers.end - layers.start)
            .level_count(mips.end - mips.start)
            .build();
        let barrier = [vk::ImageMemoryBarrier::builder()
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(subresource)
            .image(image.handle())
            .old_layout(old_layout.flag())
            .new_layout(new_layout.flag())
            .src_access_mask(old_layout.access_flag())
            .dst_access_mask(new_layout.access_flag())
            .build()];

        unsafe {
            self.handle.cmd_pipeline_barrier(
                buffer,
                old_layout.stage_flag(),
                new_layout.stage_flag(),
                vk::DependencyFlags::default(),
                &[],
                &[],
                &barrier,
            );
        }
    }

    fn cleanup_resources(&self, frame: usize) {
        // cleanup pipelines
        let destroyed_pipelines = &mut self.destroyed_pipelines.borrow_mut()[frame];
        for p in destroyed_pipelines.iter() {
            unsafe {
                self.handle.destroy_pipeline(*p, None);
            }
        }
        destroyed_pipelines.clear();

        // cleanup buffers
        let destroyed_buffers = &mut self.destroyed_buffers.borrow_mut()[frame];
        for (b, m) in destroyed_buffers.iter() {
            unsafe {
                self.handle.destroy_buffer(*b, None);
                self.handle.free_memory(*m, None);
            }
        }
        destroyed_buffers.clear();

        // cleanup images
        let destroyed_images = &mut self.destroyed_images.borrow_mut()[frame];
        for (i, m) in destroyed_images.iter() {
            unsafe {
                self.handle.destroy_image(*i, None);
                self.handle.free_memory(*m, None);
            }
        }
        destroyed_images.clear();
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        for i in 0..IN_FLIGHT_FRAME_COUNT {
            self.cleanup_resources(i);
        }
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
            self.command_pools
                .iter()
                .for_each(|p| self.destroy_command_pool(*p));
            self.handle.destroy_device(None);
        }
    }
}
