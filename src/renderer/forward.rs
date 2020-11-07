// Oliver Berzs
// https://github.com/oberzs/draw-it

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::time::Instant;

use super::Camera;
use super::Pcf;
use super::ShadowRenderer;
use super::Target;
use crate::device::Commands;
use crate::device::Device;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderWorld;
use crate::pipeline::Uniforms;
use crate::storage::Store;

pub(crate) struct ForwardRenderer {
    shadow_renderer: ShadowRenderer,
    start_time: Instant,
}

pub(crate) struct RenderStores<'s> {
    pub(crate) shaders: &'s Store<Shader>,
    pub(crate) fonts: &'s Store<Font>,
    pub(crate) materials: &'s Store<Material>,
    pub(crate) textures: &'s Store<Texture>,
    pub(crate) meshes: &'s Store<Mesh>,
}

impl ForwardRenderer {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms, shadow_map_size: u32) -> Self {
        let shadow_renderer = ShadowRenderer::new(device, uniforms, shadow_map_size);

        Self {
            start_time: Instant::now(),
            shadow_renderer,
        }
    }

    pub(crate) fn render(
        &mut self,
        device: &Device,
        framebuffer: &mut Framebuffer,
        camera: &Camera,
        stores: RenderStores<'_>,
        uniforms: &Uniforms,
        target: Target<'_, '_>,
    ) {
        // shadow mapping pass
        let mut view = camera.clone();
        view.depth = target.shadow_depth;
        let shadow_params =
            self.shadow_renderer
                .render(device, uniforms, stores.meshes, &target, view);

        let cmd = device.commands();

        // bind current shadow map set
        // cmd.bind_descriptor(shader_layout, self.shadow_frames[current].descriptor);

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

        // update world uniform
        framebuffer.update_world(ShaderWorld {
            shadow_splits: shadow_params.splits,
            shadow_texels: shadow_params.texels,
            shadow_diameters: shadow_params.diameters,
            world_to_shadow: shadow_params.world_to_shadow,
            time: self.start_time.elapsed().as_secs_f32(),
            camera_position: camera.transform.position,
            world_to_view: camera.world_to_view(),
            view_to_clip: camera.view_to_clip(),
            skybox_index: target.skybox.map(|s| s.id()).unwrap_or(0),
            ambient_color: Vector3::from(target.ambient_color),
            lights,
            shadow_pcf,
        });

        // do render pass
        cmd.begin_render_pass(framebuffer, target.clear_color.to_rgba_norm());
        cmd.set_view(framebuffer.size());
        cmd.bind_descriptor(uniforms, framebuffer.world());

        // skybox rendering
        if target.skybox.is_some() {
            record_skybox(cmd, &target, &stores, uniforms, camera);
        }

        // normal mesh rendering
        if !target.mesh_orders.is_empty() {
            record_meshes(cmd, &target, &stores, uniforms);
        }

        // shape rendering
        if !target.shape_orders.is_empty() {
            record_shapes(device, framebuffer, &target, &stores, uniforms);
        }

        // text rendering
        if !target.text_orders.is_empty() {
            record_text(device, framebuffer, &target, &stores, uniforms);
        }

        // line rendering
        if !target.line_orders.is_empty() {
            record_lines(device, framebuffer, &target, &stores, uniforms);
        }

        // end rendering
        cmd.end_render_pass();
        framebuffer.blit_to_texture(cmd);
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        self.shadow_renderer.destroy(device, uniforms);
    }
}

fn record_meshes(
    cmd: &Commands,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    uniforms: &Uniforms,
) {
    for s_order in &target.mesh_orders {
        // bind shader
        let shader = stores.shaders.get(&s_order.shader);
        cmd.bind_shader(shader);

        for m_order in &s_order.orders {
            // bind material
            let material = stores.materials.get(&m_order.material);
            cmd.bind_material(uniforms, material);

            for order in &m_order.orders {
                let mesh = stores.meshes.get(&order.mesh);

                cmd.push_constants(
                    uniforms,
                    ShaderConstants {
                        local_to_world: order.local_to_world,
                        sampler_index: order.sampler_index,
                    },
                );
                cmd.bind_mesh(mesh);
                cmd.draw(mesh.index_count(), 0);
            }
        }
    }
}

