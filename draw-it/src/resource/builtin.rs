// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// builtin resource creation

use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;

use super::Ref;
use super::ResourceManager;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;

#[derive(Debug, Clone)]
pub struct Builtins {
    pub white_texture: Ref<Texture>,
    pub white_material: Ref<Material>,
    pub font_material: Ref<Material>,
    pub surface_mesh: Ref<Mesh>,
    pub quad_mesh: Ref<Mesh>,
    pub cube_mesh: Ref<Mesh>,
    pub sphere_mesh: Ref<Mesh>,
    pub grid_mesh: Ref<Mesh>,
    pub phong_shader: Ref<Shader>,
    pub sdf_font_shader: Ref<Shader>,
    pub bitmap_font_shader: Ref<Shader>,
    pub blit_shader: Ref<Shader>,
    pub wireframe_shader: Ref<Shader>,
    pub line_shader: Ref<Shader>,
    pub unshaded_shader: Ref<Shader>,
    pub skybox_shader: Ref<Shader>,
    pub kenney_font: Ref<Font>,
}

impl Builtins {
    pub(crate) fn new(
        device: &Arc<Device>,
        resources: &mut ResourceManager,
        framebuffer: &Framebuffer,
        layout: &ShaderLayout,
        uniform: &mut ImageUniform,
    ) -> Result<Self> {
        // textures
        let white_texture =
            resources.add_texture(Texture::new(device, uniform, Default::default())?);

        // materials
        let white_material = {
            let mut mat = Material::new(device, layout)?;
            mat.set_phong_color([255, 255, 255]);
            resources.add_material(mat)
        };
        let font_material = {
            let mut mat = Material::new(device, layout)?;
            mat.set_font_color([0, 0, 0]);
            mat.set_font_width(0.5);
            mat.set_font_edge(0.1);
            resources.add_material(mat)
        };

        // meshes
        let surface_mesh = resources.add_mesh(create_surface(device)?);
        let quad_mesh = resources.add_mesh(create_quad(device)?);
        let cube_mesh = resources.add_mesh(create_cube(device)?);
        let sphere_mesh = resources.add_mesh(create_sphere(device, 3)?);
        let grid_mesh = resources.add_mesh(create_grid(device, 50)?);

        // shaders
        let phong_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/phong.shader"),
        )?);

        let sdf_font_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/sdf-font.shader"),
        )?);

        let bitmap_font_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/bitmap-font.shader"),
        )?);

        let blit_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/blit.shader"),
        )?);

        let wireframe_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/wireframe.shader"),
        )?);

        let line_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/lines.shader"),
        )?);

        let unshaded_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/unshaded.shader"),
        )?);

        let skybox_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../shaders/skybox.shader"),
        )?);

        // fonts
        let kenney_font = resources.add_font(Font::new(
            device,
            uniform,
            include_bytes!("../../fonts/kenney-future.font"),
        )?);

        Ok(Self {
            white_texture,
            white_material,
            font_material,
            surface_mesh,
            quad_mesh,
            cube_mesh,
            sphere_mesh,
            grid_mesh,
            phong_shader,
            sdf_font_shader,
            bitmap_font_shader,
            blit_shader,
            wireframe_shader,
            line_shader,
            unshaded_shader,
            skybox_shader,
            kenney_font,
        })
    }
}

fn create_surface(device: &Arc<Device>) -> Result<Mesh> {
    let vertices = &[
        Vector3::new(-1.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
    ];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let indices = &[0, 1, 2, 0, 2, 3];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            indices,
            uvs,
            ..Default::default()
        },
    )
}

fn create_quad(device: &Arc<Device>) -> Result<Mesh> {
    let vertices = &[
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ];
    let uvs = &[
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ];
    let indices = &[0, 1, 2, 0, 2, 3];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            indices,
            uvs,
            ..Default::default()
        },
    )
}

fn create_cube(device: &Arc<Device>) -> Result<Mesh> {
    let top = create_rectangle(
        device,
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    )?;

    let bottom = create_rectangle(
        device,
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
    )?;

    let back = create_rectangle(
        device,
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
    )?;

    let front = create_rectangle(
        device,
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
    )?;

    let left = create_rectangle(
        device,
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
    )?;

    let right = create_rectangle(
        device,
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
    )?;

    combine_meshes(device, &[top, bottom, front, back, left, right])
}

fn create_grid(device: &Arc<Device>, size: u32) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut colors = vec![];
    let mut indices = vec![];
    let half = size as i32 / 2;

    for x in -half..=half {
        let color = if x == 0 {
            Color::GREEN
        } else {
            Color::rgba_norm(0.5, 0.5, 0.5, 0.5)
        };
        let vc = vertices.len() as u32;
        vertices.extend(&[
            Vector3::new(x as f32, 0.0, half as f32),
            Vector3::new(x as f32, 0.0, -half as f32),
        ]);
        colors.extend(&[color, color]);
        indices.extend(&[vc, vc + 1]);
    }
    for z in -half..=half {
        let color = if z == 0 {
            Color::BLUE
        } else {
            Color::rgba_norm(0.5, 0.5, 0.5, 0.5)
        };
        let vc = vertices.len() as u32;
        vertices.extend(&[
            Vector3::new(half as f32, 0.0, z as f32),
            Vector3::new(-half as f32, 0.0, z as f32),
        ]);
        colors.extend(&[color, color]);
        indices.extend(&[vc, vc + 1]);
    }

    Mesh::new(
        device,
        MeshOptions {
            vertices: &vertices,
            indices: &indices,
            colors: &colors,
            ..Default::default()
        },
    )
}

