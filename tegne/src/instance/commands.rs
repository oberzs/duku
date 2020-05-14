use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferCopy;
use ash::vk::BufferImageCopy;
use ash::vk::ClearColorValue;
use ash::vk::ClearDepthStencilValue;
use ash::vk::ClearValue;
use ash::vk::CommandBuffer;
use ash::vk::CommandBufferAllocateInfo;
use ash::vk::CommandBufferBeginInfo;
use ash::vk::CommandBufferLevel;
use ash::vk::CommandBufferUsageFlags;
use ash::vk::CommandPool;
use ash::vk::CommandPoolCreateFlags;
use ash::vk::CommandPoolCreateInfo;
use ash::vk::CommandPoolResetFlags;
use ash::vk::DependencyFlags;
use ash::vk::Extent2D;
use ash::vk::Filter;
use ash::vk::Image as VkImage;
use ash::vk::ImageBlit;
use ash::vk::ImageMemoryBarrier;
use ash::vk::IndexType;
use ash::vk::Offset2D;
use ash::vk::Pipeline;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineLayout;
use ash::vk::PipelineStageFlags;
use ash::vk::Rect2D;
use ash::vk::RenderPassBeginInfo;
use ash::vk::ShaderStageFlags;
use ash::vk::SubpassContents;
use ash::vk::Viewport;
use ash::Device as LogicalDevice;
use std::mem;
use std::slice;
use std::sync::Arc;
use std::sync::Weak;

use super::Device;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::images::Framebuffer;
use crate::images::Image;
use crate::images::ImageLayout;
use crate::images::LayoutChange;
use crate::shaders::Descriptor;
use crate::shaders::PushConstants;
use crate::shaders::RenderPass;
use crate::utils::OrError;

pub(crate) struct Commands {
    buffer: CommandBuffer,
    pool: CommandPool,
    device: Weak<Device>,
    dropped: bool,
}

