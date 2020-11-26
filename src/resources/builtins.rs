// Oliver Berzs
// https://github.com/oberzs/duku

use std::collections::HashMap;
use std::f32::consts::PI;

use super::Handle;
use super::Resources;
use crate::device::Device;
use crate::error::Result;
use crate::font::Font;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Format;
use crate::image::Mips;
use crate::image::Msaa;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::Uniforms;

#[derive(Debug, Clone)]
pub struct Builtins {
    // textures
    pub white_texture: Handle<Texture>,
    pub blue_texture: Handle<Texture>,
    pub black_texture: Handle<Texture>,

    // cubemaps
    pub white_cubemap: Handle<Cubemap>,

    // materials
    pub white_material: Handle<Material>,

    // meshes
    pub surface_mesh: Handle<Mesh>,
    pub quad_mesh: Handle<Mesh>,
    pub cube_mesh: Handle<Mesh>,
    pub ico_sphere_mesh: Handle<Mesh>,
    pub uv_sphere_mesh: Handle<Mesh>,

    // shaders
    pub pbr_shader: Handle<Shader>,
    pub font_shader: Handle<Shader>,
    pub wireframe_shader: Handle<Shader>,
    pub line_shader: Handle<Shader>,
    pub shape_shader: Handle<Shader>,
    pub unshaded_shader: Handle<Shader>,
    pub skybox_shader: Handle<Shader>,
    pub fullscreen_shader: Handle<Shader>,

    // fonts
    pub fira_font: Handle<Font>,
}

impl Builtins {
    pub(crate) fn new(
        device: &Device,
        resources: &mut Resources,
        uniforms: &mut Uniforms,
        msaa: Msaa,
    ) -> Result<Self> {
        // textures
        let white_texture = {
            let tex = Texture::new(
                device,
                uniforms,
                vec![255, 255, 255, 255],
                1,
                1,
                Format::Rgba,
                Mips::Zero,
            )?;
            resources.add_texture(tex)
        };
        let blue_texture = {
            let tex = Texture::new(
                device,
                uniforms,
                vec![128, 128, 255, 255],
                1,
                1,
                Format::Rgba,
                Mips::Zero,
            )?;
            resources.add_texture(tex)
        };
        let black_texture = {
            let tex = Texture::new(
                device,
                uniforms,
                vec![0, 0, 0, 255],
                1,
                1,
                Format::Rgba,
                Mips::Zero,
            )?;
            resources.add_texture(tex)
        };

        // cubemaps
        let white_cubemap = {
            let cub = Cubemap::new(
                &device,
                uniforms,
                1,
                Format::Rgba,
                CubemapSides {
                    top: vec![255, 255, 255, 255],
                    bottom: vec![255, 255, 255, 255],
                    front: vec![255, 255, 255, 255],
                    back: vec![255, 255, 255, 255],
                    left: vec![255, 255, 255, 255],
                    right: vec![255, 255, 255, 255],
                },
            )?;
            resources.add_cubemap(cub)
        };

        // materials
        let white_material = {
            let mut mat = Material::new(device, uniforms)?;
            mat.albedo_color([255, 255, 255]);
            mat.albedo_texture(white_texture.clone());
            mat.normal_texture(blue_texture.clone());
            mat.update();
            resources.add_material(mat)
        };

        // meshes
        let surface_mesh = resources.add_mesh(create_surface(device));
        let quad_mesh = resources.add_mesh(create_quad(device));
        let cube_mesh = resources.add_mesh(create_cube(device));
        let ico_sphere_mesh = resources.add_mesh(create_ico_sphere(device, 3));
        let uv_sphere_mesh = resources.add_mesh(create_uv_sphere(device, 30, 30));

        // shaders
        let pbr_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/pbr.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let font_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/font.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let wireframe_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/wireframe.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let line_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/lines.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let shape_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/shape.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let unshaded_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/unshaded.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let skybox_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/skybox.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        let fullscreen_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                uniforms,
                msaa,
                include_bytes!("../../shaders/fullscreen.spirv"),
            )
            .expect("bad shader");
            resources.add_shader(shader)
        };

        // fonts
        let fira_font = {
            let font = Font::fira_mono(device, uniforms)?;
            resources.add_font(font)
        };

        Ok(Self {
            white_texture,
            blue_texture,
            black_texture,
            white_cubemap,
            white_material,
            surface_mesh,
            quad_mesh,
            cube_mesh,
            ico_sphere_mesh,
            uv_sphere_mesh,
            pbr_shader,
            font_shader,
            wireframe_shader,
            line_shader,
            shape_shader,
            unshaded_shader,
            skybox_shader,
            fullscreen_shader,
            fira_font,
        })
    }

    pub(crate) fn invalidate_handles(&mut self) {
        self.white_texture.invalidate();
        self.blue_texture.invalidate();
        self.black_texture.invalidate();
        self.white_cubemap.invalidate();
        self.white_material.invalidate();
        self.surface_mesh.invalidate();
        self.quad_mesh.invalidate();
        self.cube_mesh.invalidate();
        self.ico_sphere_mesh.invalidate();
        self.uv_sphere_mesh.invalidate();
        self.pbr_shader.invalidate();
        self.font_shader.invalidate();
        self.wireframe_shader.invalidate();
        self.line_shader.invalidate();
        self.shape_shader.invalidate();
        self.unshaded_shader.invalidate();
        self.skybox_shader.invalidate();
        self.fullscreen_shader.invalidate();
        self.fira_font.invalidate();
    }
}

