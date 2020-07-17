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
use crate::camera::Camera;
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
use crate::pipeline::ShadowMapUniform;
use crate::pipeline::Uniform;
use crate::pipeline::WorldData;

pub(crate) struct ForwardRenderer {
    shadow_frames: Vec<ShadowMapSet>,
    shadow_shader: Shader,
    blur_shader: Shader,
    start_time: Instant,
    device: Arc<Device>,
}

struct ShadowMapSet {
    main_framebuffers: [Framebuffer; 3],
    temp_framebuffers: [Framebuffer; 3],
    main_uniform: ShadowMapUniform,
    temp_uniform: ShadowMapUniform,
    matrices: [Matrix4; 4],
    cascades: [f32; 4],
    map_size: u32,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &ImageUniform,
    ) -> Result<Self> {
        profile_scope!("new");

        let shadow_frames = (0..IN_FLIGHT_FRAME_COUNT)
            .map(|_| ShadowMapSet::new(device, shader_layout, image_uniform, 1024))
            .collect::<Result<Vec<_>>>()?;

        let shadow_shader = Shader::new(
            device,
            &shadow_frames[0].main_framebuffers[0],
            shader_layout,
            include_bytes!("../../shaders/shadow.shader"),
            Default::default(),
        )?;
        let blur_shader = Shader::new(
            device,
            &shadow_frames[0].main_framebuffers[0],
            shader_layout,
            include_bytes!("../../shaders/shadow-blur.shader"),
            Default::default(),
        )?;

        Ok(Self {
            start_time: Instant::now(),
            device: Arc::clone(device),
            shadow_frames,
            shadow_shader,
            blur_shader,
        })
    }

    pub(crate) fn draw(
        &mut self,
        framebuffer: &Framebuffer,
        shader_layout: &ShaderLayout,
        target: Target,
    ) -> Result<RenderStats> {
        let cmd = self.device.command_buffer();
        self.device.cmd_set_line_width(cmd, 1.0);

        let current = self.device.current_frame();

        // reset current matrices and cascades
        self.shadow_frames[current].matrices = [Matrix4::identity(); 4];
        self.shadow_frames[current].cascades = [0.0; 4];

        // shadow mapping
        if target.do_shadow_mapping {
            self.shadow_pass(shader_layout, &target, &framebuffer.camera)?;
        }

        // bind current shadow map set
        self.device.cmd_bind_uniform(
            cmd,
            shader_layout,
            &self.shadow_frames[current].main_uniform,
        );

        // update world uniform
        framebuffer.world_uniform().update(WorldData {
            lights: target.lights(),
            world_matrix: framebuffer.camera.matrix(),
            camera_position: framebuffer.camera.transform.position,
            time: self.start_time.elapsed().as_secs_f32(),
            cascade_splits: self.shadow_frames[current].cascades,
            light_matrices: self.shadow_frames[current].matrices,
            variance_min: 0.00002,
            shadow_low: 0.1,
        })?;

        // do render pass
        self.device
            .cmd_begin_render_pass(cmd, framebuffer, target.clear.to_rgba_norm());
        self.device
            .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        self.device
            .cmd_bind_uniform(cmd, shader_layout, framebuffer.world_uniform());

        let mut render_stats = RenderStats::default();
        let mut unique_shaders = HashSet::new();
        let mut unique_materials = HashSet::new();

        for s_order in &target.orders_by_shader {
            s_order.shader.with(|s| {
                self.device.cmd_bind_shader(cmd, s);
                unique_shaders.insert(s.handle());
            });
            render_stats.shader_rebinds += 1;

            for m_order in &s_order.orders_by_material {
                m_order.material.with(|m| {
                    self.device.cmd_bind_material(cmd, shader_layout, m);
                    unique_materials.insert(m.uniform().descriptor());
                });
                render_stats.material_rebinds += 1;

                for order in &m_order.orders {
                    render_stats.drawn_indices += self.draw_order(order, shader_layout);
                    render_stats.draw_calls += 1;
                }
            }
        }

        self.device.cmd_end_render_pass(cmd);
        framebuffer.blit_to_texture(cmd);

        render_stats.time = self.start_time.elapsed().as_secs_f32();
        render_stats.shaders_used = unique_shaders.len() as u32;
        render_stats.materials_used = unique_materials.len() as u32;

        Ok(render_stats)
    }

    fn shadow_pass(
        &mut self,
        shader_layout: &ShaderLayout,
        target: &Target,
        view: &Camera,
    ) -> Result<()> {
        let cmd = self.device.command_buffer();
        let current = self.device.current_frame();

        // bind temp shadow map set so we can write to main one
        self.device.cmd_bind_uniform(
            cmd,
            shader_layout,
            &self.shadow_frames[current].main_uniform,
        );

        // render shadow map for each cascade
        let light_dir = target.main_light.coords.shrink();

        let mut prev_cs = 0.0;
        for (i, cs) in target.cascade_splits.iter().enumerate() {
            let map_size = self.shadow_frames[current].map_size;

            // get view frustum bounding sphere
            let bounds = view.bounding_sphere_for_split(prev_cs, *cs);
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
            shadow_origin *= map_size as f32 / 2.0;
            let rounded_origin = shadow_origin.round();
            let mut round_offset = rounded_origin - shadow_origin;
            round_offset *= 2.0 / map_size as f32;
            light_ortho_matrix.col_w.x += round_offset.x;
            light_ortho_matrix.col_w.y += round_offset.y;
            let light_matrix = light_ortho_matrix * light_view_matrix;

            // set uniform variables for normal render
            self.shadow_frames[current].matrices[i] = light_matrix;
            self.shadow_frames[current].cascades[i] = view.depth * cs;
            prev_cs = *cs;

            // update world uniform
            let framebuffer = &self.shadow_frames[current].main_framebuffers[i];
            framebuffer.world_uniform().update(WorldData {
                lights: [Default::default(); 4],
                world_matrix: light_matrix,
                camera_position: Vector3::default(),
                time: self.start_time.elapsed().as_secs_f32(),
                cascade_splits: [0.0; 4],
                light_matrices: [Matrix4::identity(); 4],
                variance_min: 0.0,
                shadow_low: 0.0,
            })?;

            // do render pass
            self.device
                .cmd_begin_render_pass(cmd, framebuffer, [1.0, 1.0, 1.0, 1.0]);
            self.device
                .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
            self.device
                .cmd_bind_uniform(cmd, shader_layout, framebuffer.world_uniform());
            self.device.cmd_bind_shader(cmd, &self.shadow_shader);

            for s_order in &target.orders_by_shader {
                for m_order in &s_order.orders_by_material {
                    for order in &m_order.orders {
                        if order.cast_shadows {
                            self.draw_order(order, shader_layout);
                        }
                    }
                }
            }
            self.device.cmd_end_render_pass(cmd);

            if target.shadow_softness > 0.0 {
                // do post processing - Gaussian blur
                // using push constants for shader properties for speed
                self.device.cmd_set_view(cmd, map_size, map_size);
                self.device.cmd_bind_shader(cmd, &self.blur_shader);
                let index_count = target.builtins.surface_mesh.with(|m| {
                    self.device.cmd_bind_mesh(cmd, m);
                    m.index_count()
                });
                let mut model_matrix = Matrix4::identity();
                let blur_amount = target.shadow_softness / map_size as f32;

                // pass #1 - horizontal
                self.device.cmd_bind_uniform(
                    cmd,
                    shader_layout,
                    &self.shadow_frames[current].main_uniform,
                );
                self.device.cmd_begin_render_pass(
                    cmd,
                    &self.shadow_frames[current].temp_framebuffers[i],
                    [1.0, 1.0, 1.0, 1.0],
                );
                model_matrix.col_x = Vector4::new(blur_amount, 0.0, 0.0, 0.0);
                self.device.cmd_push_constants(
                    cmd,
                    shader_layout,
                    PushConstants {
                        albedo_index: i as i32,
                        sampler_index: 0,
                        model_matrix,
                    },
                );
                self.device.cmd_draw(cmd, index_count);
                self.device.cmd_end_render_pass(cmd);

                // pass #2 - vertical
                self.device.cmd_bind_uniform(
                    cmd,
                    shader_layout,
                    &self.shadow_frames[current].temp_uniform,
                );
                self.device
                    .cmd_begin_render_pass(cmd, framebuffer, [1.0, 1.0, 1.0, 1.0]);
                model_matrix.col_x = Vector4::new(0.0, blur_amount, 0.0, 0.0);
                self.device.cmd_push_constants(
                    cmd,
                    shader_layout,
                    PushConstants {
                        albedo_index: i as i32,
                        sampler_index: 0,
                        model_matrix,
                    },
                );
                self.device.cmd_draw(cmd, index_count);
                self.device.cmd_end_render_pass(cmd);
            }
        }

        Ok(())
    }

    fn draw_order(&self, order: &Order, shader_layout: &ShaderLayout) -> u32 {
        let cmd = self.device.command_buffer();
        let albedo_index = match &order.albedo {
            Albedo::Texture(tex) => tex.with(|t| t.image_index()),
            Albedo::Framebuffer(fra) => fra.with(|f| f.texture_index()),
        };
        order.mesh.with(|m| {
            self.device.cmd_push_constants(
                cmd,
                shader_layout,
                PushConstants {
                    model_matrix: order.model,
                    sampler_index: order.sampler_index,
                    albedo_index,
                },
            );
            self.device.cmd_bind_mesh(cmd, m);
            self.device.cmd_draw(cmd, m.index_count());
            m.index_count()
        })
    }
}

impl ShadowMapSet {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &ImageUniform,
        map_size: u32,
    ) -> Result<Self> {
        let main_framebuffers = [
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
        ];
        let temp_framebuffers = [
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
        ];
        let main_uniform = ShadowMapUniform::new(
            shader_layout,
            [
                main_framebuffers[0].stored_view(),
                main_framebuffers[1].stored_view(),
                main_framebuffers[2].stored_view(),
            ],
        )?;
        let temp_uniform = ShadowMapUniform::new(
            shader_layout,
            [
                temp_framebuffers[0].stored_view(),
                temp_framebuffers[1].stored_view(),
                temp_framebuffers[2].stored_view(),
            ],
        )?;

        Ok(Self {
            matrices: [Matrix4::identity(); 4],
            cascades: [0.0; 4],
            main_framebuffers,
            temp_framebuffers,
            main_uniform,
            temp_uniform,
            map_size,
        })
    }

    fn shadow_framebuffer(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &ImageUniform,
        size: u32,
    ) -> Result<Framebuffer> {
        Framebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Float2],
                camera_type: CameraType::Orthographic,
                multisampled: false,
                depth: false,
                width: size,
                height: size,
            },
        )
    }
}
