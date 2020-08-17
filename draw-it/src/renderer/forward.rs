// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::rc::Rc;

use super::Albedo;
use super::Camera;
use super::CameraType;
use super::Order;
use super::OrdersByShader;
use super::RenderData;
use super::TextOrder;
use crate::device::Device;
use crate::device::IN_FLIGHT_FRAME_COUNT;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::image::FramebufferOptions;
use crate::image::Msaa;
use crate::image::WorldUpdateData;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::CoreShader;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShadowMapUniform;
use crate::resource::Builtins;
use crate::resource::Index;
use crate::resource::Storage;
use crate::stats::Stats;

pub(crate) struct ForwardRenderer {
    shadow_frames: [ShadowMapSet; IN_FLIGHT_FRAME_COUNT],
    shadow_shader: CoreShader,
    pcf: Pcf,
    device: Rc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub enum Pcf {
    X16,
    X4,
    Disabled,
}

struct ShadowMapSet {
    framebuffers: [CoreFramebuffer; 4],
    uniform: ShadowMapUniform,
    matrices: [Matrix4; 4],
    cascades: [f32; 4],
    map_size: u32,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        shadow_map_size: u32,
        pcf: Pcf,
    ) -> Result<Self> {
        let shadow_frames = [
            ShadowMapSet::new(device, shader_layout, image_uniform, shadow_map_size)?,
            ShadowMapSet::new(device, shader_layout, image_uniform, shadow_map_size)?,
        ];

        let shadow_shader = CoreShader::new(
            device,
            &shadow_frames[0].framebuffers[0],
            shader_layout,
            include_bytes!("../../shaders/shadow.shader"),
        )?;

        Ok(Self {
            device: Rc::clone(device),
            shadow_frames,
            shadow_shader,
            pcf,
        })
    }

    pub(crate) fn draw(
        &mut self,
        framebuffer: &Index,
        storage: &mut Storage,
        builtins: &Builtins,
        shader_layout: &ShaderLayout,
        data: RenderData,
        stats: &mut Stats,
    ) -> Result<()> {
        let cmd = self.device.command_buffer();
        self.device.cmd_set_line_width(cmd, data.line_width);

        let current = self.device.current_frame();

        // reset current matrices and cascades
        self.shadow_frames[current].matrices = [Matrix4::identity(); 4];
        self.shadow_frames[current].cascades = [0.0; 4];

        // shadow mapping
        if data.has_shadow_casters {
            let mut view = storage.framebuffers.get(framebuffer).camera.clone();
            view.depth = 50.0;
            self.shadow_pass(shader_layout, storage, &data, &view)?;
        }

        // bind current shadow map set
        self.device
            .cmd_bind_uniform(cmd, shader_layout, &self.shadow_frames[current].uniform);

        let pcf = match self.pcf {
            Pcf::Disabled => 2.0,
            Pcf::X4 => 0.0,
            Pcf::X16 => 1.0,
        };

        let lights = [
            data.lights[0].data(),
            data.lights[1].data(),
            data.lights[2].data(),
            data.lights[3].data(),
        ];

        // update world uniform
        let camera_position = storage
            .framebuffers
            .get(framebuffer)
            .camera
            .transform
            .position;
        let camera_depth = storage.framebuffers.get(framebuffer).camera.depth;
        let world_matrix = storage.framebuffers.get(framebuffer).camera.matrix();
        storage
            .framebuffers
            .get_mut(framebuffer)
            .world_buffer()
            .update_data(&[WorldUpdateData {
                cascade_splits: self.shadow_frames[current].cascades,
                light_matrices: self.shadow_frames[current].matrices,
                bias: data.bias,
                time: stats.time,
                camera_position,
                world_matrix,
                lights,
                pcf,
            }])?;

        // do render pass
        {
            let framebuffer = storage.framebuffers.get(framebuffer);
            self.device
                .cmd_begin_render_pass(cmd, framebuffer, data.clear.to_rgba_norm());
            self.device
                .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
            self.device
                .cmd_bind_descriptor(cmd, shader_layout, framebuffer.world_descriptor());
        }

        // let mut unique_shaders = HashSet::new();
        // let mut unique_materials = HashSet::new();

        // skybox rendering
        if data.skybox {
            self.skybox_pass(
                builtins,
                storage,
                shader_layout,
                camera_position,
                camera_depth,
                stats,
            );
        }

        // normal mesh rendering
        self.normal_pass(&data.orders_by_shader, storage, shader_layout, stats);

        // text rendering
        self.text_pass(&data.text_orders, storage, builtins, shader_layout, stats);

        // end rendering
        self.device.cmd_end_render_pass(cmd);
        storage
            .framebuffers
            .get_mut(framebuffer)
            .blit_to_texture(cmd);

        // stats.shaders_used = unique_shaders.len() as u32;
        // stats.materials_used = unique_materials.len() as u32;

        Ok(())
    }

