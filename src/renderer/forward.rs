// Oliver Berzs
// https://github.com/oberzs/duku

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::time::Instant;

use super::Camera;
use super::Pcf;
use super::ShadowRenderer;
use super::Target;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::device::Commands;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Descriptor;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderWorld;
use crate::pipeline::Uniforms;

pub(crate) struct ForwardRenderer {
    target_resources: Vec<TargetResources>,
    shadow_renderer: ShadowRenderer,
    start_time: Instant,
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
    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        shadow_map_size: u32,
        target_count: u32,
    ) -> Result<Self> {
        let shadow_renderer = ShadowRenderer::new(device, uniforms, shadow_map_size, target_count)?;
        let target_resources: Vec<_> = (0..target_count)
            .map(|_| TargetResources::new(device, uniforms))
            .collect::<Result<_>>()?;

        Ok(Self {
            start_time: Instant::now(),
            target_index: 0,
            target_resources,
            shadow_renderer,
        })
    }

    pub(crate) fn add_target(&mut self, device: &Device, uniforms: &mut Uniforms) -> Result<()> {
        self.target_resources
            .push(TargetResources::new(device, uniforms)?);
        self.shadow_renderer.add_target(device, uniforms)?;

        Ok(())
    }

    pub(crate) fn render(
        &mut self,
        device: &Device,
        framebuffer: &Framebuffer,
        camera: &Camera,
        uniforms: &Uniforms,
        target: Target,
    ) {
        // shadow mapping pass
        let mut view = camera.clone();
        view.depth = target.shadow_depth;
        let shadow_params =
            self.shadow_renderer
                .render(device, uniforms, &target, view, self.target_index);

        let target_resources = &mut self.target_resources[self.target_index];
        let cmd = device.commands();

        let shadow_pcf = match target.shadow_pcf {
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
            .map(|s| s.shader_index())
            .unwrap_or(0);

        // update world uniform
        target_resources.world_buffer.copy_from_data(&[ShaderWorld {
            shadow_splits: shadow_params.splits,
            shadow_texels: shadow_params.texels,
            shadow_diameters: shadow_params.diameters,
            world_to_shadow: shadow_params.world_to_shadow,
            time: self.start_time.elapsed().as_secs_f32(),
            camera_position: camera.transform.position,
            world_to_view: camera.world_to_view(),
            view_to_clip: camera.view_to_clip(),
            ambient_color: Vector3::from(target.ambient_color),
            max_white_point: target.max_white_point,
            skybox_index,
            lights,
            shadow_pcf,
        }]);

        // do render pass
        cmd.begin_render_pass(framebuffer, target.clear_color.to_rgba_norm());
        cmd.set_view(framebuffer.width(), framebuffer.height());
        cmd.bind_descriptor(uniforms, target_resources.world_descriptor);

        // skybox rendering
        if target.skybox.is_some() {
            record_skybox(cmd, &target, uniforms, camera);
        }

        // normal mesh rendering
        if !target.mesh_orders.is_empty() {
            record_meshes(cmd, &target, uniforms);
        }

        // shape rendering
        if !target.shape_orders.is_empty() {
            self.record_shapes(device, &target, uniforms);
        }

        // text rendering
        if !target.text_orders.is_empty() {
            self.record_text(device, &target, uniforms);
        }

        // line rendering
        if !target.line_orders.is_empty() {
            self.record_lines(device, &target, uniforms);
        }

        // end rendering
        cmd.end_render_pass();
        framebuffer.blit_to_texture(cmd);

        self.target_index = (self.target_index + 1) % self.target_resources.len();
    }

    fn record_text(&mut self, device: &Device, target: &Target, uniforms: &Uniforms) {
        let cmd = device.commands();
        let Target {
            text_orders,
            builtins,
            ..
        } = &target;

        // update text batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut textures = vec![];
        let mut indices = vec![];
        let mut uvs = vec![];

        for order in text_orders {
            let mut transform = order.transform;
            let quat = transform.rotation;
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

                let data = order.font.char_data(c);
                let mut local_transform = transform;
                local_transform.position.x += data.x_offset * transform.scale.x;
                local_transform.position.y -= data.y_offset * transform.scale.y;

                let pos1 = local_transform.position;
                let pos2 = pos1
                    + Vector3::new(
                        data.width * transform.scale.x,
                        -data.height * transform.scale.y,
                        0.0,
                    );

                let o = vertices.len() as u32;
                vertices.extend(&[
                    quat * Vector3::new(pos1.x, pos1.y, pos1.z),
                    quat * Vector3::new(pos2.x, pos1.y, pos1.z),
                    quat * Vector3::new(pos2.x, pos2.y, pos1.z),
                    quat * Vector3::new(pos1.x, pos2.y, pos1.z),
                ]);
                uvs.extend(&[
                    Vector2::new(data.uvs.x, data.uvs.y),
                    Vector2::new(data.uvs.z, data.uvs.y),
                    Vector2::new(data.uvs.z, data.uvs.w),
                    Vector2::new(data.uvs.x, data.uvs.w),
                ]);
                colors.extend(&[order.color; 4]);
                textures.extend(&[order.font.texture().shader_index(); 4]);
                indices.extend(&[o, o + 1, o + 2, o, o + 2, o + 3]);

                transform.position.x += data.advance * transform.scale.x;
            }
        }

        // bind shader
        cmd.bind_shader(&builtins.font_shader);

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material);

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
                local_to_world: Matrix4::identity(),
                sampler_index: 7,
            },
        );
        cmd.draw(text_mesh.index_count(), 0);
    }

    fn record_lines(&mut self, device: &Device, target: &Target, uniforms: &Uniforms) {
        let cmd = device.commands();
        let Target {
            line_orders,
            builtins,
            ..
        } = &target;

        // update line batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut indices = vec![];

        for order in line_orders {
            let matrix = Matrix4::from(order.transform);
            let point_1 = matrix * order.points[0];
            let point_2 = matrix * order.points[1];

            let o = vertices.len() as u32;
            vertices.extend(&[point_1, point_2]);
            colors.extend(&[order.color, order.color]);
            indices.extend(&[o, o + 1]);
        }

        // bind shader
        cmd.bind_shader(&builtins.line_shader);

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material);

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
                local_to_world: Matrix4::identity(),
                sampler_index: 0,
            },
        );
        cmd.draw(line_mesh.index_count(), 0);
    }

    fn record_shapes(&mut self, device: &Device, target: &Target, uniforms: &Uniforms) {
        let cmd = device.commands();
        let Target {
            shape_orders,
            builtins,
            ..
        } = &target;

        // update shape batching mesh
        let mut vertices = vec![];
        let mut colors = vec![];
        let mut textures = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        for order in shape_orders {
            let matrix = Matrix4::from(order.transform);
            let point_1 = matrix * order.points[0];
            let point_2 = matrix * order.points[1];
            let point_3 = matrix * order.points[2];

            let texture = order.texture.shader_index();
            let sampler = order.sampler_index;

            let o = vertices.len() as u32;
            vertices.extend(&[point_1, point_2, point_3]);
            colors.extend(&[order.color, order.color, order.color]);
            textures.extend(&[texture, texture, texture]);
            uvs.extend(&[order.uvs[0], order.uvs[1], order.uvs[2]]);
            indices.extend(&[o, o + 1, o + 2]);

            // use normal to store sampler
            normals.extend(&[Vector3::new(sampler as f32, 0.0, 0.0); 3]);
        }

        // bind shader
        cmd.bind_shader(&builtins.shape_shader);

        // bind material
        cmd.bind_material(uniforms, &builtins.white_material);

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
                local_to_world: Matrix4::identity(),
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
    fn new(device: &Device, uniforms: &mut Uniforms) -> Result<Self> {
        let world_buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let world_descriptor = uniforms.world_set(device, &world_buffer)?;
        let text_mesh = Mesh::new(device);
        let line_mesh = Mesh::new(device);
        let shape_mesh = Mesh::new(device);

        Ok(Self {
            world_buffer,
            world_descriptor,
            text_mesh,
            line_mesh,
            shape_mesh,
        })
    }

    fn destroy(&self, device: &Device) {
        self.world_buffer.destroy(device);
        self.text_mesh.destroy(device);
        self.line_mesh.destroy(device);
        self.shape_mesh.destroy(device);
    }
}

