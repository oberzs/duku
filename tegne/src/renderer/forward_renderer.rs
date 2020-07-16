// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use super::Albedo;
use super::Order;
use super::RenderStats;
use super::Target;
use crate::camera::CameraType;
use crate::device::Device;
use crate::device::IN_FLIGHT_FRAME_COUNT;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::ImageFormat;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::ShadowMapUniform;
use crate::pipeline::Uniform;
use crate::pipeline::WorldData;

const CASCADE_COUNT: usize = 3;

pub(crate) struct ForwardRenderer {
    shadow_framebuffers: Vec<Vec<Framebuffer>>,
    shadow_uniforms: Vec<ShadowMapUniform>,
    shadow_shader: Shader,
    shadow_map_size: u32,
    start_time: Instant,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &ImageUniform,
    ) -> Result<Self> {
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
                    image_uniform,
                    FramebufferOptions {
                        attachment_formats: &[ImageFormat::Float2],
                        camera_type: CameraType::Orthographic,
                        multisampled: false,
                        depth: false,
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
            include_bytes!("../../shaders/shadow.shader"),
            ShaderOptions {
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
        device.cmd_set_line_width(cmd, 1.0);

        let light_dir = target.main_light.coords.shrink();

        let mut light_matrices = [Matrix4::identity(); 4];
        let mut cascade_splits = [0.0; 4];

        // shadow mapping
        if target.do_shadow_mapping {
            // bind other random shadow map set
            device.cmd_bind_uniform(
                cmd,
                shader_layout,
                &self.shadow_uniforms[(device.current_frame() + 1) % IN_FLIGHT_FRAME_COUNT],
            );

            // render shadow map for each cascade
            let mut prev_cs = 0.0;
            for (i, cs) in target.cascade_splits.iter().enumerate() {
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

                device.cmd_begin_render_pass(cmd, shadow_framebuffer, target.clear.to_rgba_norm());
                device.cmd_set_view(cmd, shadow_framebuffer.width(), shadow_framebuffer.height());
                device.cmd_bind_uniform(cmd, shader_layout, shadow_framebuffer.world_uniform());
                device.cmd_bind_shader(cmd, &self.shadow_shader);

                for s_order in &target.orders_by_shader {
                    for m_order in &s_order.orders_by_material {
                        for order in &m_order.orders {
                            if order.cast_shadows {
                                self.draw_order(device, order, shader_layout);
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
        device.cmd_bind_uniform(
            cmd,
            shader_layout,
            &self.shadow_uniforms[device.current_frame()],
        );

        // normal render
        // update world uniform
        framebuffer.world_uniform().update(WorldData {
            lights: target.lights(),
            world_matrix: framebuffer.camera.matrix(),
            camera_position: framebuffer.camera.transform.position,
            time: self.start_time.elapsed().as_secs_f32(),
            bias: target.bias,
            cascade_splits,
            light_matrices,
        })?;

        device.cmd_begin_render_pass(cmd, framebuffer, target.clear.to_rgba_norm());
        device.cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        device.cmd_bind_uniform(cmd, shader_layout, framebuffer.world_uniform());

        let mut render_stats = RenderStats::default();
        let mut unique_shaders = HashSet::new();
        let mut unique_materials = HashSet::new();

        for s_order in &target.orders_by_shader {
            s_order.shader.with(|s| {
                device.cmd_bind_shader(cmd, s);
                unique_shaders.insert(s.handle());
            });
            render_stats.shader_rebinds += 1;

            for m_order in &s_order.orders_by_material {
                m_order.material.with(|m| {
                    device.cmd_bind_material(cmd, shader_layout, m);
                    unique_materials.insert(m.uniform().descriptor());
                });
                render_stats.material_rebinds += 1;

                for order in &m_order.orders {
                    render_stats.drawn_indices += self.draw_order(device, order, shader_layout);
                    render_stats.draw_calls += 1;
                }
            }
        }

        device.cmd_end_render_pass(cmd);
        framebuffer.blit_to_texture(cmd);

        render_stats.time = self.start_time.elapsed().as_secs_f32();
        render_stats.shaders_used = unique_shaders.len() as u32;
        render_stats.materials_used = unique_materials.len() as u32;

        Ok(render_stats)
    }

    fn draw_order(&self, device: &Device, order: &Order, shader_layout: &ShaderLayout) -> u32 {
        let cmd = device.command_buffer();
        let albedo_index = match &order.albedo {
            Albedo::Texture(tex) => tex.with(|t| t.image_index()),
            Albedo::Framebuffer(fra) => fra.with(|f| f.texture_index()),
        };
        order.mesh.with(|m| {
            device.cmd_push_constants(
                cmd,
                shader_layout,
                PushConstants {
                    model_matrix: order.model,
                    sampler_index: order.sampler_index,
                    albedo_index,
                },
            );
            device.cmd_bind_mesh(cmd, m);
            device.cmd_draw(cmd, m.index_count());
            m.index_count()
        })
    }
}
