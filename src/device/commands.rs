// Oliver Berzs
// https://github.com/oberzs/duku

use std::cell::Cell;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashSet;
use std::mem;
use std::ops::Range;
use std::ptr;
use std::slice;

use super::Stats;
use crate::color::Rgbf;
use crate::image::Canvas;
use crate::image::Image;
use crate::image::ImageLayout;
use crate::mesh::Mesh;
use crate::pipeline::Descriptor;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConstants;
use crate::pipeline::Uniforms;
use crate::vk;

pub(crate) struct Commands {
    buffer: Cell<vk::CommandBuffer>,
    pool: vk::CommandPool,

    stats: Cell<Stats>,
    used_materials: RefCell<HashSet<vk::DescriptorSet>>,
    used_shaders: RefCell<HashSet<vk::Pipeline>>,
}

impl Commands {
    pub(crate) fn new(device: vk::Device, queue_index: u32) -> Self {
        let pool_info = vk::CommandPoolCreateInfo {
            s_type: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::COMMAND_POOL_CREATE_TRANSIENT_BIT,
            queue_family_index: queue_index,
        };
        let mut pool = 0;
        unsafe {
            vk::check(vk::create_command_pool(
                device,
                &pool_info,
                ptr::null(),
                &mut pool,
            ));
        }

        let buffer_info = vk::CommandBufferAllocateInfo {
            s_type: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: pool,
            level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
            command_buffer_count: 1,
        };
        let mut buffer = 0;
        unsafe {
            vk::check(vk::allocate_command_buffers(
                device,
                &buffer_info,
                &mut buffer,
            ));
        }

        Self {
            stats: Cell::new(Stats::default()),
            used_materials: RefCell::new(HashSet::new()),
            used_shaders: RefCell::new(HashSet::new()),
            buffer: Cell::new(buffer),
            pool,
        }
    }

    pub(crate) fn free(&self, device: vk::Device) {
        let buffers = [self.buffer.get()];
        unsafe {
            vk::check(vk::reset_command_pool(
                device,
                self.pool,
                vk::COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT,
            ));
            vk::free_command_buffers(device, self.pool, 1, buffers.as_ptr());
        }
    }

    pub(crate) fn recreate(&self, device: vk::Device) {
        let buffer_info = vk::CommandBufferAllocateInfo {
            s_type: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.pool,
            level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
            command_buffer_count: 1,
        };
        let mut buffer = self.buffer.get();
        unsafe { vk::allocate_command_buffers(device, &buffer_info, &mut buffer) };
        self.buffer.set(buffer);
    }

    pub(crate) fn destroy(&self, device: vk::Device) {
        unsafe {
            vk::destroy_command_pool(device, self.pool, ptr::null());
        }
    }

    pub(crate) fn buffer(&self) -> vk::CommandBuffer {
        self.buffer.get()
    }

    pub(crate) fn reset_stats(&self) {
        self.stats.set(Stats::default());
        self.used_materials.borrow_mut().clear();
        self.used_shaders.borrow_mut().clear();
    }

    pub(crate) fn stats(&self) -> Stats {
        self.stats.get()
    }

    pub(crate) fn begin(&self) {
        let info = vk::CommandBufferBeginInfo {
            s_type: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            p_inheritance_info: ptr::null(),
        };
        unsafe {
            vk::check(vk::begin_command_buffer(self.buffer.get(), &info));
        }
    }

    pub(crate) fn end(&self) {
        unsafe {
            vk::check(vk::end_command_buffer(self.buffer.get()));
        }
    }

