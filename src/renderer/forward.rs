// Oliver Berzs
// https://github.com/oberzs/duku

use std::cmp::Ordering;
use std::time::Instant;

use super::Camera;
use super::CharOrder;
use super::Clear;
use super::LightType;
use super::LineOrder;
use super::Pcf;
use super::ShaderOrder;
use super::ShadowRenderer;
use super::ShadowSplitParams;
use super::Target;
use super::TriOrder;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Rgbf;
use crate::device::Commands;
use crate::device::Device;
use crate::image::Canvas;
use crate::math::Mat4;
use crate::math::Quat;
use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::pipeline::Descriptor;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderWorld;
use crate::pipeline::Uniforms;
use crate::resources::Builtins;

pub(crate) struct ForwardRenderer {
    target_resources: Vec<TargetResources>,
    shadow_renderer: ShadowRenderer,
    start_time: Instant,
    required_targets: usize,
    target_index: usize,
}

struct TargetResources {
    world_descriptor: Descriptor,
    world_buffer: Buffer<ShaderWorld>,
    text_mesh: Mesh,
    line_mesh: Mesh,
    shape_mesh: Mesh,
}

impl ForwardRenderer {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms, shadow_map_size: u32) -> Self {
        let shadow_renderer = ShadowRenderer::new(device, uniforms, shadow_map_size);

        Self {
            start_time: Instant::now(),
            target_resources: vec![],
            required_targets: 0,
            target_index: 0,
            shadow_renderer,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.required_targets = 0;
        self.target_index = 0;
    }

    pub(crate) fn require_target(&mut self, device: &Device, uniforms: &mut Uniforms) {
        self.required_targets += 1;
        if self.required_targets > self.target_resources.len() {
            self.target_resources
                .push(TargetResources::new(device, uniforms));
            self.shadow_renderer.require_target(device, uniforms);
        }
    }

    pub(crate) fn render(
        &mut self,
        device: &Device,
        canvas: &Canvas,
        camera: &Camera,
        uniforms: &mut Uniforms,
        target: Target,
    ) {
        // check whether should do shadow mapping
        // if there is no lights, then not
        let shadow_light = target
            .lights
            .iter()
            .enumerate()
            .find(|(_, l)| l.light_type == LightType::Directional);
        let mut shadow_light_index = 4;
        let mut shadow_params = ShadowSplitParams::default();
        if let Some(l) = shadow_light {
            shadow_light_index = l.0 as u32;
            let mut view = camera.clone();
            view.depth = target.shadow_depth;
            shadow_params = self.shadow_renderer.render(
                device,
                uniforms,
                &target,
                *l.1,
                view,
                self.target_index,
            );
        }

        let target_resources = &mut self.target_resources[self.target_index];
        let cmd = device.commands();

        let shadow_pcf = match target.shadow_softness {
            Pcf::Disabled => 2.0,
            Pcf::X4 => 0.0,
            Pcf::X16 => 1.0,
        };

        let lights = [
            target.lights[0].shader(),
            target.lights[1].shader(),
            target.lights[2].shader(),
            target.lights[3].shader(),
        ];

        let skybox_index = target
            .skybox
            .as_ref()
            .map(|s| s.read().shader_index())
            .unwrap_or(0);

        // update world uniform
        target_resources.world_buffer.copy_from_data(&[ShaderWorld {
            shadow_splits: shadow_params.splits,
            shadow_texels: shadow_params.texels,
            shadow_diameters: shadow_params.diameters,
            world_to_shadow: shadow_params.world_to_shadow,
            time: self.start_time.elapsed().as_secs_f32(),
            camera_position: camera.position,
            world_to_view: camera.world_to_view(),
            view_to_clip: camera.view_to_clip(),
            ambient_color: target.ambient,
            exposure: target.exposure,
            shadow_light_index,
            skybox_index,
            lights,
            shadow_pcf,
        }]);

        // do render pass
        cmd.begin_render_pass(canvas);

        // clear attachments
        if matches!(target.clear, Clear::Depth | Clear::ColorDepth) {
            cmd.clear_depth_attachment(canvas);
        }
        if matches!(target.clear, Clear::Color | Clear::ColorDepth) {
            cmd.clear_color_attachments(canvas, target.background.into());
        }

        cmd.set_view(canvas.width, canvas.height);
        cmd.bind_descriptor(uniforms, target_resources.world_descriptor);

        let Target {
            builtins,
            tri_orders,
            char_orders,
            mesh_orders,
            line_orders,
            skybox,
            ..
        } = target;

        // skybox rendering
        if skybox.is_some() {
            record_skybox(cmd, uniforms, camera, &builtins);
        }

        // normal mesh rendering
        if !mesh_orders.is_empty() {
            record_meshes(cmd, uniforms, mesh_orders);
        }

        // tri rendering
        if !tri_orders.is_empty() {
            self.record_shapes(device, uniforms, &builtins, tri_orders);
        }

        // text rendering
        if !char_orders.is_empty() {
            self.record_text(device, uniforms, &builtins, char_orders);
        }

        // line rendering
        if !line_orders.is_empty() {
            self.record_lines(device, uniforms, &builtins, line_orders);
        }

        // end rendering
        cmd.end_render_pass();
        canvas.blit_to_texture(cmd);

        self.target_index = (self.target_index + 1) % self.target_resources.len();
    }

