// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Font - struct for a renderable font

mod fira_mono;

use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;
use crate::device::Device;
use crate::image::CoreTexture;
use crate::image::ImageFormat;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::CoreMesh;
use crate::mesh::MeshData;
use crate::pipeline::ImageUniform;
use crate::storage::Index;

// user facing framebuffer data
#[derive(Debug)]
pub struct Font {
    pub(crate) index: Index,
}

// data storage for a font
pub(crate) struct CoreFont {
    char_data: HashMap<char, CharData>,
    mesh: CoreMesh,
    _texture: CoreTexture,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct CharData {
    pub(crate) index_offset: usize,
    pub(crate) x_offset: f32,
    pub(crate) y_offset: f32,
    pub(crate) advance: f32,
}

impl Font {
    pub(crate) const fn new(index: Index) -> Self {
        Self { index }
    }
}

impl CoreFont {
    pub(crate) fn fira_mono(device: &Rc<Device>, uniform: &mut ImageUniform) -> Self {
        let atlas_width = fira_mono::ATLAS_WIDTH;
        let atlas_height = fira_mono::ATLAS_HEIGHT;
        let line_height = fira_mono::LINE_HEIGHT;

        let texture = CoreTexture::new(
            device,
            uniform,
            fira_mono::DATA.to_vec(),
            atlas_width,
            atlas_height,
            ImageFormat::Gray,
        );

        let mut char_data = HashMap::new();
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut uvs = vec![];
        let mut offset = 0;
        for (c, metrics) in fira_mono::metrics() {
            let u_min = metrics.x as f32 / atlas_width as f32;
            let v_min = metrics.y as f32 / atlas_height as f32;
            let u_max = u_min + (metrics.width as f32 / atlas_width as f32);
            let v_max = v_min + (metrics.height as f32 / atlas_height as f32);

            let width = metrics.width as f32 / line_height as f32;
            let height = metrics.height as f32 / line_height as f32;

            let x_offset = metrics.xo as f32 / line_height as f32;
            let y_offset = metrics.yo as f32 / line_height as f32;
            let advance = metrics.advance as f32 / line_height as f32;

            let o = vertices.len() as u16;

            vertices.extend(&[
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(width, 0.0, 0.0),
                Vector3::new(width, -height, 0.0),
                Vector3::new(0.0, -height, 0.0),
            ]);
            uvs.extend(&[
                Vector2::new(u_min, v_min),
                Vector2::new(u_max, v_min),
                Vector2::new(u_max, v_max),
                Vector2::new(u_min, v_max),
            ]);
            indices.extend(&[o, o + 1, o + 2, o, o + 2, o + 3]);

            char_data.insert(
                c,
                CharData {
                    index_offset: offset,
                    x_offset,
                    y_offset,
                    advance,
                },
            );
            offset += 6;
        }

        let vertex_count = vertices.len();
        let normals = vec![Vector3::ZERO; vertex_count];
        let colors = vec![Color::WHITE; vertex_count];
        let textures = vec![texture.image_index(); vertex_count];

        let mut mesh = CoreMesh::new(device);
        mesh.update(MeshData {
            textures,
            vertices,
            normals,
            colors,
            uvs,
            indices,
        });

        Self {
            char_data,
            mesh,
            _texture: texture,
        }
    }

    // pub(crate) const fn texture(&self) -> &CoreTexture {
    //     &self.texture
    // }

    pub(crate) const fn mesh(&self) -> &CoreMesh {
        &self.mesh
    }

    pub(crate) fn char_data(&self, c: char) -> CharData {
        match self.char_data.get(&c) {
            Some(data) => *data,
            None => *self.char_data.get(&'?').expect("bad default"),
        }
    }
}