    pub(crate) fn draw_core(
        &mut self,
        framebuffer: &mut CoreFramebuffer,
        storage: &mut Storage,
        builtins: &Builtins,
        shader_layout: &ShaderLayout,
        data: RenderData,
        stats: &mut Stats,
    ) -> Result<()> {
        let cmd = self.device.command_buffer();
        self.device.cmd_set_line_width(cmd, data.line_width);

        let current = self.device.current_frame();

        // reset current matrices and cascades
        self.shadow_frames[current].matrices = [Matrix4::identity(); 4];
        self.shadow_frames[current].cascades = [0.0; 4];

        // shadow mapping
        if data.has_shadow_casters {
            let mut view = framebuffer.camera.clone();
            view.depth = 50.0;
            self.shadow_pass(shader_layout, storage, &data, &view)?;
        }

        // bind current shadow map set
        self.device
            .cmd_bind_uniform(cmd, shader_layout, &self.shadow_frames[current].uniform);

        let pcf = match self.pcf {
            Pcf::Disabled => 2.0,
            Pcf::X4 => 0.0,
            Pcf::X16 => 1.0,
        };

        let lights = [
            data.lights[0].data(),
            data.lights[1].data(),
            data.lights[2].data(),
            data.lights[3].data(),
        ];

        // update world uniform
        let camera_position = framebuffer.camera.transform.position;
        let camera_depth = framebuffer.camera.depth;
        let world_matrix = framebuffer.camera.matrix();
        framebuffer.world_buffer().update_data(&[WorldUpdateData {
            cascade_splits: self.shadow_frames[current].cascades,
            light_matrices: self.shadow_frames[current].matrices,
            bias: data.bias,
            time: stats.time,
            camera_position,
            world_matrix,
            lights,
            pcf,
        }])?;

        // do render pass
        self.device
            .cmd_begin_render_pass(cmd, framebuffer, data.clear.to_rgba_norm());
        self.device
            .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        self.device
            .cmd_bind_descriptor(cmd, shader_layout, framebuffer.world_descriptor());

        // let mut unique_shaders = HashSet::new();
        // let mut unique_materials = HashSet::new();

        // skybox rendering
        if data.skybox {
            self.skybox_pass(
                builtins,
                storage,
                shader_layout,
                camera_position,
                camera_depth,
                stats,
            );
        }

        // normal mesh rendering
        self.normal_pass(&data.orders_by_shader, storage, shader_layout, stats);

        // text rendering
        self.text_pass(&data.text_orders, storage, builtins, shader_layout, stats);

        // end rendering
        self.device.cmd_end_render_pass(cmd);
        framebuffer.blit_to_texture(cmd);

        // stats.shaders_used = unique_shaders.len() as u32;
        // stats.materials_used = unique_materials.len() as u32;

        Ok(())
    }

