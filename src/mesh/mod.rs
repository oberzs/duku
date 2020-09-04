// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh - struct representing a renderable object

mod vertex;

use std::rc::Rc;
use std::sync::mpsc::Sender;

use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Color;
use crate::device::Device;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::storage::Index;
use crate::vk;
pub(crate) use vertex::Vertex;

// user facing Mesh data
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Color>,
    pub textures: Vec<i32>,
    pub indices: Vec<u16>,

    pub(crate) index: Index,

    updater: Sender<(Index, MeshData)>,
}

pub struct MeshBuilder {
    mesh: Mesh,
}

// GPU data storage for a mesh
pub(crate) struct CoreMesh {
    vertex_buffer: Buffer<Vertex>,
    index_buffer: Buffer<u16>,
}

pub(crate) struct MeshData {
    pub(crate) vertices: Vec<Vector3>,
    pub(crate) normals: Vec<Vector3>,
    pub(crate) colors: Vec<Color>,
    pub(crate) uvs: Vec<Vector2>,
    pub(crate) textures: Vec<i32>,
    pub(crate) indices: Vec<u16>,
}

impl Mesh {
    pub(crate) fn new(index: Index, updater: Sender<(Index, MeshData)>) -> Self {
        Self {
            vertices: vec![Vector3::ZERO; 1],
            normals: vec![Vector3::ZERO; 1],
            colors: vec![Color::WHITE; 1],
            uvs: vec![Vector2::ZERO; 1],
            textures: vec![0; 1],
            indices: vec![0; 3],
            updater,
            index,
        }
    }

    pub(crate) fn combine(
        index: Index,
        updater: Sender<(Index, MeshData)>,
        meshes: &[Self],
    ) -> Self {
        let mut offset = 0;
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];
        let mut textures = vec![];

        for mesh in meshes {
            indices.extend(mesh.indices.iter().map(|t| t + offset));
            vertices.extend(&mesh.vertices);
            normals.extend(&mesh.normals);
            uvs.extend(&mesh.uvs);
            colors.extend(&mesh.colors);
            textures.extend(&mesh.textures);
            offset = vertices.len() as u16;
        }

        let mut result = Self::new(index, updater);
        result.vertices = vertices;
        result.normals = normals;
        result.colors = colors;
        result.uvs = uvs;
        result.textures = textures;
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
            for norm in &mut self.normals {
                *norm = norm.unit();
            }
        }
    }

    pub fn update(&self) {
        let data = MeshData {
            vertices: self.vertices.clone(),
            normals: self.normals.clone(),
            colors: self.colors.clone(),
            uvs: self.uvs.clone(),
            textures: self.textures.clone(),
            indices: self.indices.clone(),
        };
        self.updater
            .send((self.index.clone(), data))
            .expect("bad receiver");
    }
}

impl MeshBuilder {
    pub(crate) fn new(mesh: Mesh) -> Self {
        Self { mesh }
    }

    pub fn vertices(mut self, vertices: Vec<Vector3>) -> Self {
        self.mesh.vertices = vertices;
        self
    }

    pub fn normals(mut self, normals: Vec<Vector3>) -> Self {
        self.mesh.normals = normals;
        self
    }

    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.mesh.colors = colors;
        self
    }

    pub fn uvs(mut self, uvs: Vec<Vector2>) -> Self {
        self.mesh.uvs = uvs;
        self
    }

    pub fn indices(mut self, indices: Vec<u16>) -> Self {
        self.mesh.indices = indices;
        self
    }

    pub fn calculated_normals(mut self) -> Self {
        self.mesh.calculate_normals();
        self
    }

    pub fn build(self) -> Mesh {
        self.mesh.update();
        self.mesh
    }
}

impl CoreMesh {
    pub(crate) fn new(device: &Rc<Device>) -> Self {
        let vertex_buffer = Buffer::dynamic(device, BufferUsage::Vertex, 1);
        let index_buffer = Buffer::dynamic(device, BufferUsage::Index, 3);

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub(crate) fn update(&mut self, data: MeshData) {
        let vertices: Vec<_> = data
            .vertices
            .iter()
            .zip(data.uvs.iter().chain([Vector2::ZERO].iter().cycle()))
            .zip(data.normals.iter().chain([Vector3::ZERO].iter().cycle()))
            .zip(data.colors.iter().chain([Color::WHITE].iter().cycle()))
            .zip(data.textures.iter().chain([0].iter().cycle()))
            .map(|((((pos, uv), normal), col), tex)| Vertex {
                pos: *pos,
                uv: *uv,
                norm: *normal,
                col: col.to_rgba_norm_vec(),
                tex: *tex,
            })
            .collect();

        // resize buffers if needed
        if vertices.len() > self.vertex_buffer.len() {
            self.vertex_buffer.resize(vertices.len());
        }
        if data.indices.len() > self.index_buffer.len() {
            self.index_buffer.resize(data.indices.len());
        }

        self.vertex_buffer.copy_from_data(&vertices);
        self.index_buffer.copy_from_data(&data.indices);
    }

    pub(crate) fn vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer.handle()
    }

    pub(crate) fn index_buffer(&self) -> vk::Buffer {
        self.index_buffer.handle()
    }

    pub(crate) fn index_count(&self) -> usize {
        self.index_buffer.len()
    }
}
