// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh - struct representing a renderable object

mod vertex;

use ash::vk;
use std::sync::mpsc::Sender;
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
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Color>,
    pub indices: Vec<u16>,

    pub(crate) index: Index,

    updater: Sender<(Index, MeshUpdateData)>,
}

// GPU data storage for a mesh
pub(crate) struct CoreMesh {
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    index_count: usize,
}

pub(crate) struct MeshUpdateData {
    pub(crate) vertices: Vec<Vector3>,
    pub(crate) normals: Vec<Vector3>,
    pub(crate) colors: Vec<Color>,
    pub(crate) uvs: Vec<Vector2>,
    pub(crate) indices: Vec<u16>,
}

impl Mesh {
    pub(crate) fn new(index: Index, updater: Sender<(Index, MeshUpdateData)>) -> Self {
        Self {
            vertices: vec![Vector3::ZERO; 1],
            normals: vec![Vector3::ZERO; 1],
            colors: vec![Color::WHITE; 1],
            uvs: vec![Vector2::ZERO; 1],
            indices: vec![0; 3],
            updater,
            index,
        }
    }

    pub(crate) fn combine(
        index: Index,
        updater: Sender<(Index, MeshUpdateData)>,
        meshes: &[Self],
    ) -> Self {
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

        let mut result = Self::new(index, updater);
        result.vertices = vertices;
        result.normals = normals;
        result.colors = colors;
        result.uvs = uvs;
        result.indices = indices;
        result.update();
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
    }

    pub fn update(&self) {
        let data = MeshUpdateData {
            vertices: self.vertices.clone(),
            normals: self.normals.clone(),
            colors: self.colors.clone(),
            uvs: self.uvs.clone(),
            indices: self.indices.clone(),
        };
        self.updater
            .send((self.index.clone(), data))
            .expect("bad receiver");
    }
}

impl CoreMesh {
    pub(crate) fn new(device: &Arc<Device>) -> Result<Self> {
        let vertex_buffer = DynamicBuffer::new::<Vertex>(device, BufferUsage::Vertex, 1)?;
        let index_buffer = DynamicBuffer::new::<u16>(device, BufferUsage::Index, 3)?;

        Ok(Self {
            index_count: 3,
            vertex_buffer,
            index_buffer,
        })
    }

    pub(crate) fn update(&mut self, data: MeshUpdateData) -> Result<()> {
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
        self.index_buffer.update_data(&data.indices)?;
        self.index_count = data.indices.len();

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
}
