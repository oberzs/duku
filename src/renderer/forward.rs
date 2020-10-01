// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::time::Instant;

use super::Camera;
use super::MeshOrder;
use super::Target;
use crate::device::Commands;
use crate::device::Device;
use crate::device::FRAMES_IN_FLIGHT;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::Size;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::Mesh;
use crate::pipeline::Descriptor;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderWorld;
use crate::storage::Store;

pub(crate) struct ForwardRenderer {
    shadow_frames: [ShadowMapSet; FRAMES_IN_FLIGHT],
    shadow_shader: Shader,
    shadow_pcf: Pcf,
    start_time: Instant,
}

#[derive(Debug, Copy, Clone)]
pub enum Pcf {
    X16,
    X4,
    Disabled,
}

pub(crate) struct RenderStores<'s> {
    pub(crate) shaders: &'s Store<Shader>,
    pub(crate) fonts: &'s Store<Font>,
    pub(crate) materials: &'s Store<Material>,
    pub(crate) meshes: &'s Store<Mesh>,
}

struct ShadowMapSet {
    framebuffers: [Framebuffer; 4],
    descriptor: Descriptor,
    world_to_shadow: [Matrix4; 4],
    cascades: [f32; 4],
    map_size: u32,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Device,
        shader_layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
        shadow_map_size: u32,
        shadow_pcf: Pcf,
    ) -> Self {
        let shadow_frames = [
            ShadowMapSet::new(device, shader_layout, shader_images, shadow_map_size),
            ShadowMapSet::new(device, shader_layout, shader_images, shadow_map_size),
        ];

        let shadow_shader = Shader::from_spirv_bytes(
            device,
            &shadow_frames[0].framebuffers[0],
            shader_layout,
            include_bytes!("../../shaders/shadow.spirv"),
        )
        .expect("bad shader");

        Self {
            start_time: Instant::now(),
            shadow_frames,
            shadow_shader,
            shadow_pcf,
        }
    }

    pub(crate) fn draw(
        &mut self,
        device: &Device,
        framebuffer: &mut Framebuffer,
        camera: &Camera,
        stores: RenderStores<'_>,
        shader_layout: &ShaderLayout,
        target: Target<'_, '_>,
    ) {
        let current = device.current_frame();

        // reset current matrices and cascades
        self.shadow_frames[current].world_to_shadow = [Matrix4::identity(); 4];
        self.shadow_frames[current].cascades = [0.0; 4];

        // shadow mapping pass
        let mut view = camera.clone();
        view.depth = 50.0;
        self.shadow_pass(device, shader_layout, stores.meshes, &target, &view);

        let cmd = device.commands();

        // bind current shadow map set
        cmd.bind_descriptor(shader_layout, self.shadow_frames[current].descriptor);

        let shadow_pcf = match self.shadow_pcf {
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
        framebuffer.update_world(
            device,
            ShaderWorld {
                shadow_cascades: self.shadow_frames[current].cascades,
                world_to_shadow: self.shadow_frames[current].world_to_shadow,
                shadow_bias: target.shadow_bias,
                time: self.start_time.elapsed().as_secs_f32(),
                camera_position: camera.transform.position,
                world_to_view: camera.world_to_view(),
                view_to_clip: camera.view_to_clip(),
                lights,
                shadow_pcf,
            },
        );

        // do render pass
        cmd.begin_render_pass(framebuffer, target.clear_color.to_rgba_norm());
        cmd.set_view(framebuffer.size());
        cmd.bind_descriptor(shader_layout, framebuffer.world());

        // skybox rendering
        if target.skybox {
            record_skybox(cmd, &target, &stores, shader_layout, camera);
        }

        // normal mesh rendering
        if !target.mesh_orders.is_empty() {
            record_meshes(cmd, &target, &stores, shader_layout);
        }

        // text rendering
        if !target.text_orders.is_empty() {
            record_text(device, framebuffer, &target, &stores, shader_layout);
        }

        // line rendering
        if !target.line_orders.is_empty() {
            record_lines(device, framebuffer, &target, &stores, shader_layout);
        }

        // end rendering
        cmd.end_render_pass();
        framebuffer.blit_to_texture(cmd);
    }

    fn shadow_pass(
        &mut self,
        device: &Device,
        shader_layout: &ShaderLayout,
        meshes: &Store<Mesh>,
        target: &Target<'_, '_>,
        view: &Camera,
    ) {
        let light_dir = target
            .lights
            .iter()
            .find(|l| l.shadows)
            .map(|l| l.coords)
            .unwrap_or_default();

        let cmd = device.commands();
        let current = device.current_frame();

        // bind temp shadow map set so we can write to main one
        cmd.bind_descriptor(shader_layout, self.shadow_frames[current].descriptor);

        // render shadow map for each cascade
        let mut prev_cs = 0.0;
        for (i, cs) in target.shadow_cascades.iter().enumerate() {
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
            self.shadow_frames[current].world_to_shadow[i] = light_matrix;
            self.shadow_frames[current].cascades[i] = view.depth * cs;
            prev_cs = *cs;

            // update world uniform
            let framebuffer = &mut self.shadow_frames[current].framebuffers[i];
            framebuffer.update_world(
                device,
                ShaderWorld {
                    world_to_shadow: [Matrix4::identity(); 4],
                    camera_position: Vector3::default(),
                    lights: [Default::default(); 4],
                    world_to_view: light_view_matrix,
                    view_to_clip: light_ortho_matrix,
                    shadow_cascades: [0.0; 4],
                    shadow_bias: 0.0,
                    shadow_pcf: 0.0,
                    time: 0.0,
                },
            );

            // do render pass
            cmd.begin_render_pass(framebuffer, (1.0, 1.0, 1.0, 1.0));
            cmd.set_view(framebuffer.size());
            cmd.bind_descriptor(shader_layout, framebuffer.world());
            cmd.bind_shader(&self.shadow_shader);

            for s_order in &target.mesh_orders {
                for m_order in &s_order.orders {
                    for order in &m_order.orders {
                        if order.shadows {
                            record_order(cmd, meshes, shader_layout, order);
                        }
                    }
                }
            }
            cmd.end_render_pass();
        }
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.shadow_shader.destroy(device);
        for frame in &self.shadow_frames {
            for framebuffer in &frame.framebuffers {
                framebuffer.destroy(device);
            }
        }
    }
}