    fn record_text(
        &mut self,
        device: &Device,
        uniforms: &Uniforms,
        builtins: &Builtins,
        orders: Vec<CharOrder>,
    ) {
        let cmd = device.commands();

        // update text batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut textures = vec![];
        let mut indices = vec![];
        let mut uvs = vec![];

        for order in orders {
            let o = vertices.len() as u32;
            vertices.extend(&[
                order.points[0],
                order.points[1],
                order.points[2],
                order.points[3],
            ]);
            uvs.extend(&[order.uvs[0], order.uvs[1], order.uvs[2], order.uvs[3]]);
            colors.extend(&[order.color.into(); 4]);
            textures.extend(&[order.texture; 4]);
            indices.extend(&[o, o + 1, o + 2, o, o + 2, o + 3]);
        }

        // bind shader
        cmd.bind_shader(&builtins.font_shader.read());

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material.read());

        // bind and draw mesh
        let text_mesh = &mut self.target_resources[self.target_index].text_mesh;

        text_mesh.vertices = vertices;
        text_mesh.colors = colors;
        text_mesh.textures = textures;
        text_mesh.uvs = uvs;
        text_mesh.indices = indices;
        text_mesh.update(device);

        cmd.bind_mesh(text_mesh);
        cmd.push_constants(
            uniforms,
            ShaderConstants {
                local_to_world: Mat4::identity(),
                tint_color: Vec3::default(),
                texture_index: 0,
                sampler_index: 0,
            },
        );
        cmd.draw(text_mesh.index_count(), 0);
    }

    fn record_lines(
        &mut self,
        device: &Device,
        uniforms: &Uniforms,
        builtins: &Builtins,
        orders: Vec<LineOrder>,
    ) {
        let cmd = device.commands();

        // update line batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut indices = vec![];

        for order in orders {
            let o = vertices.len() as u32;
            let color = order.color.into();
            vertices.extend(&[order.points[0], order.points[1]]);
            colors.extend(&[color, color]);
            indices.extend(&[o, o + 1]);
        }

        // bind shader
        cmd.bind_shader(&builtins.line_shader.read());

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material.read());

        // bind and draw mesh
        let line_mesh = &mut self.target_resources[self.target_index].line_mesh;
        line_mesh.vertices = vertices;
        line_mesh.colors = colors;
        line_mesh.indices = indices;
        line_mesh.update(device);

        cmd.bind_mesh(line_mesh);
        cmd.push_constants(
            uniforms,
            ShaderConstants {
                local_to_world: Mat4::identity(),
                tint_color: Vec3::default(),
                texture_index: 0,
                sampler_index: 0,
            },
        );
        cmd.draw(line_mesh.index_count(), 0);
    }

    fn record_shapes(
        &mut self,
        device: &Device,
        uniforms: &Uniforms,
        builtins: &Builtins,
        mut orders: Vec<TriOrder>,
    ) {
        let cmd = device.commands();

        // update shape batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut textures = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        // order shape orders
        orders.sort_by(|a, b| {
            // sort by opacity
            if a.opaque && !b.opaque {
                Ordering::Less
            } else if !a.opaque && b.opaque {
                Ordering::Greater
            } else if a.opaque && b.opaque {
                Ordering::Equal
            } else {
                // sort by z. might need to change in the future
                if a.points[0].z > b.points[0].z {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });

        // add opaque shapes
        for order in orders {
            let texture = order.texture;
            let sampler = order.sampler_index;
            let color = order.color.into();

            let o = vertices.len() as u32;
            vertices.extend(&[order.points[0], order.points[1], order.points[2]]);
            colors.extend(&[color, color, color]);
            textures.extend(&[texture, texture, texture]);
            uvs.extend(&[order.uvs[0], order.uvs[1], order.uvs[2]]);
            indices.extend(&[o, o + 1, o + 2]);

            // use normal to store sampler
            normals.extend(&[Vec3::new(sampler as f32, 0.0, 0.0); 3]);
        }

        // bind shader
        cmd.bind_shader(&builtins.shape_shader.read());

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material.read());

        // bind and draw mesh
        let shape_mesh = &mut self.target_resources[self.target_index].shape_mesh;
        shape_mesh.vertices = vertices;
        shape_mesh.colors = colors;
        shape_mesh.textures = textures;
        shape_mesh.uvs = uvs;
        shape_mesh.normals = normals;
        shape_mesh.indices = indices;
        shape_mesh.update(device);

        cmd.bind_mesh(shape_mesh);
        cmd.push_constants(
            uniforms,
            ShaderConstants {
                local_to_world: Mat4::identity(),
                tint_color: Vec3::default(),
                texture_index: 0,
                sampler_index: 0,
            },
        );
        cmd.draw(shape_mesh.index_count(), 0);
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        for resources in &self.target_resources {
            resources.destroy(device);
        }
        self.shadow_renderer.destroy(device, uniforms);
    }
}

