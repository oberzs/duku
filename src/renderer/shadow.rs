// Oliver Berzs
// https://github.com/oberzs/duku

// ShadowRenderer - generates shadow maps
// using parallel-split shadow maps (PSSM)

use super::Camera;
use super::Light;
use super::Target;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Rgbf;
use crate::device::Device;
use crate::image::Canvas;
use crate::image::Msaa;
use crate::math::Mat4;
use crate::math::Vec3;
use crate::math::Vec4;
use crate::pipeline::Descriptor;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConfig;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderWorld;
use crate::pipeline::Uniforms;

const SHADOW_SPLIT_COUNT: usize = 4;

pub(crate) struct ShadowRenderer {
    target_resources: Vec<TargetResources>,
    shader: Shader,
    map_size: u32,
}

#[derive(Default)]
pub(crate) struct ShadowSplitParams {
    pub(crate) world_to_shadow: [Mat4; SHADOW_SPLIT_COUNT],
    pub(crate) splits: [f32; SHADOW_SPLIT_COUNT],
    pub(crate) texels: [f32; SHADOW_SPLIT_COUNT],
    pub(crate) diameters: [f32; SHADOW_SPLIT_COUNT],
}

struct TargetResources {
    world_descriptors: [Descriptor; SHADOW_SPLIT_COUNT],
    world_buffers: [Buffer<ShaderWorld>; SHADOW_SPLIT_COUNT],
    shadow_descriptor: Descriptor,
    shadow_maps: [Canvas; SHADOW_SPLIT_COUNT],
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl ShadowRenderer {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms, map_size: u32) -> Self {
        let shader = Shader::from_spirv_bytes(
            device,
            uniforms,
            Msaa::Disabled,
            include_bytes!("../../shaders/shadow.spirv"),
        )
        .expect("bad shader");

        Self {
            target_resources: vec![],
            shader,
            map_size,
        }
    }

    pub(crate) fn require_target(&mut self, device: &Device, uniforms: &mut Uniforms) {
        self.target_resources.push(TargetResources::new(
            device,
            uniforms,
            self.shader.config(),
            self.map_size,
        ));
    }