fn create_surface(device: &Device) -> Mesh {
    let mut mesh = Mesh::new(device);

    mesh.vertices = vec![
        Vector3::new(-1.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
    ];
    mesh.uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    mesh.indices = vec![0, 1, 2, 0, 2, 3];

    mesh.calculate_normals();
    mesh.update(device);
    mesh
}

fn create_quad(device: &Device) -> Mesh {
    let mut mesh = Mesh::new(device);

    mesh.vertices = vec![
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ];
    mesh.uvs = vec![
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ];
    mesh.indices = vec![0, 1, 2, 0, 2, 3];

    mesh.calculate_normals();
    mesh.update(device);
    mesh
}

pub(crate) fn create_cube(device: &Device) -> Mesh {
    let top = create_rectangle(
        device,
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    );

    let bottom = create_rectangle(
        device,
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
    );

    let back = create_rectangle(
        device,
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
    );

    let front = create_rectangle(
        device,
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
    );

    let left = create_rectangle(
        device,
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
    );

    let right = create_rectangle(
        device,
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
    );

    let mesh = Mesh::combine(device, &[&top, &bottom, &front, &back, &left, &right]);
    top.destroy(device);
    bottom.destroy(device);
    front.destroy(device);
    back.destroy(device);
    left.destroy(device);
    right.destroy(device);
    mesh
}

fn create_rectangle<V: Into<Vector3>>(device: &Device, p1: V, p2: V, p3: V, p4: V) -> Mesh {
    let mut mesh = Mesh::new(device);

    mesh.vertices = vec![p1.into(), p2.into(), p3.into(), p4.into()];
    mesh.uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    mesh.indices = vec![0, 1, 2, 0, 2, 3];
    mesh.calculate_normals();
    mesh.update(device);

    mesh
}

pub(crate) fn create_ico_sphere(device: &Device, detail_level: u32) -> Mesh {
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
    for vertex in &vertices {
        let u = vertex.z.atan2(vertex.x) / (2.0 * PI);
        let v = (vertex.y.asin() / PI) + 0.5;
        uvs.push(Vector2::new(u, v));
    }

    let mut mesh = Mesh::new(device);
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.uvs = uvs;
    mesh.calculate_normals();
    mesh.update(device);
    mesh
}

pub(crate) fn create_uv_sphere(device: &Device, meridians: u32, parallels: u32) -> Mesh {
    let mut vertices = vec![];
    let mut indices = vec![];

    vertices.push(Vector3::new(0.0, 1.0, 0.0) * 0.5);
    for j in 0..(parallels - 1) {
        let polar = PI * (j + 1) as f32 / parallels as f32;
        let sp = polar.sin();
        let cp = polar.cos();

        for i in 0..meridians {
            let azimuth = 2.0 * PI * i as f32 / meridians as f32;
            let sa = azimuth.sin();
            let ca = azimuth.cos();
            let x = sp * ca;
            let y = cp;
            let z = sp * sa;

            vertices.push(Vector3::new(x, y, z) * 0.5);
        }
    }
    vertices.push(Vector3::new(0.0, -1.0, 0.0) * 0.5);

    for i in 0..meridians {
        let a = i + 1;
        let b = (i + 1) % meridians + 1;
        indices.extend(&[0, b, a]);
    }

    for j in 0..(parallels - 2) {
        let a_start = j * meridians + 1;
        let b_start = (j + 1) * meridians + 1;
        for i in 0..meridians {
            let a = a_start + i;
            let a1 = a_start + (i + 1) % meridians;
            let b = b_start + i;
            let b1 = b_start + (i + 1) % meridians;
            indices.extend(&[a, a1, b1, a, b1, b]);
        }
    }

    for i in 0..meridians {
        let a = i + meridians * (parallels - 2) + 1;
        let b = (i + 1) % meridians + meridians * (parallels - 2) + 1;
        indices.extend(&[vertices.len() as u32 - 1, a, b]);
    }

    let mut uvs = vec![];
    for vertex in &vertices {
        let u = vertex.z.atan2(vertex.x) / (2.0 * PI);
        let v = (vertex.y.asin() / PI) + 0.5;
        uvs.push(Vector2::new(u, v));
    }

    let mut mesh = Mesh::new(device);
    mesh.vertices = vertices;
    mesh.uvs = uvs;
    mesh.indices = indices;
    mesh.calculate_normals();
    mesh.update(device);
    mesh
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