fn record_meshes(cmd: &Commands, target: &Target, uniforms: &Uniforms) {
    for s_order in &target.mesh_orders {
        // bind shader
        cmd.bind_shader(&s_order.shader);

        for m_order in &s_order.orders {
            // bind material
            cmd.bind_material(uniforms, &m_order.material);

            for order in &m_order.orders {
                cmd.push_constants(
                    uniforms,
                    ShaderConstants {
                        local_to_world: order.local_to_world,
                        sampler_index: order.sampler_index,
                    },
                );
                cmd.bind_mesh(&order.mesh);
                cmd.draw(order.mesh.index_count(), 0);
            }
        }
    }
}

fn record_skybox(cmd: &Commands, target: &Target, uniforms: &Uniforms, camera: &Camera) {
    cmd.bind_shader(&target.builtins.skybox_shader);
    cmd.bind_mesh(&target.builtins.cube_mesh);

    let local_to_world = Matrix4::from(Transform {
        position: camera.transform.position,
        scale: Vector3::uniform(camera.depth * 2.0 - 0.1),
        ..Default::default()
    });
    cmd.push_constants(
        uniforms,
        ShaderConstants {
            sampler_index: 0,
            local_to_world,
        },
    );
    cmd.draw(target.builtins.cube_mesh.index_count(), 0);
}