fn create_rectangle<V: Into<Vector3>>(
    device: &Arc<Device>,
    p1: V,
    p2: V,
    p3: V,
    p4: V,
) -> Result<Mesh> {
    let vertices = &[p1.into(), p2.into(), p3.into(), p4.into()];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let indices = &[0, 1, 2, 0, 2, 3];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            indices,
            uvs,
            ..Default::default()
        },
    )
}

fn create_sphere(device: &Arc<Device>, detail_level: u32) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut indices = vec![];

    // 12 icosahedron vertices
    let t = (1.0 + 5.0f32.sqrt()) / 2.0;

    vertices.push(Vector3::new(-1.0, t, 0.0).unit() * 0.5);
    vertices.push(Vector3::new(1.0, t, 0.0).unit() * 0.5);
    vertices.push(Vector3::new(-1.0, -t, 0.0).unit() * 0.5);
    vertices.push(Vector3::new(1.0, -t, 0.0).unit() * 0.5);

    vertices.push(Vector3::new(0.0, -1.0, t).unit() * 0.5);
    vertices.push(Vector3::new(0.0, 1.0, t).unit() * 0.5);
    vertices.push(Vector3::new(0.0, -1.0, -t).unit() * 0.5);
    vertices.push(Vector3::new(0.0, 1.0, -t).unit() * 0.5);

    vertices.push(Vector3::new(t, 0.0, -1.0).unit() * 0.5);
    vertices.push(Vector3::new(t, 0.0, 1.0).unit() * 0.5);
    vertices.push(Vector3::new(-t, 0.0, -1.0).unit() * 0.5);
    vertices.push(Vector3::new(-t, 0.0, 1.0).unit() * 0.5);

    // 20 icosahedron triangles
    indices.extend(&[0, 11, 5]);
    indices.extend(&[0, 5, 1]);
    indices.extend(&[0, 1, 7]);
    indices.extend(&[0, 7, 10]);
    indices.extend(&[0, 10, 11]);

    indices.extend(&[1, 5, 9]);
    indices.extend(&[5, 11, 4]);
    indices.extend(&[11, 10, 2]);
    indices.extend(&[10, 7, 6]);
    indices.extend(&[7, 1, 8]);

    indices.extend(&[3, 9, 4]);
    indices.extend(&[3, 4, 2]);
    indices.extend(&[3, 2, 6]);
    indices.extend(&[3, 6, 8]);
    indices.extend(&[3, 8, 9]);

    indices.extend(&[4, 9, 5]);
    indices.extend(&[2, 4, 11]);
    indices.extend(&[6, 2, 10]);
    indices.extend(&[8, 6, 7]);
    indices.extend(&[9, 8, 1]);

    // refine triangles
    let mut midpoints = HashMap::new();
    for _ in 0..detail_level {
        let mut new_indices = vec![];
        for tri in indices.chunks(3) {
            // replace triangle with 4 triangles
            let a = get_middle_point(&mut vertices, tri[0], tri[1], &mut midpoints);
            let b = get_middle_point(&mut vertices, tri[1], tri[2], &mut midpoints);
            let c = get_middle_point(&mut vertices, tri[2], tri[0], &mut midpoints);

            new_indices.extend(&[tri[0], a, c]);
            new_indices.extend(&[tri[1], b, a]);
            new_indices.extend(&[tri[2], c, b]);
            new_indices.extend(&[a, b, c]);
        }
        indices = new_indices;
    }

    let mut uvs = vec![];
    for vertex in vertices.iter() {
        let u = vertex.z.atan2(vertex.x) / (2.0 * PI);
        let v = (vertex.y.asin() / PI) + 0.5;
        uvs.push(Vector2::new(u, v));
    }

    Mesh::new(
        device,
        MeshOptions {
            vertices: &vertices,
            indices: &indices,
            uvs: &uvs,
            ..Default::default()
        },
    )
}

fn get_middle_point(
    vertices: &mut Vec<Vector3>,
    p1: u32,
    p2: u32,
    midpoints: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    match (midpoints.get(&(p1, p2)), midpoints.get(&(p2, p1))) {
        (Some(i), _) => *i,
        (_, Some(i)) => *i,
        (None, None) => {
            let point_1 = vertices[p1 as usize];
            let point_2 = vertices[p2 as usize];
            let middle = (point_1 + point_2) / 2.0;

            vertices.push(middle.unit() * 0.5);
            let i = vertices.len() as u32 - 1;
            midpoints.insert((p1, p2), i);
            i
        }
    }
}

fn combine_meshes(device: &Arc<Device>, meshes: &[Mesh]) -> Result<Mesh> {
    let mut offset = 0;
    let mut indices = vec![];
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut colors = vec![];
    for mesh in meshes {
        indices.extend(mesh.indices().iter().map(|t| t + offset));
        vertices.extend(mesh.vertices());
        normals.extend(mesh.normals());
        uvs.extend(mesh.uvs());
        colors.extend(mesh.colors());
        offset = vertices.len() as u32;
    }

    Mesh::new(
        device,
        MeshOptions {
            vertices: &vertices,
            normals: &normals,
            uvs: &uvs,
            colors: &colors,
            indices: &indices,
        },
    )
}