impl Commands {
    pub(crate) fn new(device: &Arc<Device>) -> Self {
        let pool_info = CommandPoolCreateInfo::builder()
            .flags(CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(device.properties().graphics_index)
            .build();

        let pool = unsafe {
            device
                .logical()
                .create_command_pool(&pool_info, None)
                .or_error("cannot create command pool")
        };

        let buffer = create_buffer(device, pool);

        Self {
            buffer,
            pool,
            device: Arc::downgrade(device),
            dropped: false,
        }
    }

    pub(crate) fn reset(&mut self) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device
                .logical()
                .reset_command_pool(self.pool, CommandPoolResetFlags::empty())
                .or_error("cannot reset command pool");
        }
        self.buffer = create_buffer(&device, self.pool);
        Ok(())
    }

    pub(crate) fn begin(&self) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let begin_info = CommandBufferBeginInfo::builder();
        unsafe {
            device
                .logical()
                .begin_command_buffer(self.buffer, &begin_info)
                .or_error("cannot begin command buffer");
        }
        Ok(())
    }

    pub(crate) fn begin_one_time(&self) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let begin_info =
            CommandBufferBeginInfo::builder().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            device
                .logical()
                .begin_command_buffer(self.buffer, &begin_info)
                .or_error("cannot begin command buffer");
        }
        Ok(())
    }

    pub(crate) fn end(&self) -> Result<CommandBuffer> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device
                .logical()
                .end_command_buffer(self.buffer)
                .or_error("cannot end command buffer");
        }
        Ok(self.buffer)
    }

    pub(crate) fn begin_render_pass(
        &self,
        framebuffer: &Framebuffer,
        render_pass: &RenderPass,
        clear: [f32; 4],
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let clear_values = framebuffer
            .iter_attachments()
            .map(|image| {
                if image.is_depth_format() {
                    ClearValue {
                        depth_stencil: ClearDepthStencilValue {
                            depth: 1.0,
                            stencil: 0,
                        },
                    }
                } else {
                    ClearValue {
                        color: ClearColorValue { float32: clear },
                    }
                }
            })
            .collect::<Vec<_>>();

        let info = RenderPassBeginInfo::builder()
            .render_pass(render_pass.vk())
            .framebuffer(framebuffer.vk())
            .render_area(Rect2D {
                offset: Offset2D { x: 0, y: 0 },
                extent: Extent2D {
                    width: framebuffer.width(),
                    height: framebuffer.height(),
                },
            })
            .clear_values(&clear_values);
        unsafe {
            device
                .logical()
                .cmd_begin_render_pass(self.buffer, &info, SubpassContents::INLINE);
        }
        Ok(())
    }

    pub(crate) fn end_render_pass(&self) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device.logical().cmd_end_render_pass(self.buffer);
        }
        Ok(())
    }

    pub(crate) fn bind_pipeline(&self, pipeline: Pipeline) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device
                .logical()
                .cmd_bind_pipeline(self.buffer, PipelineBindPoint::GRAPHICS, pipeline);
        }
        Ok(())
    }

    pub(crate) fn bind_descriptor(
        &self,
        descriptor: Descriptor,
        layout: PipelineLayout,
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let sets = [descriptor.1];
        unsafe {
            device.logical().cmd_bind_descriptor_sets(
                self.buffer,
                PipelineBindPoint::GRAPHICS,
                layout,
                descriptor.0,
                &sets,
                &[],
            );
        }
        Ok(())
    }

    pub(crate) fn bind_vertex_buffer(&self, buffer: Buffer) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let buffers = [buffer];
        let offsets = [0];
        unsafe {
            device
                .logical()
                .cmd_bind_vertex_buffers(self.buffer, 0, &buffers, &offsets);
        }
        Ok(())
    }

    pub(crate) fn bind_index_buffer(&self, buffer: Buffer) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device
                .logical()
                .cmd_bind_index_buffer(self.buffer, buffer, 0, IndexType::UINT32);
        }
        Ok(())
    }

    pub(crate) fn set_push_constant(
        &self,
        constants: PushConstants,
        layout: PipelineLayout,
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            let constant_ptr: *const PushConstants = &constants;
            let data: &[u8] =
                slice::from_raw_parts(constant_ptr as *const u8, mem::size_of::<PushConstants>());

            device.logical().cmd_push_constants(
                self.buffer,
                layout,
                ShaderStageFlags::VERTEX | ShaderStageFlags::FRAGMENT,
                0,
                data,
            );
        }
        Ok(())
    }

    pub(crate) fn draw(&self, count: u32) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device
                .logical()
                .cmd_draw_indexed(self.buffer, count, 1, 0, 0, 0);
        }
        Ok(())
    }

    pub(crate) fn copy_buffer(&self, src: Buffer, dst: Buffer, size: usize) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let region = BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size((size as u32).into())
            .build();
        let regions = [region];
        unsafe {
            device
                .logical()
                .cmd_copy_buffer(self.buffer, src, dst, &regions);
        }
        Ok(())
    }

    pub(crate) fn set_view(&self, width: u32, height: u32) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissor = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D { width, height },
        };
        let viewports = [viewport];
        let scissors = [scissor];

        unsafe {
            device
                .logical()
                .cmd_set_viewport(self.buffer, 0, &viewports);
            device.logical().cmd_set_scissor(self.buffer, 0, &scissors);
        }
        Ok(())
    }

    pub(crate) fn set_line_width(&self, width: f32) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        unsafe {
            device.logical().cmd_set_line_width(self.buffer, width);
        }
        Ok(())
    }

    pub(crate) fn set_pipeline_barrier(
        &self,
        barrier: ImageMemoryBarrier,
        src_stage: PipelineStageFlags,
        dst_stage: PipelineStageFlags,
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let barriers = [barrier];
        unsafe {
            device.logical().cmd_pipeline_barrier(
                self.buffer,
                src_stage,
                dst_stage,
                DependencyFlags::default(),
                &[],
                &[],
                &barriers,
            );
        }
        Ok(())
    }

    pub(crate) fn copy_buffer_to_image(
        &self,
        buffer: Buffer,
        image: VkImage,
        region: BufferImageCopy,
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let regions = [region];
        unsafe {
            device.logical().cmd_copy_buffer_to_image(
                self.buffer,
                buffer,
                image,
                ImageLayout::TransferDst.flag(),
                &regions,
            );
        }
        Ok(())
    }

    pub(crate) fn blit_image(
        &self,
        src: VkImage,
        dst: VkImage,
        blit: ImageBlit,
        filter: Filter,
    ) -> Result<()> {
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let regions = [blit];
        unsafe {
            device.logical().cmd_blit_image(
                self.buffer,
                src,
                ImageLayout::TransferSrc.flag(),
                dst,
                ImageLayout::TransferDst.flag(),
                &regions,
                filter,
            );
        }
        Ok(())
    }

    pub(crate) fn change_image_layout<'a>(&'a self, image: &'a Image) -> LayoutChange<'a> {
        LayoutChange::new(self, image)
    }

    pub(crate) fn manual_drop(&mut self, logical: &LogicalDevice) {
        unsafe {
            logical.destroy_command_pool(self.pool, None);
        }
        self.dropped = true;
    }
}

impl Drop for Commands {
    fn drop(&mut self) {
        if !self.dropped {
            let device = self
                .device
                .upgrade()
                .ok_or(ErrorKind::DeviceDropped)
                .unwrap();
            unsafe {
                device.logical().destroy_command_pool(self.pool, None);
            }
        }
    }
}

fn create_buffer(device: &Arc<Device>, pool: CommandPool) -> CommandBuffer {
    let info = CommandBufferAllocateInfo::builder()
        .command_pool(pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    unsafe {
        device
            .logical()
            .allocate_command_buffers(&info)
            .or_error("cannot allocate command buffers")[0]
    }
}