    pub(crate) fn render(
        &mut self,
        device: &Device,
        uniforms: &mut Uniforms,
        target: &Target,
        light: Light,
        view: Camera,
        target_index: usize,
    ) -> ShadowSplitParams {
        let light_dir = light.coords;

        let mut params = ShadowSplitParams {
            world_to_shadow: [Mat4::identity(); SHADOW_SPLIT_COUNT],
            splits: [0.0; SHADOW_SPLIT_COUNT],
            texels: [0.0; SHADOW_SPLIT_COUNT],
            diameters: [0.0; SHADOW_SPLIT_COUNT],
        };

        let target_resources = &mut self.target_resources[target_index];
        let cmd = device.commands();

        cmd.bind_descriptor(uniforms, target_resources.shadow_descriptor);

        // calculate shadow map splits
        for i in 1..=SHADOW_SPLIT_COUNT {
            params.splits[i - 1] = pssm_split(view.near(), view.depth, i, target.shadow_split);
        }

        // render shadow map for each split
        for i in 0..SHADOW_SPLIT_COUNT {
            // get view frustum bounding sphere
            let prev_split = if i == 0 { 0.0 } else { params.splits[i - 1] };
            let bounds = bounds_for_split(&view, prev_split, params.splits[i]);
            let diameter = bounds.radius * 2.0;
            let up = if light_dir.y < 1.0 && light_dir.y > -1.0 {
                Vec3::up()
            } else {
                Vec3::forward()
            };
            let light_position = bounds.center - light_dir * bounds.radius;
            let light_view_matrix =
                Mat4::look_rotation(light_dir, up) * Mat4::translation(-light_position);
            let mut light_ortho_matrix = Mat4::orthographic(diameter, diameter, 0.0, diameter);

            // stabilize shadow map by using texel units
            let shadow_matrix = light_ortho_matrix * light_view_matrix;
            let mut shadow_origin = Vec4::new(0.0, 0.0, 0.0, 1.0);
            shadow_origin = shadow_matrix * shadow_origin;
            shadow_origin *= self.map_size as f32 / 2.0;
            let rounded_origin = shadow_origin.round();
            let mut round_offset = rounded_origin - shadow_origin;
            round_offset *= 2.0 / self.map_size as f32;
            light_ortho_matrix.w.x += round_offset.x;
            light_ortho_matrix.w.y += round_offset.y;

            params.world_to_shadow[i] = light_ortho_matrix * light_view_matrix;
            params.texels[i] = diameter / self.map_size as f32;
            params.diameters[i] = diameter;

            // update world uniform
            target_resources.world_buffers[i].copy_from_data(&[ShaderWorld {
                world_to_view: light_view_matrix,
                view_to_clip: light_ortho_matrix,

                // these fields are not important
                shadow_light_index: 0,
                world_to_shadow: [Mat4::identity(); 4],
                camera_position: Vec3::default(),
                ambient_color: Vec3::default(),
                lights: [Default::default(); 4],
                shadow_splits: [0.0; 4],
                shadow_texels: [0.0; 4],
                shadow_diameters: [0.0; 4],
                exposure: 0.0,
                shadow_pcf: 0.0,
                skybox_index: 0,
                time: 0.0,
            }]);

            // do render pass
            let texture = &target_resources.shadow_maps[i];
            cmd.begin_render_pass(texture, Rgbf::gray(1.0));
            cmd.set_view(texture.width, texture.height);
            cmd.bind_descriptor(uniforms, target_resources.world_descriptors[i]);
            cmd.bind_shader(&self.shader);

            for s_order in &target.mesh_orders {
                for m_order in &s_order.orders {
                    for order in &m_order.orders {
                        if order.shadows {
                            cmd.push_constants(
                                uniforms,
                                ShaderConstants {
                                    local_to_world: order.matrix,
                                    tint_color: Vec3::default(),
                                    texture_index: 0,
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
            cmd.end_render_pass();
        }

        params
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        self.shader.destroy(device);
        for resources in &self.target_resources {
            resources.destroy(device, uniforms);
        }
    }
}

impl TargetResources {
    fn new(device: &Device, uniforms: &mut Uniforms, config: ShaderConfig, map_size: u32) -> Self {
        let shadow_maps = [
            Canvas::new(device, uniforms, config, map_size, map_size),
            Canvas::new(device, uniforms, config, map_size, map_size),
            Canvas::new(device, uniforms, config, map_size, map_size),
            Canvas::new(device, uniforms, config, map_size, map_size),
        ];
        let shadow_descriptor = uniforms.shadow_map_set(
            device,
            [
                shadow_maps[0].stored_view(),
                shadow_maps[1].stored_view(),
                shadow_maps[2].stored_view(),
                shadow_maps[3].stored_view(),
            ],
        );

        let world_buffers = [
            Buffer::dynamic(device, BufferUsage::Uniform, 1),
            Buffer::dynamic(device, BufferUsage::Uniform, 1),
            Buffer::dynamic(device, BufferUsage::Uniform, 1),
            Buffer::dynamic(device, BufferUsage::Uniform, 1),
        ];
        let world_descriptors = [
            uniforms.world_set(device, &world_buffers[0]),
            uniforms.world_set(device, &world_buffers[1]),
            uniforms.world_set(device, &world_buffers[2]),
            uniforms.world_set(device, &world_buffers[3]),
        ];

        Self {
            world_descriptors,
            world_buffers,
            shadow_maps,
            shadow_descriptor,
        }
    }

    fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        for buffer in &self.world_buffers {
            buffer.destroy(device);
        }
        for map in &self.shadow_maps {
            map.destroy(device, uniforms);
        }
    }
}

fn pssm_split(near: f32, far: f32, i: usize, split_coef: f32) -> f32 {
    let c = match split_coef {
        c if c < 0.0 => 0.0,
        c if c > 1.0 => 1.0,
        c => c,
    };

    c * logorithmic_split(near, far, i) + (1.0 - c) * uniform_split(near, far, i)
}

fn uniform_split(near: f32, far: f32, i: usize) -> f32 {
    near + (far - near) * (i as f32 / SHADOW_SPLIT_COUNT as f32)
}

fn logorithmic_split(near: f32, far: f32, i: usize) -> f32 {
    near * (far / near).powf(i as f32 / SHADOW_SPLIT_COUNT as f32)
}

fn bounds_for_split(view: &Camera, near: f32, far: f32) -> Sphere {
    let mut frustum_corners = [
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
    ];

    let view_to_clip = view.view_to_clip();
    let world_to_view = view.world_to_view();
    let full_depth = view.depth - view.near();

    let inverse_projection = view_to_clip.inverse().expect("bad projection");

    // get projection frustum corners from NDC
    for corner in &mut frustum_corners {
        let point = inverse_projection * Vec4::from((*corner, 1.0));
        *corner = point.xyz() / point.w;
    }

    // cut out a section (near -> far) from the frustum
    for i in 0..4 {
        let corner_ray = frustum_corners[i + 4] - frustum_corners[i];
        let near_corner_ray = corner_ray * (near / full_depth);
        let far_corner_ray = corner_ray * (far / full_depth);
        frustum_corners[i + 4] = frustum_corners[i] + far_corner_ray;
        frustum_corners[i] += near_corner_ray;
    }

    let frustum_center = frustum_corners.iter().sum::<Vec3>() / frustum_corners.len() as f32;

    // get bounding sphere radius
    // sphere makes it axis-aligned
    let mut radius = 0.0;
    for corner in &frustum_corners {
        let distance = (*corner - frustum_center).length();
        if distance > radius {
            radius = distance;
        }
    }

    // round radius to 1/16 increments
    radius = (radius * 16.0).ceil() / 16.0;

    // transform frustum center into view space
    let center = world_to_view.inverse().expect("no inverse") * frustum_center;

    Sphere { center, radius }
}
