// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh - struct representing a renderable object

mod vertex;

use ash::vk;
use std::sync::Arc;

use crate::buffer::BufferUsage;
use crate::buffer::DynamicBuffer;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::resource::Index;
pub(crate) use vertex::Vertex;

// user facing Mesh data
#[derive(Debug, Clone)]
pub struct Mesh {
    pub(crate) index: Index,

    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    colors: Vec<Color>,
    indices: Vec<u16>,
}

// GPU data storage for a mesh
pub(crate) struct CoreMesh {
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    index_count: usize,
    version: u32,
}

pub(crate) struct MeshUpdateData<'data> {
    pub(crate) vertices: &'data [Vector3],
    pub(crate) normals: &'data [Vector3],
    pub(crate) colors: &'data [Color],
    pub(crate) uvs: &'data [Vector2],
    pub(crate) indices: &'data [u16],
}

impl Mesh {
    pub(crate) fn new(index: Index) -> Self {
        Self {
            vertices: vec![Vector3::ZERO; 1],
            normals: vec![Vector3::ZERO; 1],
            colors: vec![Color::WHITE; 1],
            uvs: vec![Vector2::ZERO; 1],
            indices: vec![0; 3],
            index,
        }
    }

    pub(crate) fn combine(index: Index, meshes: &[Self]) -> Self {
        let mut offset = 0;
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];

        for mesh in meshes {
            indices.extend(mesh.indices.iter().map(|t| t + offset));
            vertices.extend(&mesh.vertices);
            normals.extend(&mesh.normals);
            uvs.extend(&mesh.uvs);
            colors.extend(&mesh.colors);
            offset = vertices.len() as u16;
        }

        let mut result = Self::new(index);
        result.set_vertices(vertices);
        result.set_normals(normals);
        result.set_colors(colors);
        result.set_uvs(uvs);
        result.set_indices(indices);
        result
    }

    pub fn calculate_normals(&mut self) {
        self.normals = vec![Vector3::ZERO; self.vertices.len()];
        if self.indices.len() % 3 == 0 {
            for tri in self.indices.chunks(3) {
                let a = tri[0] as usize;
                let b = tri[1] as usize;
                let c = tri[2] as usize;
                let vtx_a = self.vertices[a];
                let vtx_b = self.vertices[b];
                let vtx_c = self.vertices[c];
                let normal = (vtx_b - vtx_a).cross(vtx_c - vtx_a);
                self.normals[a] += normal;
                self.normals[b] += normal;
                self.normals[c] += normal;
            }
            for norm in self.normals.iter_mut() {
                *norm = norm.unit();
            }
        }
        self.index.bump();
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vector3>) {
        self.vertices = vertices;
        self.index.bump();
    }

    pub fn set_normals(&mut self, normals: Vec<Vector3>) {
        self.normals = normals;
        self.index.bump();
    }

    pub fn set_colors(&mut self, colors: Vec<Color>) {
        self.colors = colors;
        self.index.bump();
    }

    pub fn set_uvs(&mut self, uvs: Vec<Vector2>) {
        self.uvs = uvs;
        self.index.bump();
    }

    pub fn set_indices(&mut self, indices: Vec<u16>) {
        self.indices = indices;
        self.index.bump();
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

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }
}

impl CoreMesh {
    pub(crate) fn new(device: &Arc<Device>) -> Result<Self> {
        let vertex_buffer = DynamicBuffer::new::<Vertex>(device, BufferUsage::Vertex, 1)?;
        let index_buffer = DynamicBuffer::new::<u16>(device, BufferUsage::Index, 3)?;

        Ok(Self {
            index_count: 3,
            version: 0,
            vertex_buffer,
            index_buffer,
        })
    }

    pub(crate) fn update_if_needed(
        &mut self,
        data: MeshUpdateData<'_>,
        version: u32,
    ) -> Result<()> {
        // check if data has been updated
        if self.version != version {
            let vertices = data
                .vertices
                .iter()
                .zip(data.uvs.iter().chain([Vector2::ZERO].iter().cycle()))
                .zip(data.normals.iter().chain([Vector3::ZERO].iter().cycle()))
                .zip(data.colors.iter().chain([Color::WHITE].iter().cycle()))
                .map(|(((pos, uv), normal), col)| Vertex {
                    pos: *pos,
                    uv: *uv,
                    norm: *normal,
                    col: col.to_rgba_norm_vec(),
                })
                .collect::<Vec<_>>();

            self.vertex_buffer.update_data(&vertices)?;
            self.index_buffer.update_data(data.indices)?;
            self.index_count = data.indices.len();
            self.version = version;
        }

        Ok(())
    }

    pub(crate) fn vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer.handle()
    }

    pub(crate) fn index_buffer(&self) -> vk::Buffer {
        self.index_buffer.handle()
    }

    pub(crate) fn index_count(&self) -> usize {
        self.index_count
    }

    pub(crate) fn version(&self) -> u32 {
        self.version
    }
}