impl TargetResources {
    fn new(device: &Device, uniforms: &mut Uniforms) -> Self {
        let world_buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let world_descriptor = uniforms.world_set(device, &world_buffer);
        let text_mesh = Mesh::new(device);
        let line_mesh = Mesh::new(device);
        let shape_mesh = Mesh::new(device);

        Self {
            world_buffer,
            world_descriptor,
            text_mesh,
            line_mesh,
            shape_mesh,
        }
    }

    fn destroy(&self, device: &Device) {
        self.world_buffer.destroy(device);
        self.text_mesh.destroy(device);
        self.line_mesh.destroy(device);
        self.shape_mesh.destroy(device);
    }
}

fn record_meshes(cmd: &Commands, uniforms: &Uniforms, orders: Vec<ShaderOrder>) {
    for s_order in orders {
        // bind shader
        cmd.bind_shader(&s_order.shader.read());

        for m_order in &s_order.orders {
            // bind material
            cmd.bind_material(uniforms, &m_order.material.read());

            for order in &m_order.orders {
                cmd.push_constants(
                    uniforms,
                    ShaderConstants {
                        local_to_world: order.matrix,
                        tint_color: Rgbf::from(order.color).into(),
                        texture_index: order.texture_index,
                        sampler_index: order.sampler_index,
                    },
                );
                {
                    let m = order.mesh.read();
                    cmd.bind_mesh(&m);
                    cmd.draw(m.index_count(), 0);
                }
            }
        }
    }
}

fn record_skybox(cmd: &Commands, uniforms: &Uniforms, camera: &Camera, builtins: &Builtins) {
    cmd.bind_shader(&builtins.skybox_shader.read());
    let local_to_world = Mat4::compose(
        camera.position,
        Vec3::uniform(camera.depth * 2.0 - 0.1),
        Quat::default(),
    );
    cmd.push_constants(
        uniforms,
        ShaderConstants {
            sampler_index: 0,
            texture_index: 0,
            tint_color: Vec3::default(),
            local_to_world,
        },
    );
    {
        let m = builtins.cube_mesh.read();
        cmd.bind_mesh(&m);
        cmd.draw(m.index_count(), 0);
    }
}
