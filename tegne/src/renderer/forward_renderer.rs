// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::sync::Arc;
use std::time::Instant;

use super::Order;
use super::RenderStats;
use super::Target;
use crate::camera::CameraType;
use crate::device::Device;
use crate::device::IN_FLIGHT_FRAME_COUNT;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::AttachmentType;
use crate::pipeline::Material;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::ShadowMapUniform;
use crate::pipeline::WorldData;
use crate::resource::Ref;

const CASCADE_COUNT: usize = 3;

pub(crate) struct ForwardRenderer {
    shadow_framebuffers: Vec<Vec<Framebuffer>>,
    shadow_uniforms: Vec<ShadowMapUniform>,
    shadow_shader: Shader,
    shadow_map_size: u32,
    start_time: Instant,
}

impl ForwardRenderer {
    pub(crate) fn new(device: &Arc<Device>, shader_layout: &ShaderLayout) -> Result<Self> {
        profile_scope!("new");

        let shadow_map_size = 2048;

        let mut shadow_framebuffers = vec![];
        let mut shadow_uniforms = vec![];
        for frame in 0..IN_FLIGHT_FRAME_COUNT {
            shadow_framebuffers.push(vec![]);
            for _ in 0..CASCADE_COUNT {
                shadow_framebuffers[frame].push(Framebuffer::new(
                    device,
                    shader_layout,
                    FramebufferOptions {
                        attachment_types: &[AttachmentType::Depth],
                        camera_type: CameraType::Orthographic,
                        multisampled: false,
                        width: shadow_map_size,
                        height: shadow_map_size,
                    },
                )?);
            }

            shadow_uniforms.push(ShadowMapUniform::new(
                shader_layout,
                [
                    shadow_framebuffers[frame][0].stored_view(),
                    shadow_framebuffers[frame][1].stored_view(),
                    shadow_framebuffers[frame][2].stored_view(),
                ],
            )?);
        }

        let shadow_shader = Shader::new(
            device,
            &shadow_framebuffers[0][0],
            shader_layout,
            include_bytes!("../../assets/shaders/shadow.shader"),
            ShaderOptions {
                front_cull: false,
                ..Default::default()
            },
        )?;

        Ok(Self {
            start_time: Instant::now(),
            shadow_framebuffers,
            shadow_uniforms,
            shadow_shader,
            shadow_map_size,
        })
    }