    fn normal_pass(
        &self,
        orders_by_shader: &[OrdersByShader],
        storage: &Storage,
        shader_layout: &ShaderLayout,
        stats: &mut Stats,
    ) {
        let cmd = self.device.command_buffer();

        for s_order in orders_by_shader {
            // bind shader
            let shader = storage.shaders.get(&s_order.shader);
            self.device.cmd_bind_shader(cmd, shader);
            // unique_shaders.insert(s.handle());
            stats.shader_rebinds += 1;

            for m_order in &s_order.orders_by_material {
                // bind material
                let material = storage.materials.get(&m_order.material);
                self.device.cmd_bind_material(cmd, shader_layout, material);
                // unique_materials.insert(material.descriptor());
                stats.material_rebinds += 1;

                for order in &m_order.orders {
                    stats.drawn_indices += self.draw_order(storage, shader_layout, order);
                    stats.draw_calls += 1;
                }
            }
        }
    }

    fn skybox_pass(
        &self,
        builtins: &Builtins,
        storage: &Storage,
        shader_layout: &ShaderLayout,
        camera_position: Vector3,
        camera_depth: f32,
        stats: &mut Stats,
    ) {
        let cmd = self.device.command_buffer();

        let shader = storage.shaders.get(&builtins.skybox_shader.index);
        self.device.cmd_bind_shader(cmd, shader);
        // unique_shaders.insert(s.handle());
        stats.shader_rebinds += 1;

        let mesh = storage.meshes.get(&builtins.cube_mesh.index);
        self.device.cmd_bind_mesh(cmd, mesh);

        let model_matrix = (Transform {
            position: camera_position,
            scale: Vector3::uniform(camera_depth * 2.0 - 0.1),
            ..Default::default()
        })
        .as_matrix();
        self.device.cmd_push_constants(
            cmd,
            shader_layout,
            PushConstants {
                sampler_index: 0,
                albedo_index: 0,
                model_matrix,
            },
        );
        self.device.cmd_draw(cmd, mesh.index_count(), 0);

        stats.drawn_indices += mesh.index_count() as u32;
        stats.draw_calls += 1;
    }

    fn text_pass(
        &self,
        orders: &[TextOrder],
        storage: &Storage,
        builtins: &Builtins,
        shader_layout: &ShaderLayout,
        stats: &mut Stats,
    ) {
        let cmd = self.device.command_buffer();

        for order in orders {
            let font = storage.fonts.get(&order.font);

            let font_size = order.size;

            let shader_index = if font.is_bitmap(font_size) {
                builtins.bitmap_font_shader.index.clone()
            } else {
                builtins.sdf_font_shader.index.clone()
            };
            let sampler_index = if font.is_bitmap(font_size) { 7 } else { 1 };

            // bind shader
            let shader = storage.shaders.get(&shader_index);
            self.device.cmd_bind_shader(cmd, shader);
            // unique_shaders.insert(s.handle());
            stats.shader_rebinds += 1;

            // bind material
            let material = storage.materials.get(&order.material);
            self.device.cmd_bind_material(cmd, shader_layout, material);
            // unique_materials.insert(material.descriptor());
            stats.material_rebinds += 1;

            let albedo_index = font.texture(font_size).image_index();
            let mesh = font.mesh(font_size);
            let margin = font.margin(font_size);
            self.device.cmd_bind_mesh(cmd, mesh);

            let mut transform = order.transform;
            let start_x = transform.position.x;
            transform.scale *= font_size as f32;
            transform.position.x -= margin * font_size as f32;

            for c in order.text.chars() {
                // handle whitespace
                if c == ' ' {
                    transform.position.x += transform.scale.x / 3.0;
                    continue;
                }
                if c == '\n' {
                    transform.position.x = start_x;
                    transform.position.y -= transform.scale.y;
                    continue;
                }

                self.device.cmd_push_constants(
                    cmd,
                    shader_layout,
                    PushConstants {
                        model_matrix: transform.as_matrix(),
                        sampler_index,
                        albedo_index,
                    },
                );

                let data = font.char_data(font_size, c);
                self.device.cmd_draw(cmd, 6, data.offset);

                stats.drawn_indices += 6;
                stats.draw_calls += 1;

                transform.position.x += data.advance * transform.scale.x;
            }
        }
    }

