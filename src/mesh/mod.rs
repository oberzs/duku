// Oliver Berzs
// https://github.com/oberzs/draw-it

// Mesh - struct representing a renderable object

mod vertex;

use std::iter;

use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Color;
use crate::device::Device;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::storage::Handle;
use crate::storage::Storage;
use crate::vk;

pub(crate) use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    tangents: Vec<Vector3>,
    colors: Vec<Color>,
    textures: Vec<u32>,
    indices: Vec<u32>,

    should_update: bool,

    vertex_buffer: Buffer<Vertex>,
    index_buffer: Buffer<u32>,
    index_count: usize,
}

pub struct MeshBuilder<'s> {
    pub(crate) storage: &'s mut Storage,
    pub(crate) mesh: Mesh,
}

impl Mesh {
    pub(crate) fn new(device: &Device) -> Self {
        let vertex_buffer = Buffer::dynamic(device, BufferUsage::Vertex, 1);
        let index_buffer = Buffer::dynamic(device, BufferUsage::Index, 3);

        Self {
            vertices: vec![Vector3::ZERO; 1],
            uvs: vec![Vector2::ZERO; 1],
            normals: vec![Vector3::ZERO; 1],
            tangents: vec![Vector3::ZERO; 1],
            colors: vec![Color::WHITE; 1],
            textures: vec![0; 1],
            indices: vec![0; 3],
            should_update: true,
            index_count: 3,
            vertex_buffer,
            index_buffer,
        }
    }

    pub(crate) fn combine(device: &Device, meshes: &[&Self]) -> Self {
        let mut offset = 0;
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut tangents = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];
        let mut textures = vec![];

        for mesh in meshes {
            indices.extend(mesh.indices.iter().map(|t| t + offset));
            vertices.extend(&mesh.vertices);
            normals.extend(&mesh.normals);
            tangents.extend(&mesh.tangents);
            uvs.extend(&mesh.uvs);
            colors.extend(&mesh.colors);
            textures.extend(&mesh.textures);
            offset = vertices.len() as u32;
        }

