use ash::vk::Buffer as VkBuffer;
use log::debug;
use log::info;
use std::cell::Cell;
use std::rc::Rc;
use std::rc::Weak;
use tegne_math::Vector2;
use tegne_math::Vector3;

use super::Vertex;
use crate::buffer::Buffer;
use crate::buffer::BufferType;
use crate::buffer::DynamicBuffer;
use crate::buffer::FixedBuffer;
use crate::instance::Device;
use crate::utils::error;
use crate::utils::OrError;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    vertex_buffer: DynamicBuffer,
    index_buffer: FixedBuffer,
    should_update: Cell<bool>,
    drawn_triangles: u32,
}

pub struct MeshBuilder {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    triangles: Vec<u32>,
    device: Weak<Device>,
}

impl Mesh {
    pub(crate) fn builder(device: &Rc<Device>) -> MeshBuilder {
        MeshBuilder {
            vertices: vec![],
            uvs: vec![],
            normals: vec![],
            triangles: vec![],
            device: Rc::downgrade(device),
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

    pub fn vertices(&self) -> &[Vector3] {
        &self.vertices
    }

    pub(crate) fn uvs(&self) -> &[Vector2] {
        &self.uvs
    }

    pub(crate) fn normals(&self) -> &[Vector3] {
        &self.normals
    }

    pub(crate) fn drawn_triangles(&self) -> u32 {
        self.drawn_triangles as u32
    }
}

impl MeshBuilder {
    pub fn build(self) -> Mesh {
        debug!("build mesh");
        let vertex_buffer =
            DynamicBuffer::new::<Vertex>(&self.device(), self.vertices.len(), BufferType::Vertex);
        let index_buffer =
            FixedBuffer::new::<u32>(&self.device(), &self.triangles, BufferType::Index);

        let size = if !self.vertices.is_empty() {
            self.vertices.len()
        } else if !self.uvs.is_empty() {
            self.uvs.len()
        } else if !self.normals.is_empty() {
            self.normals.len()
        } else {
            0
        };

        let vertices = if self.vertices.is_empty() {
            vec![Vector3::default(); size]
        } else {
            self.vertices
        };

        let uvs = if self.uvs.is_empty() {
            vec![Vector2::default(); size]
        } else {
            self.uvs
        };

        let normals = if self.normals.is_empty() {
            vec![Vector3::default(); size]
        } else {
            self.normals
        };

        let mesh = Mesh {
            vertices,
            uvs,
            normals,
            vertex_buffer,
            index_buffer,
            should_update: Cell::new(true),
            drawn_triangles: self.triangles.len() as u32 / 3,
        };
        info!("mesh built");
        mesh
    }

    pub fn with_vertices(mut self, vertices: &[Vector3]) -> Self {
        if (!self.uvs.is_empty() && self.uvs.len() != vertices.len())
            || !self.normals.is_empty() && self.normals.len() != vertices.len()
        {
            error("wrong amount of vertices");
        }
        self.vertices = vertices.to_owned();
        self
    }

    pub fn with_uvs(mut self, uvs: &[Vector2]) -> Self {
        if (!self.vertices.is_empty() && self.vertices.len() != uvs.len())
            || !self.normals.is_empty() && self.normals.len() != uvs.len()
        {
            error("wrong amount of uvs");
        }
        self.uvs = uvs.to_owned();
        self
    }

    pub fn with_normals(mut self, normals: &[Vector3]) -> Self {
        if (!self.uvs.is_empty() && self.uvs.len() > normals.len())
            || !self.vertices.is_empty() && self.vertices.len() > normals.len()
        {
            error("wrong amount of normals");
        }
        self.normals = normals.to_owned();
        self
    }

    pub fn with_triangles(mut self, triangles: &[u32]) -> Self {
        self.triangles = triangles.to_owned();
        self
    }

    pub fn with_smooth_normals(mut self) -> Self {
        self.normals = vec![Vector3::default(); self.vertices.len()];

        for tri in self.triangles.chunks(3) {
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

        self
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}
