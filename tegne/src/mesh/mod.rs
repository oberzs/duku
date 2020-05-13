mod vertex;

use ash::vk::Buffer as VkBuffer;
use std::cell::Cell;
use std::rc::Rc;
use tegne_math::Vector2;
use tegne_math::Vector3;

use crate::buffer::Buffer;
use crate::buffer::BufferType;
use crate::buffer::DynamicBuffer;
use crate::buffer::FixedBuffer;
use crate::instance::Device;
use crate::utils::error;
pub(crate) use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    vertex_buffer: DynamicBuffer,
    index_buffer: FixedBuffer,
    should_update: Cell<bool>,
    drawn_triangles: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MeshOptions<'slice> {
    pub vertices: &'slice [Vector3],
    pub uvs: &'slice [Vector2],
    pub normals: &'slice [Vector3],
    pub triangles: &'slice [u32],
}

impl Mesh {
    pub(crate) fn new(device: &Rc<Device>, options: MeshOptions<'_>) -> Self {
        if options.vertices.is_empty() {
            error("no vertices in mesh");
        }
        if options.triangles.is_empty() {
            error("no triangles in mesh");
        }

        let vertex_count = options.vertices.len();
        let index_count = options.triangles.len();

        if options.uvs.len() > vertex_count {
            error("too many uvs");
        }
        if options.normals.len() > vertex_count {
            error("too many normals");
        }

        let vertex_buffer = DynamicBuffer::new::<Vertex>(device, vertex_count, BufferType::Vertex);
        let index_buffer = FixedBuffer::new::<u32>(device, options.triangles, BufferType::Index);

        let vertices = options.vertices.to_vec();

        let mut uvs = vec![Vector2::default(); vertex_count];
        uvs[..options.uvs.len()].clone_from_slice(options.uvs);

        let mut normals = vec![Vector3::default(); vertex_count];
        normals[..options.normals.len()].clone_from_slice(options.normals);

        // calculate smooth normals
        if options.normals.is_empty() {
            for tri in options.triangles.chunks(3) {
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

        Self {
            vertices,
            uvs,
            normals,
            vertex_buffer,
            index_buffer,
            should_update: Cell::new(true),
            drawn_triangles: index_count as u32 / 3,
        }
    }

    pub fn set_vertices(&mut self, vertices: &[Vector3]) {
        self.vertices = vertices.to_owned();
        self.should_update.set(true);
    }

    pub fn set_uvs(&mut self, uvs: &[Vector2]) {
        self.uvs = uvs.to_owned();
        self.should_update.set(true);
    }

    pub fn set_normals(&mut self, normals: &[Vector3]) {
        self.normals = normals.to_owned();
        self.should_update.set(true);
    }

    pub fn set_drawn_triangles(&mut self, count: u32) {
        self.drawn_triangles = count;
    }

    pub(crate) fn vk_vertex_buffer(&self) -> VkBuffer {
        if self.should_update.get() {
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
            self.vertex_buffer.update_data(&vertices);
            self.should_update.set(false);
        }
        self.vertex_buffer.vk_buffer()
    }

    pub(crate) fn vk_index_buffer(&self) -> VkBuffer {
        self.index_buffer.vk_buffer()
    }

    pub(crate) fn drawn_triangles(&self) -> u32 {
        self.drawn_triangles
    }
}
