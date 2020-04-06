use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferCopy;
use ash::vk::BufferImageCopy;
// use ash::vk::ClearColorValue;
// use ash::vk::ClearDepthStencilValue;
// use ash::vk::ClearValue;
use ash::vk::CommandBuffer;
use ash::vk::CommandBufferAllocateInfo;
use ash::vk::CommandBufferBeginInfo;
use ash::vk::CommandBufferLevel;
use ash::vk::CommandBufferUsageFlags;
use ash::vk::CommandPool;
use ash::vk::DependencyFlags;
use ash::vk::DescriptorSet;
use ash::vk::Extent2D;
use ash::vk::Filter;
use ash::vk::Image;
use ash::vk::ImageBlit;
use ash::vk::ImageLayout;
use ash::vk::ImageMemoryBarrier;
use ash::vk::IndexType;
use ash::vk::Offset2D;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineLayout;
use ash::vk::PipelineStageFlags;
use ash::vk::Rect2D;
// use ash::vk::RenderPass;
// use ash::vk::RenderPassBeginInfo;
// use ash::vk::ShaderStageFlags;
// use ash::vk::SubpassContents;
use ash::vk::Viewport;
// use std::mem;
use std::rc::Rc;
// use std::slice;
use ash::vk::CommandPoolCreateFlags;
use ash::vk::CommandPoolCreateInfo;
use ash::vk::CommandPoolResetFlags;

use crate::tegne::Device;
use crate::utils::OrError;

pub struct CommandRecorder {
    buffer: CommandBuffer,
    pool: CommandPool,
    device: Rc<Device>,
}

impl CommandRecorder {
    pub fn new(device: &Rc<Device>, queue: u32) -> Self {
        let pool_info = CommandPoolCreateInfo::builder()
            .flags(CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(queue)
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
            device: Rc::clone(device),
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            self.device
                .logical()
                .reset_command_pool(self.pool, CommandPoolResetFlags::empty())
                .or_error("cannot reset command pool");
        }
        self.buffer = create_buffer(&self.device, self.pool);
    }

    pub fn begin(&self) {
        let begin_info = CommandBufferBeginInfo::builder();
        unsafe {
            self.device
                .logical()
                .begin_command_buffer(self.buffer, &begin_info)
                .or_error("cannot begin command buffer");
        }
    }