impl ShadowMapSet {
    pub(crate) fn new(
        device: &Device,
        shader_layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
        map_size: u32,
    ) -> Self {
        let framebuffers = [
            Self::shadow_framebuffer(device, shader_layout, shader_images, map_size),
            Self::shadow_framebuffer(device, shader_layout, shader_images, map_size),
            Self::shadow_framebuffer(device, shader_layout, shader_images, map_size),
            Self::shadow_framebuffer(device, shader_layout, shader_images, map_size),
        ];
        let descriptor = shader_layout.shadow_map_set(
            device,
            [
                framebuffers[0].stored_view(),
                framebuffers[1].stored_view(),
                framebuffers[2].stored_view(),
                framebuffers[3].stored_view(),
            ],
        );

        Self {
            world_to_shadow: [Matrix4::identity(); 4],
            cascades: [0.0; 4],
            framebuffers,
            descriptor,
            map_size,
        }
    }

    fn shadow_framebuffer(
        device: &Device,
        shader_layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
        size: u32,
    ) -> Framebuffer {
        Framebuffer::new(
            device,
            shader_layout,
            shader_images,
            &[ImageFormat::Depth],
            Msaa::Disabled,
            Size::new(size, size),
        )
    }
}

fn record_meshes(
    cmd: &Commands,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    shader_layout: &ShaderLayout,
) {
    for s_order in &target.mesh_orders {
        // bind shader
        let shader = stores.shaders.get(&s_order.shader);
        cmd.bind_shader(shader);

        for m_order in &s_order.orders {
            // bind material
            let material = stores.materials.get(&m_order.material);
            cmd.bind_material(shader_layout, material);

            for order in &m_order.orders {
                record_order(cmd, stores.meshes, shader_layout, order);
            }
        }
    }
}

fn record_skybox(
    cmd: &Commands,
    target: &Target<'_, '_>,
    stores: &RenderStores<'_>,
    shader_layout: &ShaderLayout,
    camera: &Camera,
) {
    let shader = stores.shaders.get(&target.builtins.skybox_shader);
    cmd.bind_shader(shader);

    let mesh = stores.meshes.get(&target.builtins.cube_mesh);
    cmd.bind_mesh(mesh);

    let local_to_world = (Transform {
        position: camera.transform.position,
        scale: Vector3::uniform(camera.depth * 2.0 - 0.1),
        ..Default::default()
    })
    .as_matrix();
    cmd.push_constants(
        shader_layout,
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
    shader_layout: &ShaderLayout,
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

            let o = vertices.len() as u16;
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
    cmd.bind_material(shader_layout, material);

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
        shader_layout,
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
    shader_layout: &ShaderLayout,
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
        let matrix = order.transform.as_matrix();
        let point_1 = (matrix * order.point_1.extend(1.0)).shrink();
        let point_2 = (matrix * order.point_2.extend(1.0)).shrink();

        let o = vertices.len() as u16;
        vertices.extend(&[point_1, point_2]);
        colors.extend(&[order.color, order.color]);
        indices.extend(&[o, o + 1]);
    }

    // bind shader
    let shader = stores.shaders.get(&builtins.line_shader);
    cmd.bind_shader(shader);

    // bind material
    let material = stores.materials.get(&builtins.white_material);
    cmd.bind_material(shader_layout, material);

    // bind and draw mesh
    let line_mesh = framebuffer.line_mesh();
    line_mesh.set_vertices(vertices);
    line_mesh.set_colors(colors);
    line_mesh.set_indices(indices);
    line_mesh.update_if_needed(device);

    cmd.bind_mesh(line_mesh);
    cmd.push_constants(
        shader_layout,
        ShaderConstants {
            local_to_world: Matrix4::identity(),
            sampler_index: 0,
        },
    );
    cmd.draw(line_mesh.index_count(), 0);
}

fn record_order(
    cmd: &Commands,
    meshes: &Store<Mesh>,
    shader_layout: &ShaderLayout,
    order: &MeshOrder,
) {
    let mesh = meshes.get(&order.mesh);

    cmd.push_constants(
        shader_layout,
        ShaderConstants {
            local_to_world: order.local_to_world,
            sampler_index: order.sampler_index,
        },
    );
    cmd.bind_mesh(mesh);
    cmd.draw(mesh.index_count(), 0);
}
