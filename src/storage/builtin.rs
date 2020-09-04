// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// builtin resource creation

use std::collections::HashMap;
use std::f32::consts::PI;
use std::rc::Rc;

use super::Storage;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::font::CoreFont;
use crate::font::Font;
use crate::image::CoreFramebuffer;
use crate::image::CoreTexture;
use crate::image::ImageFormat;
use crate::image::Size;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::CoreMesh;
use crate::mesh::Mesh;
use crate::pipeline::CoreMaterial;
use crate::pipeline::CoreShader;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;

#[derive(Debug)]
pub struct Builtins {
    pub white_texture: Texture,
    pub white_material: Material,
    pub surface_mesh: Mesh,
    pub quad_mesh: Mesh,
    pub cube_mesh: Mesh,
    pub sphere_mesh: Mesh,
    pub grid_mesh: Mesh,
    pub phong_shader: Shader,
    pub font_shader: Shader,
    pub blit_shader: Shader,
    pub wireframe_shader: Shader,
    pub line_shader: Shader,
    pub unshaded_shader: Shader,
    pub skybox_shader: Shader,
    pub fira_font: Font,
}

impl Builtins {
    pub(crate) fn new(
        device: &Rc<Device>,
        storage: &mut Storage,
        framebuffer: &CoreFramebuffer,
        layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
    ) -> Result<Self> {
        // textures
        let white_texture = {
            let tex = CoreTexture::new(
                device,
                shader_images,
                vec![255, 255, 255, 255],
                Size::new(1, 1),
                ImageFormat::Rgba,
            );
            let shader_index = tex.shader_index();
            let (index, _) = storage.textures.add(tex);
            Texture::new(index, shader_index)
        };

        // materials
        let white_material = {
            let (index, updater) = storage.materials.add(CoreMaterial::new(device, layout));
            let mut mat = Material::new(index, updater);
            mat.set_phong_color([255, 255, 255]);
            mat.update();
            mat
        };

        // meshes
        let surface_mesh = create_surface(device, storage);
        let quad_mesh = create_quad(device, storage);
        let cube_mesh = create_cube(device, storage);
        let sphere_mesh = create_sphere(device, storage, 3);
        let grid_mesh = create_grid(device, storage, 50);

        // shaders
        let phong_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/phong.spirv"),
            )?);
            Shader::new(index)
        };

        let font_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/font.spirv"),
            )?);
            Shader::new(index)
        };

        let blit_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/blit.spirv"),
            )?);
            Shader::new(index)
        };

        let wireframe_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/wireframe.spirv"),
            )?);
            Shader::new(index)
        };

        let line_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/lines.spirv"),
            )?);
            Shader::new(index)
        };

        let unshaded_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/unshaded.spirv"),
            )?);
            Shader::new(index)
        };

        let skybox_shader = {
            let (index, _) = storage.shaders.add(CoreShader::from_spirv_bytes(
                device,
                framebuffer,
                layout,
                include_bytes!("../../shaders/skybox.spirv"),
            )?);
            Shader::new(index)
        };

        // fonts
        let fira_font = {
            let (index, _) = storage
                .fonts
                .add(CoreFont::fira_mono(device, shader_images));
            Font::new(index)
        };

        Ok(Self {
            white_texture,
            white_material,
            surface_mesh,
            quad_mesh,
            cube_mesh,
            sphere_mesh,
            grid_mesh,
            phong_shader,
            font_shader,
            blit_shader,
            wireframe_shader,
            line_shader,
            unshaded_shader,
            skybox_shader,
            fira_font,
        })
    }
}

fn create_surface(device: &Rc<Device>, storage: &mut Storage) -> Mesh {
    let (index, updater) = storage.meshes.add(CoreMesh::new(device));
    let mut mesh = Mesh::new(index, updater);

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
    mesh.update();

    mesh
}

fn create_quad(device: &Rc<Device>, storage: &mut Storage) -> Mesh {
    let (index, updater) = storage.meshes.add(CoreMesh::new(device));
    let mut mesh = Mesh::new(index, updater);

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
    mesh.update();

    mesh
}

fn create_cube(device: &Rc<Device>, storage: &mut Storage) -> Mesh {
    let top = create_rectangle(
        device,
        storage,
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    );

    let bottom = create_rectangle(
        device,
        storage,
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
    );

    let back = create_rectangle(
        device,
        storage,
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
    );

    let front = create_rectangle(
        device,
        storage,
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
    );

    let left = create_rectangle(
        device,
        storage,
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
    );

    let right = create_rectangle(
        device,
        storage,
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
    );

    let (index, updater) = storage.meshes.add(CoreMesh::new(device));

    Mesh::combine(index, updater, &[top, bottom, front, back, left, right])
}

fn create_grid(device: &Rc<Device>, storage: &mut Storage, size: u32) -> Mesh {
    let half = size as i32 / 2;
    let mut vertices = vec![];
    let mut colors = vec![];
    let mut indices = vec![];

    for x in -half..=half {
        let color = if x == 0 {
            Color::GREEN
        } else {
            Color::rgba_norm(0.5, 0.5, 0.5, 0.5)
        };
        let vc = vertices.len() as u16;
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
        let vc = vertices.len() as u16;
        vertices.extend(&[
            Vector3::new(half as f32, 0.0, z as f32),
            Vector3::new(-half as f32, 0.0, z as f32),
        ]);
        colors.extend(&[color, color]);
        indices.extend(&[vc, vc + 1]);
    }

    let (index, updater) = storage.meshes.add(CoreMesh::new(device));
    let mut mesh = Mesh::new(index, updater);
    mesh.vertices = vertices;
    mesh.colors = colors;
    mesh.indices = indices;
    mesh.update();
    mesh
}

fn create_rectangle<V: Into<Vector3>>(
    device: &Rc<Device>,
    storage: &mut Storage,
    p1: V,
    p2: V,
    p3: V,
    p4: V,
) -> Mesh {
    let (index, updater) = storage.meshes.add(CoreMesh::new(device));
    let mut mesh = Mesh::new(index, updater);

    mesh.vertices = vec![p1.into(), p2.into(), p3.into(), p4.into()];
    mesh.uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    mesh.indices = vec![0, 1, 2, 0, 2, 3];
    mesh.calculate_normals();

    mesh
}

fn create_sphere(device: &Rc<Device>, storage: &mut Storage, detail_level: u32) -> Mesh {
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

    let (index, updater) = storage.meshes.add(CoreMesh::new(device));
    let mut mesh = Mesh::new(index, updater);
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.uvs = uvs;
    mesh.calculate_normals();
    mesh.update();
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
