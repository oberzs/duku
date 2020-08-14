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
use crate::resource::hash;
use crate::resource::Index;
pub(crate) use vertex::Vertex;

// user facing Mesh data
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Color>,
    pub indices: Vec<u16>,

    pub(crate) index: Index,
}

// GPU data storage for a mesh
pub(crate) struct CoreMesh {
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    index_count: usize,
    hash: u64,
}

pub(crate) struct MeshUpdateData<'data> {
    pub(crate) vertices: &'data [Vector3],
    pub(crate) normals: &'data [Vector3],
    pub(crate) colors: &'data [Color],
    pub(crate) uvs: &'data [Vector2],
    pub(crate) indices: &'data [u16],
}

#[derive(Default, Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Color>,
    pub indices: Vec<u16>,
}

impl CoreMesh {
    pub(crate) fn new(
        device: &Arc<Device>,
        vertex_count: usize,
        index_count: usize,
    ) -> Result<Self> {
        let vertex_buffer =
            DynamicBuffer::new::<Vertex>(device, BufferUsage::Vertex, vertex_count)?;
        let index_buffer = DynamicBuffer::new::<u16>(device, BufferUsage::Index, index_count)?;

        Ok(Self {
            hash: 0,
            vertex_buffer,
            index_buffer,
            index_count,
        })
    }

    pub(crate) fn update_if_needed(&mut self, data: MeshUpdateData<'_>) -> Result<()> {
        let hash = hash::adler32(data.vertices) as u64
            + hash::adler32(data.normals) as u64
            + hash::adler32(data.colors) as u64
            + hash::adler32(data.uvs) as u64
            + hash::adler32(data.indices) as u64;

        // check if data has been updated
        if self.hash != hash {
            let vertices = data
                .vertices
                .iter()
                .zip(data.uvs.iter())
                .zip(data.normals.iter())
                .zip(data.colors.iter())
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
            self.hash = hash;
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
}

impl Mesh {
    pub(crate) fn new(index: Index, data: MeshData) -> Result<Self> {
        let MeshData {
            mut uvs,
            mut normals,
            mut colors,
            vertices,
            indices,
        } = data;

        // check if has normals
        let no_normals = normals.is_empty();

        // fill in missing defaults for all vertices
        let vertex_count = vertices.len();
        uvs.resize_with(vertex_count, || Vector2::ZERO);
        normals.resize_with(vertex_count, || Vector3::ZERO);
        colors.resize_with(vertex_count, || Color::WHITE);

        let mut mesh = Self {
            vertices,
            indices,
            normals,
            colors,
            index,
            uvs,
        };

        // generate normals if user didn't specify
        if no_normals {
            mesh.calculate_normals();
        }

        Ok(mesh)
    }

    pub(crate) fn combine(index: Index, meshes: &[Self]) -> Result<Self> {
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

        Self::new(
            index,
            MeshData {
                vertices,
                normals,
                uvs,
                colors,
                indices,
            },
        )
    }

    pub fn calculate_normals(&mut self) {
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
}
