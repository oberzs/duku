// Oliver Berzs
// https://github.com/oberzs/draw-it

// ShadowRenderer - generates shadow maps
// using parallel-split shadow maps (PSSM)

use super::Camera;
use super::LightType;
use super::Target;
use crate::device::Device;
use crate::device::FRAMES_IN_FLIGHT;
use crate::image::Format;
use crate::image::Framebuffer;
use crate::image::Msaa;
use crate::image::Size;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::Mesh;
use crate::pipeline::Descriptor;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderWorld;
use crate::pipeline::Uniforms;
use crate::storage::Store;

const SHADOW_SPLIT_COUNT: usize = 4;

#[derive(Debug, Copy, Clone)]
pub enum Pcf {
    X16,
    X4,
    Disabled,
}

pub(crate) struct ShadowRenderer {
    sets: [MapSet; FRAMES_IN_FLIGHT],
    shader: Shader,
    map_size: u32,
}

pub(crate) struct ShadowSplitParams {
    pub(crate) world_to_shadow: [Matrix4; SHADOW_SPLIT_COUNT],
    pub(crate) splits: [f32; SHADOW_SPLIT_COUNT],
    pub(crate) texels: [f32; SHADOW_SPLIT_COUNT],
    pub(crate) diameters: [f32; SHADOW_SPLIT_COUNT],
}

struct MapSet {
    maps: [Framebuffer; SHADOW_SPLIT_COUNT],
    descriptor: Descriptor,
}

struct Sphere {
    center: Vector3,
    radius: f32,
}

impl ShadowRenderer {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms, map_size: u32) -> Self {
        let sets = [
            MapSet::new(device, uniforms, map_size),
            MapSet::new(device, uniforms, map_size),
        ];

        let shader = Shader::from_spirv_bytes(
            device,
            &sets[0].maps[0],
            uniforms,
            include_bytes!("../../shaders/shadow.spirv"),
        )
        .expect("bad shader");

        Self {
            sets,
            shader,
            map_size,
        }
    }

    pub(crate) fn render(
        &self,
        device: &Device,
        uniforms: &Uniforms,
        meshes: &Store<Mesh>,
        target: &Target<'_, '_>,
        view: Camera,
    ) -> ShadowSplitParams {
        let mut params = ShadowSplitParams {
            world_to_shadow: [Matrix4::identity(); SHADOW_SPLIT_COUNT],
            splits: [0.0; SHADOW_SPLIT_COUNT],
            texels: [0.0; SHADOW_SPLIT_COUNT],
            diameters: [0.0; SHADOW_SPLIT_COUNT],
        };

        let cmd = device.commands();
        let current = device.current_frame();

        // get main light direction
        let light_dir = target
            .lights
            .iter()
            .find(|l| l.light_type == LightType::Main)
            .map(|l| l.coords)
            .unwrap_or_default();

        cmd.bind_descriptor(uniforms, self.sets[current].descriptor);

        // calculate shadow map splits
        for i in 1..=SHADOW_SPLIT_COUNT {
            params.splits[i - 1] = pssm_split(view.near(), view.depth, i, target.shadow_split_coef);
        }

        // render shadow map for each split
        for i in 0..SHADOW_SPLIT_COUNT {
            // get view frustum bounding sphere
            let prev_split = if i == 0 { 0.0 } else { params.splits[i - 1] };
            let bounds = bounds_for_split(&view, prev_split, params.splits[i]);
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
            shadow_origin *= self.map_size as f32 / 2.0;
            let rounded_origin = shadow_origin.round();
            let mut round_offset = rounded_origin - shadow_origin;
            round_offset *= 2.0 / self.map_size as f32;
            light_ortho_matrix.col_w.x += round_offset.x;
            light_ortho_matrix.col_w.y += round_offset.y;
            let light_matrix = light_ortho_matrix * light_view_matrix;

            params.world_to_shadow[i] = light_matrix;
            params.texels[i] = diameter / self.map_size as f32;
            params.diameters[i] = diameter;

            // update world uniform
            let framebuffer = &self.sets[current].maps[i];
            framebuffer.update_world(ShaderWorld {
                world_to_shadow: [Matrix4::identity(); 4],
                camera_position: Vector3::default(),
                ambient_color: Vector3::default(),
                lights: [Default::default(); 4],
                world_to_view: light_view_matrix,
                view_to_clip: light_ortho_matrix,
                shadow_splits: [0.0; 4],
                shadow_texels: [0.0; 4],
                shadow_diameters: [0.0; 4],
                shadow_pcf: 0.0,
                skybox_index: 0,
                time: 0.0,
            });

            // do render pass
            cmd.begin_render_pass(framebuffer, [1.0, 1.0, 1.0, 1.0]);
            cmd.set_view(framebuffer.size());
            cmd.bind_descriptor(uniforms, framebuffer.world());
            cmd.bind_shader(&self.shader);

            for s_order in &target.mesh_orders {
                for m_order in &s_order.orders {
                    for order in &m_order.orders {
                        if order.shadows {
                            let mesh = meshes.get(&order.mesh);

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
            cmd.end_render_pass();
        }

        params
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        self.shader.destroy(device);
        for set in &self.sets {
            set.destroy(device, uniforms);
        }
    }
}

impl MapSet {
    fn new(device: &Device, uniforms: &mut Uniforms, map_size: u32) -> Self {
        let maps = [
            create_map(device, uniforms, map_size),
            create_map(device, uniforms, map_size),
            create_map(device, uniforms, map_size),
            create_map(device, uniforms, map_size),
        ];
        let descriptor = uniforms.shadow_map_set(
            device,
            [
                maps[0].stored_view(),
                maps[1].stored_view(),
                maps[2].stored_view(),
                maps[3].stored_view(),
            ],
        );

        Self { maps, descriptor }
    }

    fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        for map in &self.maps {
            map.destroy(device, uniforms);
        }
    }
}

fn create_map(device: &Device, uniforms: &mut Uniforms, map_size: u32) -> Framebuffer {
    Framebuffer::new(
        device,
        uniforms,
        &[Format::Depth],
        Msaa::Disabled,
        Size::new(map_size, map_size),
    )
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
        Vector3::new(-1.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
        Vector3::new(-1.0, 1.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(1.0, -1.0, 1.0),
        Vector3::new(-1.0, -1.0, 1.0),
    ];

    let view_to_clip = view.view_to_clip();
    let world_to_view = view.world_to_view();
    let full_depth = view.depth - view.near();

    let inverse_projection = view_to_clip.inverse().expect("bad projection");

    // get projection frustum corners from NDC
    for corner in &mut frustum_corners {
        let point = inverse_projection * corner.extend(1.0);
        *corner = point.shrink() / point.w;
    }

    // cut out a section (near -> far) from the frustum
    for i in 0..4 {
        let corner_ray = frustum_corners[i + 4] - frustum_corners[i];
        let near_corner_ray = corner_ray * (near / full_depth);
        let far_corner_ray = corner_ray * (far / full_depth);
        frustum_corners[i + 4] = frustum_corners[i] + far_corner_ray;
        frustum_corners[i] += near_corner_ray;
    }

    let frustum_center = frustum_corners.iter().sum::<Vector3>() / frustum_corners.len() as f32;

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
    let center = world_to_view
        .inverse()
        .expect("no inverse")
        .transform_vector(frustum_center);

    Sphere { center, radius }
}
