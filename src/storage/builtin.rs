// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// builtin resource creation

use std::collections::HashMap;
use std::f32::consts::PI;

use super::Handle;
use super::Storage;
use crate::device::Device;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::ImageFormat;
use crate::image::Size;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;

#[derive(Debug)]
pub struct Builtins {
    // textures
    pub white_texture: Handle<Texture>,

    // materials
    pub white_material: Handle<Material>,

    // meshes
    pub surface_mesh: Handle<Mesh>,
    pub quad_mesh: Handle<Mesh>,
    pub cube_mesh: Handle<Mesh>,
    pub sphere_mesh: Handle<Mesh>,

    // shaders
    pub phong_shader: Handle<Shader>,
    pub font_shader: Handle<Shader>,
    pub blit_shader: Handle<Shader>,
    pub wireframe_shader: Handle<Shader>,
    pub line_shader: Handle<Shader>,
    pub shape_shader: Handle<Shader>,
    pub unshaded_shader: Handle<Shader>,
    pub skybox_shader: Handle<Shader>,

    // fonts
    pub fira_font: Handle<Font>,
}

impl Builtins {
    pub(crate) fn new(
        device: &Device,
        storage: &mut Storage,
        framebuffer: &Framebuffer,
        layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
    ) -> Self {
        // textures
        let white_texture = {
            let tex = Texture::new(
                device,
                shader_images,
                vec![255, 255, 255, 255],
                Size::new(1, 1),
                ImageFormat::Rgba,
            );
            storage.add_texture(tex)
        };

        // materials
        let white_material = {
            let mut mat = Material::new(device, layout);
            mat.set_phong_color((255, 255, 255));
            mat.update_if_needed(device);
            storage.add_material(mat)
        };

        // meshes
        let surface_mesh = storage.add_mesh(create_surface(device));
        let quad_mesh = storage.add_mesh(create_quad(device));
        let cube_mesh = storage.add_mesh(create_cube(device));
        let sphere_mesh = storage.add_mesh(create_sphere(device, 3));

        // shaders
        let phong_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/phong.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let font_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/font.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let blit_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/blit.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let wireframe_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/wireframe.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let line_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/lines.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let shape_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/shape.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let unshaded_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/unshaded.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        let skybox_shader = {
            let shader = Shader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/skybox.spirv"),
            )
            .expect("bad shader");
            storage.add_shader(shader)
        };

        // fonts
        let fira_font = {
            let font = Font::fira_mono(device, shader_images);
            storage.add_font(font)
        };

        Self {
            white_texture,
            white_material,
            surface_mesh,
            quad_mesh,
            cube_mesh,
            sphere_mesh,
            phong_shader,
            font_shader,
            blit_shader,
            wireframe_shader,
            line_shader,
            shape_shader,
            unshaded_shader,
            skybox_shader,
            fira_font,
        }
    }
}

fn create_surface(device: &Device) -> Mesh {
    let mut mesh = Mesh::new(device);

    mesh.set_vertices(vec![
        Vector3::new(-1.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
    ]);
    mesh.set_uvs(vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ]);
    mesh.set_indices(vec![0, 1, 2, 0, 2, 3]);
    mesh.calculate_normals();
    mesh.update_if_needed(device);

    mesh
}

fn create_quad(device: &Device) -> Mesh {
    let mut mesh = Mesh::new(device);

    mesh.set_vertices(vec![
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ]);
    mesh.set_uvs(vec![
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ]);
    mesh.set_indices(vec![0, 1, 2, 0, 2, 3]);
    mesh.calculate_normals();
    mesh.update_if_needed(device);

    mesh
}

fn create_cube(device: &Device) -> Mesh {
    let top = create_rectangle(
        device,
        (-0.5, 0.5, 0.5),
        (0.5, 0.5, 0.5),
        (0.5, 0.5, -0.5),
        (-0.5, 0.5, -0.5),
    );

    let bottom = create_rectangle(
        device,
        (0.5, -0.5, 0.5),
        (-0.5, -0.5, 0.5),
        (-0.5, -0.5, -0.5),
        (0.5, -0.5, -0.5),
    );

    let back = create_rectangle(
        device,
        (0.5, 0.5, 0.5),
        (-0.5, 0.5, 0.5),
        (-0.5, -0.5, 0.5),
        (0.5, -0.5, 0.5),
    );

    let front = create_rectangle(
        device,
        (-0.5, 0.5, -0.5),
        (0.5, 0.5, -0.5),
        (0.5, -0.5, -0.5),
        (-0.5, -0.5, -0.5),
    );

    let left = create_rectangle(
        device,
        (-0.5, 0.5, 0.5),
        (-0.5, 0.5, -0.5),
        (-0.5, -0.5, -0.5),
        (-0.5, -0.5, 0.5),
    );

    let right = create_rectangle(
        device,
        (0.5, 0.5, -0.5),
        (0.5, 0.5, 0.5),
        (0.5, -0.5, 0.5),
        (0.5, -0.5, -0.5),
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

    mesh.set_vertices(vec![p1.into(), p2.into(), p3.into(), p4.into()]);
    mesh.set_uvs(vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ]);
    mesh.set_indices(vec![0, 1, 2, 0, 2, 3]);
    mesh.calculate_normals();

    mesh
}

fn create_sphere(device: &Device, detail_level: u32) -> Mesh {
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
    mesh.set_vertices(vertices);
    mesh.set_indices(indices);
    mesh.set_uvs(uvs);
    mesh.calculate_normals();
    mesh.update_if_needed(device);
    mesh
}

fn get_middle_point(
    vertices: &mut Vec<Vector3>,
    p1: u16,
    p2: u16,
    midpoints: &mut HashMap<(u16, u16), u16>,
) -> u16 {
    match (midpoints.get(&(p1, p2)), midpoints.get(&(p2, p1))) {
        (Some(i), _) => *i,
        (_, Some(i)) => *i,
        (None, None) => {
            let point_1 = vertices[p1 as usize];
            let point_2 = vertices[p2 as usize];
            let middle = (point_1 + point_2) / 2.0;

            vertices.push(middle.unit() * 0.5);
            let i = vertices.len() as u16 - 1;
            midpoints.insert((p1, p2), i);
            i
        }
    }
}
