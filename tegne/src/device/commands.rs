use ash::vk;
use std::sync::Arc;

use super::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::ImageLayout;
use crate::image::ImageMemory;
use crate::pipeline::Descriptor;
use crate::pipeline::PushConstants;
use crate::pipeline::RenderPass;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;

pub(crate) struct Commands {
    buffer: vk::CommandBuffer,
    pool: vk::CommandPool,
    device: Arc<Device>,
}

pub(crate) struct LayoutChangeOptions {
    pub(crate) old_layout: ImageLayout,
    pub(crate) new_layout: ImageLayout,
    pub(crate) base_mip: u32,
    pub(crate) mip_count: u32,
}

impl Commands {
    pub(crate) fn new(device: &Arc<Device>) -> Result<Self> {
        let pool = device.create_command_pool()?;
        let buffer = create_buffer(device, pool)?;

        Ok(Self {
            buffer,
            pool,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn reset(&mut self) -> Result<()> {
        self.device.free_command_buffer(self.pool, self.buffer)?;
        self.buffer = create_buffer(&self.device, self.pool)?;
        Ok(())
    }

    pub(crate) fn begin(&self) -> Result<()> {
        self.device.begin_command_buffer(self.buffer)
    }

    pub(crate) fn end(&self) -> Result<vk::CommandBuffer> {
        self.device.end_command_buffer(self.buffer)?;
        Ok(self.buffer)
    }

    pub(crate) fn begin_render_pass(
        &self,
        framebuffer: &Framebuffer,
        render_pass: &RenderPass,
        clear: [f32; 4],
    ) {
        self.device
            .cmd_begin_render_pass(self.buffer, framebuffer, render_pass, clear);
    }

    pub(crate) fn end_render_pass(&self) {
        self.device.cmd_end_render_pass(self.buffer);
    }

    pub(crate) fn bind_shader(&self, shader: &Shader) {
        self.device.cmd_bind_shader(self.buffer, shader);
    }

    pub(crate) fn bind_descriptor(&self, descriptor: Descriptor, layout: &ShaderLayout) {
        self.device
            .cmd_bind_descriptor(self.buffer, descriptor, layout);
    }

    pub(crate) fn bind_vertex_buffer(&self, buffer: vk::Buffer) {
        self.device.cmd_bind_vertex_buffer(self.buffer, buffer);
    }

    pub(crate) fn bind_index_buffer(&self, buffer: vk::Buffer) {
        self.device.cmd_bind_index_buffer(self.buffer, buffer);
    }

    pub(crate) fn set_push_constant(&self, constants: PushConstants, layout: &ShaderLayout) {
        self.device
            .cmd_push_constants(self.buffer, constants, layout);
    }

    pub(crate) fn draw(&self, count: u32) {
        self.device.cmd_draw(self.buffer, count);
    }

    pub(crate) fn _copy_buffer(&self, src: vk::Buffer, dst: vk::Buffer, size: usize) {
        self.device.cmd_copy_buffer(self.buffer, src, dst, size);
    }

    pub(crate) fn set_view(&self, width: u32, height: u32) {
        self.device.cmd_set_view(self.buffer, width, height);
    }

    pub(crate) fn set_line_width(&self, width: f32) {
        self.device.cmd_set_line_width(self.buffer, width);
    }

    pub(crate) fn copy_buffer_to_image(
        &self,
        buffer: vk::Buffer,
        image: vk::Image,
        region: vk::BufferImageCopy,
    ) {
        self.device
            .cmd_copy_buffer_to_image(self.buffer, buffer, image, region);
    }

    pub(crate) fn blit_image(
        &self,
        src: vk::Image,
        dst: vk::Image,
        blit: vk::ImageBlit,
        filter: vk::Filter,
    ) {
        self.device
            .cmd_blit_image(self.buffer, src, dst, blit, filter);
    }

    pub(crate) fn change_image_layout(&self, image: &ImageMemory, options: LayoutChangeOptions) {
        self.device
            .cmd_change_image_layout(self.buffer, image, options);
    }
}

impl Drop for Commands {
    fn drop(&mut self) {
        self.device.destroy_command_pool(self.pool);
    }
}

impl Default for LayoutChangeOptions {
    fn default() -> Self {
        Self {
            old_layout: ImageLayout::Undefined,
            new_layout: ImageLayout::Undefined,
            base_mip: 0,
            mip_count: 1,
        }
    }
}

fn create_buffer(device: &Arc<Device>, pool: vk::CommandPool) -> Result<vk::CommandBuffer> {
    let info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    device.allocate_command_buffer(&info)
}
