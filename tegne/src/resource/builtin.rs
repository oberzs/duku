// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// builtin resource creation

use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;

use super::Ref;
use super::ResourceManager;
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
use crate::pipeline::ShaderOptions;

#[derive(Clone)]
pub(crate) struct Builtins {
    pub(crate) white_texture: Ref<Texture>,
    pub(crate) white_material: Ref<Material>,
    pub(crate) font_material: Ref<Material>,
    pub(crate) surface_mesh: Ref<Mesh>,
    pub(crate) quad_mesh: Ref<Mesh>,
    pub(crate) cube_mesh: Ref<Mesh>,
    pub(crate) sphere_mesh: Ref<Mesh>,
    pub(crate) phong_shader: Ref<Shader>,
    pub(crate) font_shader: Ref<Shader>,
    pub(crate) blit_shader: Ref<Shader>,
    pub(crate) wireframe_shader: Ref<Shader>,
    pub(crate) unshaded_shader: Ref<Shader>,
    pub(crate) roboto_font: Ref<Font>,
}

impl Builtins {
    pub(crate) fn new(
        device: &Arc<Device>,
        resources: &mut ResourceManager,
        framebuffer: &Framebuffer,
        layout: &ShaderLayout,
        uniform: &ImageUniform,
    ) -> Result<Self> {
        profile_scope!("new");

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
        let sphere_mesh = resources.add_mesh(create_sphere(device, 5)?);

        // shaders
        let phong_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../assets/shaders/phong.shader"),
            Default::default(),
        )?);

        let font_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../assets/shaders/font.shader"),
            Default::default(),
        )?);

        let blit_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../assets/shaders/blit.shader"),
            ShaderOptions {
                depth_test: false,
                ..Default::default()
            },
        )?);

        let wireframe_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../assets/shaders/wireframe.shader"),
            ShaderOptions {
                lines: true,
                depth_test: false,
                ..Default::default()
            },
        )?);

        let unshaded_shader = resources.add_shader(Shader::new(
            device,
            framebuffer,
            layout,
            include_bytes!("../../assets/shaders/unshaded.shader"),
            Default::default(),
        )?);

        // fonts
        let roboto_font = {
            let font = Font::new(
                device,
                uniform,
                resources,
                include_bytes!("../../assets/fonts/RobotoMono-Regular.font"),
            )?;
            resources.add_font(font)
        };

        Ok(Self {
            white_texture,
            white_material,
            font_material,
            surface_mesh,
            quad_mesh,
            cube_mesh,
            sphere_mesh,
            phong_shader,
            font_shader,
            blit_shader,
            wireframe_shader,
            unshaded_shader,
            roboto_font,
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
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ];
    let triangles = &[[0, 2, 1], [0, 3, 2]];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            triangles,
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
    let triangles = &[[0, 1, 2], [0, 2, 3]];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            triangles,
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
    let triangles = &[[0, 1, 2], [0, 2, 3]];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            triangles,
            uvs,
            ..Default::default()
        },
    )
}

fn create_sphere(device: &Arc<Device>, detail_level: u32) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut triangles = vec![];

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
    triangles.push([0, 11, 5]);
    triangles.push([0, 5, 1]);
    triangles.push([0, 1, 7]);
    triangles.push([0, 7, 10]);
    triangles.push([0, 10, 11]);

    triangles.push([1, 5, 9]);
    triangles.push([5, 11, 4]);
    triangles.push([11, 10, 2]);
    triangles.push([10, 7, 6]);
    triangles.push([7, 1, 8]);

    triangles.push([3, 9, 4]);
    triangles.push([3, 4, 2]);
    triangles.push([3, 2, 6]);
    triangles.push([3, 6, 8]);
    triangles.push([3, 8, 9]);

    triangles.push([4, 9, 5]);
    triangles.push([2, 4, 11]);
    triangles.push([6, 2, 10]);
    triangles.push([8, 6, 7]);
    triangles.push([9, 8, 1]);

    // refine triangles
    let mut midpoints = HashMap::new();
    for _ in 0..detail_level {
        let mut new_triangles = vec![];
        for tri in triangles {
            // replace triangle with 4 triangles
            let a = get_middle_point(&mut vertices, tri[0], tri[1], &mut midpoints);
            let b = get_middle_point(&mut vertices, tri[1], tri[2], &mut midpoints);
            let c = get_middle_point(&mut vertices, tri[2], tri[0], &mut midpoints);

            new_triangles.push([tri[0], a, c]);
            new_triangles.push([tri[1], b, a]);
            new_triangles.push([tri[2], c, b]);
            new_triangles.push([a, b, c]);
        }
        triangles = new_triangles;
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
            triangles: &triangles,
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
    let mut triangles = vec![];
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut colors = vec![];
    for mesh in meshes {
        triangles.extend(
            mesh.triangles()
                .iter()
                .map(|t| [t[0] + offset, t[1] + offset, t[2] + offset]),
        );
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
            triangles: &triangles,
        },
    )
}
