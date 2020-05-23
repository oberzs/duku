mod vertex;

use ash::vk::Buffer as VkBuffer;
use std::cell::Cell;
use std::sync::Arc;
use tegne_math::Vector2;
use tegne_math::Vector3;

use crate::buffers::BufferType;
use crate::buffers::DynamicBuffer;
use crate::error::Result;
use crate::instance::Device;
pub(crate) use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    triangles: Vec<[u32; 3]>,
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    should_update_vertices: Cell<bool>,
    should_update_triangles: Cell<bool>,
    index_count: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MeshOptions<'slice> {
    pub vertices: &'slice [Vector3],
    pub uvs: &'slice [Vector2],
    pub normals: &'slice [Vector3],
    pub triangles: &'slice [[u32; 3]],
}

impl Mesh {
    pub(crate) fn new(device: &Arc<Device>, options: MeshOptions<'_>) -> Result<Self> {
        let vertices = options.vertices.to_vec();
        let triangles = options.triangles.to_vec();
        let vertex_count = vertices.len();
        let index_count = triangles.len() * 3;

        let vertex_buffer = DynamicBuffer::new::<Vertex>(device, vertex_count, BufferType::Vertex)?;
        let index_buffer = DynamicBuffer::new::<u32>(device, index_count, BufferType::Index)?;

        let mut uvs = vec![Vector2::default(); vertex_count];
        uvs[..options.uvs.len()].clone_from_slice(options.uvs);

        let mut normals = vec![Vector3::default(); vertex_count];
        normals[..options.normals.len()].clone_from_slice(options.normals);

        // calculate smooth normals
        if options.normals.is_empty() {
            for tri in options.triangles {
                let a = tri[0] as usize;
                let b = tri[1] as usize;
                let c = tri[2] as usize;
                let vtx_a = vertices[a];
                let vtx_b = vertices[b];
                let vtx_c = vertices[c];
                let normal = (vtx_b - vtx_a).cross(vtx_c - vtx_a);
                normals[a] += normal;
                normals[b] += normal;
                normals[c] += normal;
            }
            for norm in normals.iter_mut() {
                *norm = norm.unit();
            }
        }

        Ok(Self {
            vertices,
            uvs,
            normals,
            triangles,
            vertex_buffer,
            index_buffer,
            should_update_vertices: Cell::new(true),
            should_update_triangles: Cell::new(true),
            index_count: index_count as u32,
        })
    }

    pub fn set_vertices(&mut self, vertices: &[Vector3]) {
        self.vertices = vertices.to_owned();
        self.should_update_vertices.set(true);
    }

    pub fn set_uvs(&mut self, uvs: &[Vector2]) {
        self.uvs = uvs.to_owned();
        self.should_update_vertices.set(true);
    }

    pub fn set_normals(&mut self, normals: &[Vector3]) {
        self.normals = normals.to_owned();
        self.should_update_vertices.set(true);
    }

    pub fn set_triangles(&mut self, triangles: &[[u32; 3]]) {
        self.triangles = triangles.to_owned();
        self.should_update_triangles.set(true);
    }

    pub(crate) fn vk_vertex_buffer(&self) -> Result<VkBuffer> {
        if self.should_update_vertices.get() {
            let vertices = self
                .vertices
                .iter()
                .zip(self.uvs.iter())
                .zip(self.normals.iter())
                .map(|((pos, uv), normal)| Vertex {
                    pos: *pos,
                    uv: *uv,
                    norm: *normal,
                })
                .collect::<Vec<_>>();
            self.vertex_buffer.update_data(&vertices)?;
            self.should_update_vertices.set(false);
        }
        Ok(self.vertex_buffer.vk())
    }

    pub(crate) fn vk_index_buffer(&self) -> Result<VkBuffer> {
        if self.should_update_triangles.get() {
            let indices = self
                .triangles
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<u32>>();
            self.index_buffer.update_data(&indices)?;
            self.should_update_triangles.set(false);
        }
        Ok(self.index_buffer.vk())
    }

    pub(crate) fn index_count(&self) -> u32 {
        self.index_count
    }
}