fn record_skybox(
    cmd: &Commands,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    uniforms: &Uniforms,
    camera: &Camera,
) {
    let shader = stores.shaders.get(&target.builtins.skybox_shader);
    cmd.bind_shader(shader);

    let mesh = stores.meshes.get(&target.builtins.cube_mesh);
    cmd.bind_mesh(mesh);

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
    cmd.draw(mesh.index_count(), 0);
}

fn record_text(
    device: &Device,
    framebuffer: &mut Framebuffer,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    uniforms: &Uniforms,
) {
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
        let font = stores.fonts.get(&order.font);

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

            let data = font.char_data(c);
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
                quat.rotate_vector(Vector3::new(pos1.x, pos1.y, pos1.z)),
                quat.rotate_vector(Vector3::new(pos2.x, pos1.y, pos1.z)),
                quat.rotate_vector(Vector3::new(pos2.x, pos2.y, pos1.z)),
                quat.rotate_vector(Vector3::new(pos1.x, pos2.y, pos1.z)),
            ]);
            uvs.extend(&[
                Vector2::new(data.uvs.x, data.uvs.y),
                Vector2::new(data.uvs.z, data.uvs.y),
                Vector2::new(data.uvs.z, data.uvs.w),
                Vector2::new(data.uvs.x, data.uvs.w),
            ]);
            colors.extend(&[order.color; 4]);
            textures.extend(&[font.texture().shader_index(); 4]);
            indices.extend(&[o, o + 1, o + 2, o, o + 2, o + 3]);

            transform.position.x += data.advance * transform.scale.x;
        }
    }

    // bind shader
    let shader = stores.shaders.get(&builtins.font_shader);
    cmd.bind_shader(shader);

    // bind material
    let material = stores.materials.get(&builtins.white_material);
    cmd.bind_material(uniforms, material);

    // bind and draw mesh
    let text_mesh = framebuffer.text_mesh();
    text_mesh.set_vertices(vertices);
    text_mesh.set_colors(colors);
    text_mesh.set_textures(textures);
    text_mesh.set_uvs(uvs);
    text_mesh.set_indices(indices);
    text_mesh.update_if_needed(device);

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

fn record_lines(
    device: &Device,
    framebuffer: &mut Framebuffer,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    uniforms: &Uniforms,
) {
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
        let point_1 = (matrix * order.points[0].extend(1.0)).shrink();
        let point_2 = (matrix * order.points[1].extend(1.0)).shrink();

        let o = vertices.len() as u32;
        vertices.extend(&[point_1, point_2]);
        colors.extend(&[order.color, order.color]);
        indices.extend(&[o, o + 1]);
    }

    // bind shader
    let shader = stores.shaders.get(&builtins.line_shader);
    cmd.bind_shader(shader);

    // bind material
    let material = stores.materials.get(&builtins.white_material);
    cmd.bind_material(uniforms, material);

    // bind and draw mesh
    let line_mesh = framebuffer.line_mesh();
    line_mesh.set_vertices(vertices);
    line_mesh.set_colors(colors);
    line_mesh.set_indices(indices);
    line_mesh.update_if_needed(device);

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

fn record_shapes(
    device: &Device,
    framebuffer: &mut Framebuffer,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    uniforms: &Uniforms,
) {
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
        let point_1 = (matrix * order.points[0].extend(1.0)).shrink();
        let point_2 = (matrix * order.points[1].extend(1.0)).shrink();
        let point_3 = (matrix * order.points[2].extend(1.0)).shrink();
        let texture = stores.textures.get(&order.texture).shader_index();
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
    let shader = stores.shaders.get(&builtins.shape_shader);
    cmd.bind_shader(shader);

    // bind material
    let material = stores.materials.get(&builtins.white_material);
    cmd.bind_material(uniforms, material);

    // bind and draw mesh
    let shape_mesh = framebuffer.shape_mesh();
    shape_mesh.set_vertices(vertices);
    shape_mesh.set_colors(colors);
    shape_mesh.set_textures(textures);
    shape_mesh.set_uvs(uvs);
    shape_mesh.set_normals(normals);
    shape_mesh.set_indices(indices);
    shape_mesh.update_if_needed(device);

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