    pub fn begin_one_time(&self) {
        let begin_info =
            CommandBufferBeginInfo::builder().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.device
                .logical()
                .begin_command_buffer(self.buffer, &begin_info)
                .or_error("cannot begin command buffer");
        }
    }

    pub fn end(&self) -> CommandBuffer {
        unsafe {
            self.device
                .logical()
                .end_command_buffer(self.buffer)
                .or_error("cannot end command buffer");
        }
        self.buffer
    }

    // pub fn begin_render_pass(&self, framebuffer: &Framebuffer, pass: RenderPass, clear: [f32; 4]) {
    //     let clear_values = [
    //         ClearValue {
    //             depth_stencil: ClearDepthStencilValue {
    //                 depth: 1.0,
    //                 stencil: 0,
    //             },
    //         },
    //         ClearValue {
    //             color: ClearColorValue { float32: clear },
    //         },
    //         ClearValue {
    //             color: ClearColorValue { float32: clear },
    //         },
    //     ];
    //     let info = RenderPassBeginInfo::builder()
    //         .render_pass(pass)
    //         .framebuffer(framebuffer.vk())
    //         .render_area(Rect2D {
    //             offset: Offset2D { x: 0, y: 0 },
    //             extent: Extent2D {
    //                 width: framebuffer.width(),
    //                 height: framebuffer.height(),
    //             },
    //         })
    //         .clear_values(&clear_values);
    //     unsafe {
    //         self.device.logical().cmd_begin_render_pass(
    //             self.buffer,
    //             &info,
    //             SubpassContents::INLINE,
    //         );
    //     }
    // }

    pub fn end_render_pass(&self) {
        unsafe {
            self.device.logical().cmd_end_render_pass(self.buffer);
        }
    }

    // pub fn bind_pipeline(&self, shader: &Shader) {
    //     unsafe {
    //         self.device.logical().cmd_bind_pipeline(
    //             self.buffer,
    //             PipelineBindPoint::GRAPHICS,
    //             shader.pipeline(),
    //         );
    //     }
    // }

    pub fn bind_descriptor(&self, set: (u32, DescriptorSet), layout: PipelineLayout) {
        let sets = [set.1];
        unsafe {
            self.device.logical().cmd_bind_descriptor_sets(
                self.buffer,
                PipelineBindPoint::GRAPHICS,
                layout,
                set.0,
                &sets,
                &[],
            );
        }
    }

    pub fn bind_vertex_buffer(&self, buffer: Buffer) {
        let buffers = [buffer];
        let offsets = [0];
        unsafe {
            self.device
                .logical()
                .cmd_bind_vertex_buffers(self.buffer, 0, &buffers, &offsets);
        }
    }

    pub fn bind_index_buffer(&self, buffer: Buffer) {
        unsafe {
            self.device
                .logical()
                .cmd_bind_index_buffer(self.buffer, buffer, 0, IndexType::UINT32);
        }
    }

    // pub fn set_push_constant(&self, constants: PushConstants, layout: PipelineLayout) {
    //     unsafe {
    //         let data: &[u8] = slice::from_raw_parts(
    //             &constants as *const PushConstants as *const u8,
    //             mem::size_of::<PushConstants>(),
    //         );

    //         self.device.logical().cmd_push_constants(
    //             self.buffer,
    //             layout,
    //             ShaderStageFlags::VERTEX | ShaderStageFlags::FRAGMENT,
    //             0,
    //             data,
    //         );
    //     }
    // }

    pub fn draw(&self, count: u32) {
        unsafe {
            self.device
                .logical()
                .cmd_draw_indexed(self.buffer, count, 1, 0, 0, 0);
        }
    }

    pub fn copy_buffer(&self, src: Buffer, dst: Buffer, size: usize) {
        let region = BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size((size as u32).into())
            .build();
        let regions = [region];
        unsafe {
            self.device
                .logical()
                .cmd_copy_buffer(self.buffer, src, dst, &regions);
        }
    }

    pub fn set_view(&self, width: u32, height: u32) {
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
            self.device
                .logical()
                .cmd_set_viewport(self.buffer, 0, &viewports);
            self.device
                .logical()
                .cmd_set_scissor(self.buffer, 0, &scissors);
        }
    }

    pub fn set_line_width(&self, width: f32) {
        unsafe {
            self.device.logical().cmd_set_line_width(self.buffer, width);
        }
    }

    pub fn set_pipeline_barrier(
        &self,
        barrier: ImageMemoryBarrier,
        src_stage: PipelineStageFlags,
        dst_stage: PipelineStageFlags,
    ) {
        let barriers = [barrier];
        unsafe {
            self.device.logical().cmd_pipeline_barrier(
                self.buffer,
                src_stage,
                dst_stage,
                DependencyFlags::default(),
                &[],
                &[],
                &barriers,
            );
        }
    }

    pub fn copy_buffer_to_image(&self, buffer: Buffer, image: Image, region: BufferImageCopy) {
        let regions = [region];
        unsafe {
            self.device.logical().cmd_copy_buffer_to_image(
                self.buffer,
                buffer,
                image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions,
            );
        }
    }

    pub fn blit_image(&self, src: Image, dst: Image, blit: ImageBlit) {
        let regions = [blit];
        unsafe {
            self.device.logical().cmd_blit_image(
                self.buffer,
                src,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions,
                Filter::LINEAR,
            );
        }
    }
}

impl Drop for CommandRecorder {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_command_pool(self.pool, None);
        }
    }
}

fn create_buffer(device: &Rc<Device>, pool: CommandPool) -> CommandBuffer {
    let info = CommandBufferAllocateInfo::builder()
        .command_pool(pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    unsafe {
        *device
            .logical()
            .allocate_command_buffers(&info)
            .or_error("cannot allocate command buffers")
            .get(0)
            .or_error("no command buffers allocated")
    }
}