    fn shadow_pass(
        &mut self,
        shader_layout: &ShaderLayout,
        storage: &Storage,
        data: &RenderData,
        view: &Camera,
    ) -> Result<()> {
        let light_dir = match data.lights.iter().find(|l| l.shadows) {
            Some(light) => light.coords,
            // if there is no light with shadows,
            // don't do shadow pass
            None => return Ok(()),
        };

        let cmd = self.device.command_buffer();
        let current = self.device.current_frame();

        // bind temp shadow map set so we can write to main one
        self.device
            .cmd_bind_uniform(cmd, shader_layout, &self.shadow_frames[current].uniform);

        // render shadow map for each cascade
        let mut prev_cs = 0.0;
        for (i, cs) in data.cascade_splits.iter().enumerate() {
            let map_size = self.shadow_frames[current].map_size;

            // get view frustum bounding sphere
            let bounds = view.bounding_sphere_for_split(prev_cs, *cs);
            let diameter = bounds.radius * 2.0;
            let up = if light_dir.y < 1.0 && light_dir.y > -1.0 {
                Vector3::UP
            } else {
                Vector3::FORWARD
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
            let framebuffer = &mut self.shadow_frames[current].framebuffers[i];
            framebuffer.world_buffer().update_data(&[WorldUpdateData {
                light_matrices: [Matrix4::identity(); 4],
                camera_position: Vector3::default(),
                lights: [Default::default(); 4],
                world_matrix: light_matrix,
                cascade_splits: [0.0; 4],
                bias: 0.0,
                time: 0.0,
                pcf: 0.0,
            }])?;

            // do render pass
            self.device
                .cmd_begin_render_pass(cmd, framebuffer, [1.0, 1.0, 1.0, 1.0]);
            self.device
                .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
            self.device
                .cmd_bind_descriptor(cmd, shader_layout, framebuffer.world_descriptor());
            self.device.cmd_bind_shader(cmd, &self.shadow_shader);

            for s_order in &data.orders_by_shader {
                for m_order in &s_order.orders_by_material {
                    for order in &m_order.orders {
                        if order.cast_shadows {
                            self.draw_order(storage, shader_layout, order);
                        }
                    }
                }
            }
            self.device.cmd_end_render_pass(cmd);
        }

        Ok(())
    }

    fn draw_order(&self, storage: &Storage, shader_layout: &ShaderLayout, order: &Order) -> u32 {
        let cmd = self.device.command_buffer();
        let albedo_index = match &order.albedo {
            Albedo::Texture(tex) => storage.textures.get(tex).image_index(),
            Albedo::Framebuffer(fra) => storage.framebuffers.get(fra).texture_index(),
        };
        let mesh = storage.meshes.get(&order.mesh);

        self.device.cmd_push_constants(
            cmd,
            shader_layout,
            PushConstants {
                model_matrix: order.model,
                sampler_index: order.sampler_index,
                albedo_index,
            },
        );
        self.device.cmd_bind_mesh(cmd, mesh);
        self.device.cmd_draw(cmd, mesh.index_count(), 0);
        mesh.index_count() as u32
    }
}

impl ShadowMapSet {
    pub(crate) fn new(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        map_size: u32,
    ) -> Result<Self> {
        let framebuffers = [
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size)?,
        ];
        let uniform = ShadowMapUniform::new(
            shader_layout,
            [
                framebuffers[0].stored_view(),
                framebuffers[1].stored_view(),
                framebuffers[2].stored_view(),
                framebuffers[3].stored_view(),
            ],
        )?;

        Ok(Self {
            matrices: [Matrix4::identity(); 4],
            cascades: [0.0; 4],
            framebuffers,
            uniform,
            map_size,
        })
    }

    fn shadow_framebuffer(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        size: u32,
    ) -> Result<CoreFramebuffer> {
        CoreFramebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[],
                camera_type: CameraType::Orthographic,
                msaa: Msaa::Disabled,
                depth: true,
                width: size,
                height: size,
            },
        )
    }
}