    pub(crate) fn draw(
        &self,
        device: &Device,
        framebuffer: &Framebuffer,
        shader_layout: &ShaderLayout,
        target: Target,
    ) -> Result<RenderStats> {
        let cmd = device.command_buffer();

        let light_dir = target.main_light().coords.shrink();

        let mut light_matrices = [Matrix4::identity(); 4];
        let mut cascade_splits = [0.0; 4];

        // shadow mapping
        if target.do_shadow_mapping() {
            // bind other random shadow map set
            device.cmd_bind_descriptor(
                cmd,
                self.shadow_uniforms[(device.current_frame() + 1) % IN_FLIGHT_FRAME_COUNT]
                    .descriptor(),
                shader_layout,
            );

            // render shadow map for each cascade
            let mut prev_cs = 0.0;
            for (i, cs) in target.cascade_splits().iter().enumerate() {
                let shadow_framebuffer = &self.shadow_framebuffers[device.current_frame()][i];

                // get view frustum bounding sphere
                let bounds = framebuffer.camera.bounding_sphere_for_split(prev_cs, *cs);
                let diameter = bounds.radius * 2.0;

                let up = if light_dir.y < 1.0 && light_dir.y > -1.0 {
                    Vector3::up()
                } else {
                    Vector3::forward()
                };

                let light_position = bounds.center - light_dir * bounds.radius;
                let light_view_matrix = Matrix4::look_rotation(bounds.center - light_position, up)
                    * Matrix4::translation(-light_position);
                let mut light_ortho_matrix =
                    Matrix4::orthographic_center(diameter, diameter, 0.0, diameter);

                // stabilize shadow map by using texel units
                let shadow_matrix = light_ortho_matrix * light_view_matrix;
                let mut shadow_origin = Vector4::new(0.0, 0.0, 0.0, 1.0);
                shadow_origin = shadow_matrix * shadow_origin;
                shadow_origin *= self.shadow_map_size as f32 / 2.0;

                let rounded_origin = shadow_origin.round();
                let mut round_offset = rounded_origin - shadow_origin;
                round_offset *= 2.0 / self.shadow_map_size as f32;

                light_ortho_matrix.col_w.x += round_offset.x;
                light_ortho_matrix.col_w.y += round_offset.y;

                let light_matrix = light_ortho_matrix * light_view_matrix;

                shadow_framebuffer.world_uniform().update(WorldData {
                    lights: [Default::default(); 4],
                    world_matrix: light_matrix,
                    camera_position: Vector3::default(),
                    time: self.start_time.elapsed().as_secs_f32(),
                    cascade_splits: [0.0; 4],
                    light_matrices: [Matrix4::identity(); 4],
                    bias: 0.0,
                })?;

                device.cmd_begin_render_pass(cmd, shadow_framebuffer, target.clear());
                self.setup_pass(device, shadow_framebuffer);
                self.bind_world(device, shadow_framebuffer, shader_layout);
                device.cmd_bind_shader(cmd, &self.shadow_shader);
                for s_order in target.orders_by_shader() {
                    for m_order in s_order.orders_by_material() {
                        self.bind_material(device, m_order.material(), shader_layout)?;
                        for order in m_order.orders() {
                            if order.cast_shadows {
                                self.draw_order(device, order, shader_layout, &mut 0)?;
                            }
                        }
                    }
                }
                device.cmd_end_render_pass(cmd);

                // set uniform variables for normal render
                light_matrices[i] = light_matrix;
                cascade_splits[i] = framebuffer.camera.depth * cs;
                prev_cs = *cs;
            }
        }

        // bind current shadow map set
        device.cmd_bind_descriptor(
            cmd,
            self.shadow_uniforms[device.current_frame()].descriptor(),
            shader_layout,
        );

        // normal render
        // update world uniform
        framebuffer.world_uniform().update(WorldData {
            lights: target.lights(),
            world_matrix: framebuffer.camera.matrix(),
            camera_position: framebuffer.camera.transform.position,
            time: self.start_time.elapsed().as_secs_f32(),
            bias: target.bias(),
            cascade_splits,
            light_matrices,
        })?;

        device.cmd_begin_render_pass(cmd, framebuffer, target.clear());
        self.setup_pass(device, framebuffer);
        self.bind_world(device, framebuffer, shader_layout);

        let mut drawn_indices = 0;
        let mut shaders_used = 0;
        let mut materials_used = 0;
        let mut draw_calls = 0;

        for s_order in target.orders_by_shader() {
            self.bind_shader(device, s_order.shader());
            shaders_used += 1;
            for m_order in s_order.orders_by_material() {
                self.bind_material(device, m_order.material(), shader_layout)?;
                materials_used += 1;
                for order in m_order.orders() {
                    self.draw_order(device, order, shader_layout, &mut drawn_indices)?;
                    draw_calls += 1;
                }
            }
        }

        device.cmd_end_render_pass(cmd);

        Ok(RenderStats {
            time: self.start_time.elapsed().as_secs_f32(),
            drawn_triangles: drawn_indices / 3,
            drawn_indices,
            shaders_used,
            materials_used,
            draw_calls,
        })
    }

    fn setup_pass(&self, device: &Device, framebuffer: &Framebuffer) {
        let cmd = device.command_buffer();
        device.cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        device.cmd_set_line_width(cmd, 1.0);
    }

    fn bind_world(&self, device: &Device, framebuffer: &Framebuffer, shader_layout: &ShaderLayout) {
        let cmd = device.command_buffer();
        device.cmd_bind_descriptor(cmd, framebuffer.world_uniform().descriptor(), shader_layout);
    }

    fn bind_shader(&self, device: &Device, shader: &Ref<Shader>) {
        let cmd = device.command_buffer();
        shader.with(|s| device.cmd_bind_shader(cmd, s));
    }

    fn bind_material(
        &self,
        device: &Device,
        material: &Ref<Material>,
        shader_layout: &ShaderLayout,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let descriptor = material.with(|m| m.descriptor())?;
        device.cmd_bind_descriptor(cmd, descriptor, shader_layout);
        Ok(())
    }

    fn draw_order(
        &self,
        device: &Device,
        order: Order,
        shader_layout: &ShaderLayout,
        drawn_indices: &mut u32,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let albedo_index = order.albedo.with(|t| t.image_index());
        let (vb, ib, n) = order
            .mesh
            .with(|m| (m.vertex_buffer(), m.index_buffer(), m.index_count()));

        if let Some(framebuffer) = order.framebuffer {
            let frame_descriptor = framebuffer.with(|f| f.descriptor());
            device.cmd_bind_descriptor(cmd, frame_descriptor, shader_layout);
        }

        device.cmd_push_constants(
            cmd,
            PushConstants {
                model_matrix: order.model,
                sampler_index: order.sampler_index,
                albedo_index,
            },
            shader_layout,
        );
        device.cmd_bind_vertex_buffer(cmd, vb?);
        device.cmd_bind_index_buffer(cmd, ib?);
        device.cmd_draw(cmd, n);

        *drawn_indices += n;

        Ok(())
    }
}
