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
use crate::resource::Ref;
pub(crate) use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    colors: Vec<Color>,
    indices: Vec<u32>,
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    should_update_vertices: bool,
    should_update_indices: bool,
}

#[derive(Default, Debug, Clone)]
pub struct MeshOptions {
    pub vertices: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Color>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub(crate) fn new(device: &Arc<Device>, options: MeshOptions) -> Result<Self> {
        let MeshOptions {
            mut uvs,
            mut normals,
            mut colors,
            vertices,
            indices,
        } = options;

        let vertex_count = vertices.len();
        let index_count = indices.len();

        let vertex_buffer =
            DynamicBuffer::new::<Vertex>(device, BufferUsage::Vertex, vertex_count)?;
        let index_buffer = DynamicBuffer::new::<u32>(device, BufferUsage::Index, index_count)?;

        // check if has normals
        let no_normals = normals.is_empty();

        // fill in missing defaults for all vertices
        uvs.resize_with(vertex_count, || Vector2::ZERO);
        normals.resize_with(vertex_count, || Vector3::ZERO);
        colors.resize_with(vertex_count, || Color::WHITE);

        let mut mesh = Self {
            vertices,
            uvs,
            normals,
            colors,
            indices,
            vertex_buffer,
            index_buffer,
            should_update_vertices: true,
            should_update_indices: true,
        };

        // generate normals if user didn't specify
        if no_normals {
            mesh.calculate_normals();
        }

        Ok(mesh)
    }

    pub(crate) fn combine(device: &Arc<Device>, meshes: &[Self]) -> Result<Self> {
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
            offset = vertices.len() as u32;
        }
        Self::new(
            device,
            MeshOptions {
                vertices,
                normals,
                uvs,
                colors,
                indices,
            },
        )
    }

    pub(crate) fn duplicate(&self, device: &Arc<Device>) -> Result<Self> {
        Mesh::new(
            device,
            MeshOptions {
                vertices: self.vertices.clone(),
                indices: self.indices.clone(),
                uvs: self.uvs.clone(),
                normals: self.normals.clone(),
                colors: self.colors.clone(),
            },
        )
    }

    pub(crate) fn update_if_needed(&mut self) -> Result<()> {
        if self.should_update_vertices {
            let vertices = self
                .vertices
                .iter()
                .zip(self.uvs.iter())
                .zip(self.normals.iter())
                .zip(self.colors.iter())
                .map(|(((pos, uv), normal), col)| Vertex {
                    pos: *pos,
                    uv: *uv,
                    norm: *normal,
                    col: col.to_rgba_norm_vec(),
                })
                .collect::<Vec<_>>();
            self.vertex_buffer.update_data(&vertices)?;
            self.should_update_vertices = false;
        }
        if self.should_update_indices {
            self.index_buffer.update_data(&self.indices)?;
            self.should_update_indices = false;
        }
        Ok(())
    }

    pub(crate) fn vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer.handle()
    }

    pub(crate) fn index_buffer(&self) -> vk::Buffer {
        self.index_buffer.handle()
    }

    pub(crate) fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    pub(crate) fn set_vertices(&mut self, vertices: Vec<Vector3>) {
        self.vertices = vertices;
        self.should_update_vertices = true;
    }

    pub(crate) fn set_uvs(&mut self, uvs: Vec<Vector2>) {
        self.uvs = uvs;
        self.should_update_vertices = true;
    }

    pub(crate) fn set_normals(&mut self, normals: Vec<Vector3>) {
        self.normals = normals;
        self.should_update_vertices = true;
    }

    pub(crate) fn set_colors(&mut self, colors: Vec<Color>) {
        self.colors = colors;
        self.should_update_vertices = true;
    }

    pub(crate) fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
        self.should_update_indices = true;
    }

    fn calculate_normals(&mut self) {
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
            self.should_update_vertices = true;
        }
    }
}

impl Ref<Mesh> {
    pub fn calculate_normals(&self) {
        self.with(|m| m.calculate_normals());
    }

    pub fn set_vertices(&self, vertices: Vec<Vector3>) {
        self.with(|m| m.set_vertices(vertices));
    }

    pub fn set_uvs(&self, uvs: Vec<Vector2>) {
        self.with(|m| m.set_uvs(uvs));
    }

    pub fn set_normals(&self, normals: Vec<Vector3>) {
        self.with(|m| m.set_normals(normals));
    }

    pub fn set_colors(&self, colors: Vec<Color>) {
        self.with(|m| m.set_colors(colors));
    }

    pub fn set_indices(&self, indices: Vec<u32>) {
        self.with(|m| m.set_indices(indices));
    }

    pub fn vertices(&self) -> Vec<Vector3> {
        self.with(|m| m.vertices.clone())
    }

    pub fn uvs(&self) -> Vec<Vector2> {
        self.with(|m| m.uvs.clone())
    }

    pub fn normals(&self) -> Vec<Vector3> {
        self.with(|m| m.normals.clone())
    }

    pub fn colors(&self) -> Vec<Color> {
        self.with(|m| m.colors.clone())
    }

    pub fn indices(&self) -> Vec<u32> {
        self.with(|m| m.indices.clone())
    }
}
