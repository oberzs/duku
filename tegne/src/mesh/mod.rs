// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh - struct representing a renderable object

mod vertex;

use ash::vk;
use std::sync::Arc;

use crate::buffer::BufferUsage;
use crate::buffer::DynamicBuffer;
use crate::color::colors;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::math::Vector2;
use crate::math::Vector3;
pub(crate) use vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vector3>,
    uvs: Vec<Vector2>,
    normals: Vec<Vector3>,
    colors: Vec<Color>,
    triangles: Vec<[u32; 3]>,
    vertex_buffer: DynamicBuffer,
    index_buffer: DynamicBuffer,
    should_update_vertices: bool,
    should_update_triangles: bool,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MeshOptions<'slice> {
    pub vertices: &'slice [Vector3],
    pub uvs: &'slice [Vector2],
    pub normals: &'slice [Vector3],
    pub colors: &'slice [Color],
    pub triangles: &'slice [[u32; 3]],
}

impl Mesh {
    pub(crate) fn new(device: &Arc<Device>, options: MeshOptions<'_>) -> Result<Self> {
        let vertices = options.vertices.to_vec();
        let triangles = options.triangles.to_vec();
        let vertex_count = vertices.len();
        let index_count = triangles.len() * 3;

        let vertex_buffer =
            DynamicBuffer::new::<Vertex>(device, BufferUsage::Vertex, vertex_count)?;
        let index_buffer = DynamicBuffer::new::<u32>(device, BufferUsage::Index, index_count)?;

        // fill in missing default UVs for all vertices
        let mut uvs = vec![Vector2::default(); vertex_count];
        uvs[..options.uvs.len()].clone_from_slice(options.uvs);

        // fill in missing default normals for all vertices
        let mut normals = vec![Vector3::default(); vertex_count];
        normals[..options.normals.len()].clone_from_slice(options.normals);

        // fill in missing default colors for all vertices
        let mut colors = vec![colors::WHITE; vertex_count];
        colors[..options.colors.len()].clone_from_slice(options.colors);

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
            colors,
            triangles,
            vertex_buffer,
            index_buffer,
            should_update_vertices: true,
            should_update_triangles: true,
        })
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
        if self.should_update_triangles {
            let indices = self
                .triangles
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<u32>>();
            self.index_buffer.update_data(&indices)?;
            self.should_update_triangles = false;
        }
        Ok(())
    }

    pub fn set_vertices(&mut self, vertices: &[Vector3]) {
        self.vertices = vertices.to_owned();
        self.should_update_vertices = true;
    }

    pub fn set_uvs(&mut self, uvs: &[Vector2]) {
        self.uvs = uvs.to_owned();
        self.should_update_vertices = true;
    }

    pub fn set_normals(&mut self, normals: &[Vector3]) {
        self.normals = normals.to_owned();
        self.should_update_vertices = true;
    }

    pub fn set_colors(&mut self, colors: &[Color]) {
        self.colors = colors.to_owned();
        self.should_update_vertices = true;
    }

    pub fn set_triangles(&mut self, triangles: &[[u32; 3]]) {
        self.triangles = triangles.to_owned();
        self.should_update_triangles = true;
    }

    pub fn vertices(&self) -> Vec<Vector3> {
        self.vertices.clone()
    }

    pub fn uvs(&self) -> Vec<Vector2> {
        self.uvs.clone()
    }

    pub fn normals(&self) -> Vec<Vector3> {
        self.normals.clone()
    }

    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    pub fn triangles(&self) -> Vec<[u32; 3]> {
        self.triangles.clone()
    }

    pub(crate) fn vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer.handle()
    }

    pub(crate) fn index_buffer(&self) -> vk::Buffer {
        self.index_buffer.handle()
    }

    pub(crate) fn index_count(&self) -> u32 {
        self.triangles.len() as u32 * 3
    }
}