    pub(crate) fn begin_render_pass(&self, canvas: &Canvas, clear: Rgbf) {
        // create clear values based on canvas image formats
        let clear_values: Vec<_> = canvas
            .attachments()
            .map(|format| {
                if format.is_depth() {
                    vk::ClearValue {
                        depth_stencil: vk::ClearDepthStencilValue {
                            depth: 1.0,
                            stencil: 0,
                        },
                    }
                } else {
                    vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: clear.into(),
                        },
                    }
                }
            })
            .collect();

        let info = vk::RenderPassBeginInfo {
            s_type: vk::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            p_next: ptr::null(),
            render_pass: canvas.render_pass(),
            framebuffer: canvas.framebuffer(),
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: canvas.width,
                    height: canvas.height,
                },
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            vk::cmd_begin_render_pass(self.buffer.get(), &info, vk::SUBPASS_CONTENTS_INLINE);
        }
    }

    pub(crate) fn end_render_pass(&self) {
        unsafe {
            vk::cmd_end_render_pass(self.buffer.get());
        }
    }

    pub(crate) fn bind_shader(&self, shader: &Shader) {
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
            vk::cmd_bind_pipeline(
                self.buffer.get(),
                vk::PIPELINE_BIND_POINT_GRAPHICS,
                shader.handle(),
            );
        }
    }

    pub(crate) fn bind_material(&self, uniforms: &Uniforms, material: &Material) {
        // update stats
        let mut stats = self.stats.get();
        let mut used = self.used_materials.borrow_mut();
        if !used.contains(&material.descriptor().1) {
            used.insert(material.descriptor().1);
            stats.materials_used += 1;
        }
        stats.material_rebinds += 1;
        self.stats.set(stats);

        // bind material
        self.bind_descriptor(uniforms, material.descriptor());
    }

    pub(crate) fn bind_descriptor(&self, uniforms: &Uniforms, descriptor: Descriptor) {
        let sets = [descriptor.1];
        unsafe {
            vk::cmd_bind_descriptor_sets(
                self.buffer.get(),
                vk::PIPELINE_BIND_POINT_GRAPHICS,
                uniforms.pipeline_layout(),
                descriptor.0,
                1,
                sets.as_ptr(),
                0,
                ptr::null(),
            );
        }
    }

    pub(crate) fn bind_mesh(&self, mesh: &Mesh) {
        self.bind_index_buffer(mesh.index_buffer());
        self.bind_vertex_buffer(mesh.vertex_buffer());
    }

    fn bind_vertex_buffer(&self, v_buffer: vk::Buffer) {
        let buffers = [v_buffer];
        let offsets = [0];
        unsafe {
            vk::cmd_bind_vertex_buffers(
                self.buffer.get(),
                0,
                1,
                buffers.as_ptr(),
                offsets.as_ptr(),
            );
        }
    }

    fn bind_index_buffer(&self, i_buffer: vk::Buffer) {
        unsafe {
            vk::cmd_bind_index_buffer(self.buffer.get(), i_buffer, 0, vk::INDEX_TYPE_UINT32);
        }
    }

    pub(crate) fn push_constants(&self, uniforms: &Uniforms, constants: ShaderConstants) {
        unsafe {
            let data: &[u8] = slice::from_raw_parts(
                &constants as *const ShaderConstants as *const u8,
                mem::size_of::<ShaderConstants>(),
            );

            vk::cmd_push_constants(
                self.buffer.get(),
                uniforms.pipeline_layout(),
                vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
                0,
                data.len() as u32,
                data.as_ptr().cast(),
            );
        }
    }

    pub(crate) fn draw(&self, count: usize, offset: usize) {
        // update stats
        let mut stats = self.stats.get();
        stats.drawn_indices += count as u32;
        stats.draw_calls += 1;
        self.stats.set(stats);

        // draw
        unsafe {
            vk::cmd_draw_indexed(self.buffer.get(), count as u32, 1, offset as u32, 0, 0);
        }
    }

    pub(crate) fn copy_buffer_to_image(
        &self,
        src: vk::Buffer,
        dst: vk::Image,
        region: vk::BufferImageCopy,
    ) {
        let regions = [region];
        unsafe {
            vk::cmd_copy_buffer_to_image(
                self.buffer.get(),
                src,
                dst,
                ImageLayout::TransferDst.flag(),
                1,
                regions.as_ptr(),
            );
        }
    }

    pub(crate) fn set_view(&self, width: u32, height: u32) {
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
            vk::cmd_set_viewport(self.buffer.get(), 0, 1, viewport.as_ptr());
            vk::cmd_set_scissor(self.buffer.get(), 0, 1, scissor.as_ptr());
        }
    }

    pub(crate) fn blit_image_mip(&self, image: &Image, src: u32, dst: u32) {
        fn mip_offsets(image: &Image, mip: u32) -> [vk::Offset3D; 2] {
            let orig_width = image.width();
            let orig_height = image.height();
            let scale = 2u32.pow(mip);
            let width = cmp::max(orig_width / scale, 1);
            let height = cmp::max(orig_height / scale, 1);
            [
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: width as i32,
                    y: height as i32,
                    z: 1,
                },
            ]
        }

        let src_offsets = mip_offsets(image, src);
        let dst_offsets = mip_offsets(image, dst);

        let src_subresource = vk::ImageSubresourceLayers {
            aspect_mask: image.all_aspects(),
            mip_level: src,
            base_array_layer: 0,
            layer_count: image.layer_count(),
        };
        let dst_subresource = vk::ImageSubresourceLayers {
            aspect_mask: image.all_aspects(),
            mip_level: dst,
            base_array_layer: 0,
            layer_count: image.layer_count(),
        };
        let blit = [vk::ImageBlit {
            src_subresource,
            dst_subresource,
            src_offsets,
            dst_offsets,
        }];

        unsafe {
            vk::cmd_blit_image(
                self.buffer.get(),
                image.handle(),
                ImageLayout::TransferSrc.flag(),
                image.handle(),
                ImageLayout::TransferDst.flag(),
                1,
                blit.as_ptr(),
                vk::FILTER_LINEAR,
            );
        }
    }

    pub(crate) fn blit_image(&self, src: &Image, dst: &Image) {
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

        let blit = [vk::ImageBlit {
            src_subresource: vk::ImageSubresourceLayers {
                aspect_mask: src.all_aspects(),
                mip_level: 0,
                base_array_layer: 0,
                layer_count: src.layer_count(),
            },
            dst_subresource: vk::ImageSubresourceLayers {
                aspect_mask: src.all_aspects(),
                mip_level: 0,
                base_array_layer: 0,
                layer_count: src.layer_count(),
            },
            src_offsets: [
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: src.width() as i32,
                    y: src.height() as i32,
                    z: 1,
                },
            ],
            dst_offsets: [
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: dst.width() as i32,
                    y: dst.height() as i32,
                    z: 1,
                },
            ],
        }];

        unsafe {
            vk::cmd_blit_image(
                self.buffer.get(),
                src.handle(),
                ImageLayout::TransferSrc.flag(),
                dst.handle(),
                ImageLayout::TransferDst.flag(),
                1,
                blit.as_ptr(),
                vk::FILTER_LINEAR,
            );
        }
    }

    pub(crate) fn change_image_layout(
        &self,
        image: &Image,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
        mips: Range<u32>,
        layers: Range<u32>,
    ) {
        let subresource = vk::ImageSubresourceRange {
            aspect_mask: image.all_aspects(),
            base_mip_level: mips.start,
            level_count: mips.end - mips.start,
            base_array_layer: layers.start,
            layer_count: layers.end - layers.start,
        };
        let barrier = [vk::ImageMemoryBarrier {
            s_type: vk::STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            p_next: ptr::null(),
            src_access_mask: old_layout.access_flag(),
            dst_access_mask: new_layout.access_flag(),
            old_layout: old_layout.flag(),
            new_layout: new_layout.flag(),
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            image: image.handle(),
            subresource_range: subresource,
        }];
        unsafe {
            vk::cmd_pipeline_barrier(
                self.buffer.get(),
                old_layout.stage_flag(),
                new_layout.stage_flag(),
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                1,
                barrier.as_ptr(),
            );
        }
    }
}
