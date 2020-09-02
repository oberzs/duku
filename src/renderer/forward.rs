// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::rc::Rc;
use std::time::Instant;

use super::Camera;
use super::Order;
use super::OrdersByShader;
use super::Target;
use super::TextOrder;
use crate::device::Device;
use crate::device::FRAMES_IN_FLIGHT;
use crate::image::CoreFramebuffer;
use crate::image::FramebufferOptions;
use crate::image::Msaa;
use crate::image::WorldData;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::CoreShader;
use crate::pipeline::Descriptor;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::ShaderLayout;
use crate::storage::Storage;

pub(crate) struct ForwardRenderer {
    shadow_frames: [ShadowMapSet; FRAMES_IN_FLIGHT],
    shadow_shader: CoreShader,
    start_time: Instant,
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
    descriptor: Descriptor,
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
    ) -> Self {
        let shadow_frames = [
            ShadowMapSet::new(device, shader_layout, image_uniform, shadow_map_size),
            ShadowMapSet::new(device, shader_layout, image_uniform, shadow_map_size),
        ];

        let shadow_shader = CoreShader::from_spirv_bytes(
            device,
            &shadow_frames[0].framebuffers[0],
            shader_layout,
            include_bytes!("../../shaders/shadow.spirv"),
        )
        .expect("bad shader");

        Self {
            device: Rc::clone(device),
            start_time: Instant::now(),
            shadow_frames,
            shadow_shader,
            pcf,
        }
    }

    pub(crate) fn draw(
        &mut self,
        framebuffer: &CoreFramebuffer,
        camera: &Camera,
        storage: &Storage,
        shader_layout: &ShaderLayout,
        target: Target<'_>,
    ) {
        let current = self.device.current_frame();

        // reset current matrices and cascades
        self.shadow_frames[current].matrices = [Matrix4::identity(); 4];
        self.shadow_frames[current].cascades = [0.0; 4];

        // shadow mapping
        if target.has_shadow_casters {
            let mut view = camera.clone();
            view.depth = 50.0;
            self.shadow_pass(shader_layout, storage, &target, &view);
        }

        let cmd = self.device.commands();
        cmd.set_line_width(target.line_width);

        // bind current shadow map set
        cmd.bind_descriptor(shader_layout, self.shadow_frames[current].descriptor);

        let pcf = match self.pcf {
            Pcf::Disabled => 2.0,
            Pcf::X4 => 0.0,
            Pcf::X16 => 1.0,
        };

        let lights = [
            target.lights[0].data(),
            target.lights[1].data(),
            target.lights[2].data(),
            target.lights[3].data(),
        ];

        // update world uniform
        framebuffer.world_buffer().update_data(&[WorldData {
            cascade_splits: self.shadow_frames[current].cascades,
            light_matrices: self.shadow_frames[current].matrices,
            bias: target.bias,
            time: self.start_time.elapsed().as_secs_f32(),
            camera_position: camera.transform.position,
            world_matrix: camera.matrix(),
            lights,
            pcf,
        }]);

        // do render pass
        cmd.begin_render_pass(framebuffer, target.clear.to_rgba_norm());
        cmd.set_view(framebuffer.width(), framebuffer.height());
        cmd.bind_descriptor(shader_layout, framebuffer.world_descriptor());

        // skybox rendering
        if target.skybox {
            self.skybox_pass(&target, storage, shader_layout, camera);
        }

        // normal mesh rendering
        self.normal_pass(&target.orders_by_shader, storage, shader_layout);

        // text rendering
        self.text_pass(&target.text_orders, storage, &target, shader_layout);

        // end rendering
        cmd.end_render_pass();
        framebuffer.blit_to_texture(cmd);
    }

    fn normal_pass(
        &self,
        orders_by_shader: &[OrdersByShader],
        storage: &Storage,
        shader_layout: &ShaderLayout,
    ) {
        let cmd = self.device.commands();

        for s_order in orders_by_shader {
            // bind shader
            let shader = storage.shaders.get(&s_order.shader);
            cmd.bind_shader(shader);

            for m_order in &s_order.orders_by_material {
                // bind material
                let material = storage.materials.get(&m_order.material);
                cmd.bind_material(shader_layout, material);

                for order in &m_order.orders {
                    self.draw_order(storage, shader_layout, order);
                }
            }
        }
    }

    fn skybox_pass(
        &self,
        target: &Target<'_>,
        storage: &Storage,
        shader_layout: &ShaderLayout,
        camera: &Camera,
    ) {
        let cmd = self.device.commands();

        let shader = storage.shaders.get(&target.builtins.skybox_shader.index);
        cmd.bind_shader(shader);

        let mesh = storage.meshes.get(&target.builtins.cube_mesh.index);
        cmd.bind_mesh(mesh);

        let model_matrix = (Transform {
            position: camera.transform.position,
            scale: Vector3::uniform(camera.depth * 2.0 - 0.1),
            ..Default::default()
        })
        .as_matrix();
        cmd.push_constants(
            shader_layout,
            PushConstants {
                sampler_index: 0,
                model_matrix,
            },
        );
        cmd.draw(mesh.index_count(), 0);
    }

    fn text_pass(
        &self,
        orders: &[TextOrder],
        storage: &Storage,
        target: &Target<'_>,
        shader_layout: &ShaderLayout,
    ) {
        let cmd = self.device.commands();

        // bind shader
        let shader = storage.shaders.get(&target.builtins.font_shader.index);
        cmd.bind_shader(shader);

        for order in orders {
            let font = storage.fonts.get(&order.font);

            // bind material
            let material = storage.materials.get(&order.material);
            cmd.bind_material(shader_layout, material);

            // bind mesh
            cmd.bind_mesh(font.mesh());

            let mut transform = order.transform;
            let start_x = transform.position.x;
            transform.scale *= order.size as f32;

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

                let data = font.char_data(c);

                let mut local_transform = transform;
                local_transform.position.x += data.x_offset * transform.scale.x;
                local_transform.position.y -= data.y_offset * transform.scale.y;

                cmd.push_constants(
                    shader_layout,
                    PushConstants {
                        model_matrix: local_transform.as_matrix(),
                        sampler_index: 7,
                    },
                );

                cmd.draw(6, data.index_offset);

                transform.position.x += data.advance * transform.scale.x;
            }
        }
    }

    fn shadow_pass(
        &mut self,
        shader_layout: &ShaderLayout,
        storage: &Storage,
        target: &Target<'_>,
        view: &Camera,
    ) {
        let light_dir = match target.lights.iter().find(|l| l.shadows) {
            Some(light) => light.coords,
            // if there is no light with shadows,
            // don't do shadow pass
            None => return,
        };

        let cmd = self.device.commands();
        let current = self.device.current_frame();

        // bind temp shadow map set so we can write to main one
        cmd.bind_descriptor(shader_layout, self.shadow_frames[current].descriptor);

        // render shadow map for each cascade
        let mut prev_cs = 0.0;
        for (i, cs) in target.cascade_splits.iter().enumerate() {
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
            framebuffer.world_buffer().update_data(&[WorldData {
                light_matrices: [Matrix4::identity(); 4],
                camera_position: Vector3::default(),
                lights: [Default::default(); 4],
                world_matrix: light_matrix,
                cascade_splits: [0.0; 4],
                bias: 0.0,
                time: 0.0,
                pcf: 0.0,
            }]);

            // do render pass
            cmd.begin_render_pass(framebuffer, [1.0, 1.0, 1.0, 1.0]);
            cmd.set_view(framebuffer.width(), framebuffer.height());
            cmd.bind_descriptor(shader_layout, framebuffer.world_descriptor());
            cmd.bind_shader(&self.shadow_shader);

            for s_order in &target.orders_by_shader {
                for m_order in &s_order.orders_by_material {
                    for order in &m_order.orders {
                        if order.cast_shadows {
                            self.draw_order(storage, shader_layout, order);
                        }
                    }
                }
            }
            cmd.end_render_pass();
        }
    }

    fn draw_order(&self, storage: &Storage, shader_layout: &ShaderLayout, order: &Order) {
        let cmd = self.device.commands();
        let mesh = storage.meshes.get(&order.mesh);

        cmd.push_constants(
            shader_layout,
            PushConstants {
                model_matrix: order.model,
                sampler_index: order.sampler_index,
            },
        );
        cmd.bind_mesh(mesh);
        cmd.draw(mesh.index_count(), 0);
    }
}

impl ShadowMapSet {
    pub(crate) fn new(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        map_size: u32,
    ) -> Self {
        let framebuffers = [
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size),
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size),
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size),
            Self::shadow_framebuffer(device, shader_layout, image_uniform, map_size),
        ];
        let descriptor = shader_layout.shadow_map_set([
            framebuffers[0].stored_view(),
            framebuffers[1].stored_view(),
            framebuffers[2].stored_view(),
            framebuffers[3].stored_view(),
        ]);

        Self {
            matrices: [Matrix4::identity(); 4],
            cascades: [0.0; 4],
            framebuffers,
            descriptor,
            map_size,
        }
    }

    fn shadow_framebuffer(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        size: u32,
    ) -> CoreFramebuffer {
        CoreFramebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[],
                msaa: Msaa::Disabled,
                depth: true,
                width: size,
                height: size,
            },
        )
    }
}