        let mut result = Self::new(device);
        result.vertices = vertices;
        result.normals = normals;
        result.tangents = tangents;
        result.colors = colors;
        result.uvs = uvs;
        result.textures = textures;
        result.indices = indices;
        result.should_update = true;
        result.update_if_needed(device);
        result
    }

    pub fn calculate_normals(&mut self) {
        self.normals = vec![Vector3::ZERO; self.vertices.len()];

        if self.indices.len() % 3 == 0 {
            for tri in self.indices.chunks(3) {
                let a = tri[0] as usize;
                let b = tri[1] as usize;
                let c = tri[2] as usize;

                // get vertices
                let vtx_a = self.vertices[a];
                let vtx_b = self.vertices[b];
                let vtx_c = self.vertices[c];

                // calculate normal
                let normal = (vtx_b - vtx_a).cross(vtx_c - vtx_a);
                self.normals[a] += normal;
                self.normals[b] += normal;
                self.normals[c] += normal;
            }
            for norm in &mut self.normals {
                *norm = norm.unit();
            }

            self.should_update = true;
        }

        // calculate tangents for the new normals
        self.calculate_tangents();
    }

    pub fn calculate_tangents(&mut self) {
        self.tangents = vec![Vector3::ZERO; self.vertices.len()];

        if self.indices.len() % 3 == 0 {
            for tri in self.indices.chunks(3) {
                let a = tri[0] as usize;
                let b = tri[1] as usize;
                let c = tri[2] as usize;

                // get vertices
                let vtx_a = self.vertices[a];
                let vtx_b = self.vertices[b];
                let vtx_c = self.vertices[c];

                // get uvs
                let uv_a = self.uvs.get(a).copied().unwrap_or(Vector2::ZERO);
                let uv_b = self.uvs.get(b).copied().unwrap_or(Vector2::ZERO);
                let uv_c = self.uvs.get(c).copied().unwrap_or(Vector2::ZERO);

                // calculate tangent
                let edge_1 = vtx_b - vtx_a;
                let edge_2 = vtx_c - vtx_a;
                let duv_1 = uv_b - uv_a;
                let duv_2 = uv_c - uv_a;
                let f = 1.0 / (duv_1.x * duv_2.y - duv_2.x * duv_1.y);
                let tangent = Vector3::new(
                    duv_2.y * edge_1.x - duv_1.y * edge_2.x,
                    duv_2.y * edge_1.y - duv_1.y * edge_2.y,
                    duv_2.y * edge_1.z - duv_1.y * edge_2.z,
                ) * f;
                self.tangents[a] += tangent;
                self.tangents[b] += tangent;
                self.tangents[c] += tangent;
            }
            for tan in &mut self.tangents {
                *tan = tan.unit();
            }

            self.should_update = true;
        }
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vector3>) {
        self.vertices = vertices;
        self.should_update = true;
    }

    pub fn set_normals(&mut self, normals: Vec<Vector3>) {
        self.normals = normals;
        self.should_update = true;
    }

    pub fn set_tangents(&mut self, tangents: Vec<Vector3>) {
        self.tangents = tangents;
        self.should_update = true;
    }

    pub fn set_colors(&mut self, colors: Vec<Color>) {
        self.colors = colors;
        self.should_update = true;
    }

    pub fn set_uvs(&mut self, uvs: Vec<Vector2>) {
        self.uvs = uvs;
        self.should_update = true;
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
        self.should_update = true;
    }

    pub fn vertices(&self) -> &[Vector3] {
        &self.vertices
    }

    pub fn normals(&self) -> &[Vector3] {
        &self.normals
    }

    pub fn colors(&self) -> &[Color] {
        &self.colors
    }

    pub fn uvs(&self) -> &[Vector2] {
        &self.uvs
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub(crate) fn set_textures(&mut self, textures: Vec<u32>) {
        self.textures = textures;
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device) {
        if self.should_update {
            let vertices: Vec<_> = self
                .vertices
                .iter()
                .zip(self.uvs.iter().chain(iter::repeat(&Vector2::ZERO)))
                .zip(self.normals.iter().chain(iter::repeat(&Vector3::ZERO)))
                .zip(self.tangents.iter().chain(iter::repeat(&Vector3::ZERO)))
                .zip(self.colors.iter().chain(iter::repeat(&Color::WHITE)))
                .zip(self.textures.iter().chain(iter::repeat(&0)))
                .map(|(((((pos, uv), normal), tangent), col), tex)| Vertex {
                    in_local_position: *pos,
                    in_normal: *normal,
                    in_tangent: *tangent,
                    in_uv: *uv,
                    in_color: Vector4::from(*col),
                    in_texture: *tex,
                })
                .collect();

            // resize buffers if needed
            if vertices.len() > self.vertex_buffer.len() {
                self.vertex_buffer.resize(device, vertices.len());
            }
            if self.indices.len() > self.index_buffer.len() {
                self.index_buffer.resize(device, self.indices.len());
            }

            if !vertices.is_empty() && !self.indices.is_empty() {
                self.vertex_buffer.copy_from_data(&vertices);
                self.index_buffer.copy_from_data(&self.indices);
            }
            self.index_count = self.indices.len();

            self.should_update = false;
        }
    }

    pub(crate) fn vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer.handle()
    }

    pub(crate) fn index_buffer(&self) -> vk::Buffer {
        self.index_buffer.handle()
    }

    pub(crate) const fn index_count(&self) -> usize {
        self.index_count
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.vertex_buffer.destroy(device);
        self.index_buffer.destroy(device);
    }
}

impl MeshBuilder<'_> {
    pub fn vertices(mut self, vertices: Vec<Vector3>) -> Self {
        self.mesh.set_vertices(vertices);
        self
    }

    pub fn normals(mut self, normals: Vec<Vector3>) -> Self {
        self.mesh.set_normals(normals);
        self
    }

    pub fn tangents(mut self, tangents: Vec<Vector3>) -> Self {
        self.mesh.set_tangents(tangents);
        self
    }

    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.mesh.set_colors(colors);
        self
    }

    pub fn uvs(mut self, uvs: Vec<Vector2>) -> Self {
        self.mesh.set_uvs(uvs);
        self
    }

    pub fn indices(mut self, indices: Vec<u32>) -> Self {
        self.mesh.set_indices(indices);
        self
    }

    pub fn calculated_normals(mut self) -> Self {
        self.mesh.calculate_normals();
        self
    }

    pub fn calculated_tangents(mut self) -> Self {
        self.mesh.calculate_tangents();
        self
    }

    pub fn build(self) -> Handle<Mesh> {
        self.storage.add_mesh(self.mesh)
    }
}
