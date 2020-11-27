// Oliver Berzs
// https://github.com/oberzs/duku

mod model;
mod vertex;

use std::iter;

use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Rgbf;
use crate::device::Device;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::vk;

pub(crate) use vertex::Vertex;

pub use model::Model;
pub use model::ModelNode;

/// Shape collection for rendering
///
/// # Examples
///
/// ```ignore
/// // setup a triangle
/// let mut mesh = duku.create_mesh();
/// mesh.vertices = vec![
///     Vector3::new(-1.0, -1.0),
///     Vector3::new(0.0, 1.0),
///     Vector3::new(1.0, -1.0),
/// ];
/// mesh.indices = vec![0, 1, 2];
///
/// // render a triangle
/// target.draw_mesh(&mesh);
/// ```
pub struct Mesh {
    /// vertex positions
    pub vertices: Vec<Vector3>,
    /// vertex UV coordinates
    pub uvs: Vec<Vector2>,
    /// vertex normal directions
    pub normals: Vec<Vector3>,
    /// vertex tangent directions
    pub tangents: Vec<Vector3>,
    /// vertex colors
    pub colors: Vec<Rgbf>,
    /// vertex texture indices
    pub textures: Vec<u32>,
    /// vertex indices
    pub indices: Vec<u32>,

    vertex_buffer: Buffer<Vertex>,
    index_buffer: Buffer<u32>,
    index_count: usize,
}

impl Mesh {
    pub(crate) fn new(device: &Device) -> Self {
        let vertex_buffer = Buffer::dynamic(device, BufferUsage::Vertex, 1);
        let index_buffer = Buffer::dynamic(device, BufferUsage::Index, 3);

        Self {
            vertices: vec![Vector3::default(); 1],
            uvs: vec![Vector2::default(); 1],
            normals: vec![Vector3::default(); 1],
            tangents: vec![Vector3::default(); 1],
            colors: vec![Rgbf::gray(1.0); 1],
            textures: vec![0; 1],
            indices: vec![0; 3],
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
        result.update(device);
        result
    }

    /// Calculate vertex normals automatically
    /// smoothing the values to achieve smooth
    /// shading.
    ///
    /// Calls [calculate_tangents](crate::mesh::Mesh::calculate_tangents)
    /// automatically
    pub fn calculate_normals(&mut self) {
        self.normals = vec![Vector3::default(); self.vertices.len()];

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
        }

        // calculate tangents for the new normals
        self.calculate_tangents();
    }

    /// Calculate vertex tangents automatically
    /// smoothing the values to achieve smooth
    /// shading.
    pub fn calculate_tangents(&mut self) {
        self.tangents = vec![Vector3::default(); self.vertices.len()];

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
                let uv_a = self.uvs.get(a).copied().unwrap_or_default();
                let uv_b = self.uvs.get(b).copied().unwrap_or_default();
                let uv_c = self.uvs.get(c).copied().unwrap_or_default();

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
        }
    }

    pub(crate) fn update(&mut self, device: &Device) {
        let vertices: Vec<_> = self
            .vertices
            .iter()
            .zip(self.uvs.iter().chain(iter::repeat(&Vector2::default())))
            .zip(self.normals.iter().chain(iter::repeat(&Vector3::default())))
            .zip(
                self.tangents
                    .iter()
                    .chain(iter::repeat(&Vector3::default())),
            )
            .zip(self.colors.iter().chain(iter::repeat(&Rgbf::gray(1.0))))
            .zip(self.textures.iter().chain(iter::repeat(&0)))
            .map(|(((((pos, uv), normal), tangent), col), tex)| Vertex {
                in_local_position: *pos,
                in_normal: *normal,
                in_tangent: *tangent,
                in_uv: *uv,
                in_color: (*col).into(),
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
