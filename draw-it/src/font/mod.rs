// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Font - struct for a renderable SDF font

mod format;

use std::collections::HashMap;
use std::sync::Arc;

use crate::device::Device;
use crate::error::Result;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::ImageUniform;
use crate::resource::Ref;
use crate::resource::ResourceManager;
use format::FontFile;

pub struct Font {
    texture: Ref<Texture>,
    _margin: f32,
    char_data: HashMap<char, CharData>,
}

struct CharData {
    mesh: Ref<Mesh>,
    advance: f32,
    bearing: f32,
}

impl Font {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        resources: &mut ResourceManager,
        source: &[u8],
    ) -> Result<Self> {
        let data: FontFile = bincode::deserialize(source)?;

        // create font atlas texture
        let texture = resources.add_texture(Texture::new(
            device,
            uniform,
            TextureOptions {
                data: &data.atlas,
                width: data.atlas_size,
                height: data.atlas_size,
                ..Default::default()
            },
        )?);

        let margin = data.margin as f32 / data.sdf_size as f32;

        // generate mesh for each character in atlas
        let mut char_data = HashMap::new();
        for (c, metrics) in data.char_metrics {
            // UV relative metrics
            let size_norm = data.sdf_size as f32 / data.atlas_size as f32;
            let u_min = metrics.x as f32 / data.atlas_size as f32;
            let v_min = metrics.y as f32 / data.atlas_size as f32;
            let u_max = u_min + size_norm;
            let v_max = v_min + size_norm;

            // vertex relative metrics
            let advance = metrics.advance as f32 / data.sdf_size as f32;
            let bearing = metrics.bearing as f32 / data.sdf_size as f32;

            let vertices = &[
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
            ];

            let uvs = &[
                Vector2::new(u_min, v_max),
                Vector2::new(u_max, v_max),
                Vector2::new(u_min, v_min),
                Vector2::new(u_max, v_min),
            ];

            let triangles = &[[0, 2, 3], [0, 3, 1]];

            let mesh = resources.add_mesh(Mesh::new(
                device,
                MeshOptions {
                    vertices,
                    triangles,
                    uvs,
                    ..Default::default()
                },
            )?);

            let data = CharData {
                mesh,
                advance,
                bearing,
            };

            char_data.insert(c, data);
        }

        Ok(Font {
            texture,
            _margin: margin,
            char_data,
        })
    }

    pub(crate) fn char_mesh(&self, c: char) -> &Ref<Mesh> {
        match self.char_data.get(&c) {
            Some(data) => &data.mesh,
            None => &self.char_data.get(&'?').unwrap().mesh,
        }
    }

    pub(crate) fn char_advance(&self, c: char) -> f32 {
        match self.char_data.get(&c) {
            Some(data) => data.advance,
            None => self.char_data.get(&'?').unwrap().advance,
        }
    }

    pub(crate) fn char_bearing(&self, c: char) -> f32 {
        match self.char_data.get(&c) {
            Some(data) => data.bearing,
            None => self.char_data.get(&'?').unwrap().bearing,
        }
    }

    pub(crate) fn texture(&self) -> &Ref<Texture> {
        &self.texture
    }
}
