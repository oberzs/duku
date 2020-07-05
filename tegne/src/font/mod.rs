// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Font - struct for a renderable SDF font

mod json;

use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use tar::Archive;

use crate::device::Device;
use crate::error::Result;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::ImageUniform;
use crate::resource::Id;
use crate::resource::IdRef;
use crate::resource::ResourceManager;
use json::JsonAtlasMetrics;

pub struct Font {
    texture: Id<Texture>,
    _margin: f32,
    char_data: HashMap<char, CharData>,
}

struct CharData {
    mesh: Id<Mesh>,
    advance: f32,
    bearing: f32,
}

impl Font {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        resources: &ResourceManager,
        source: &[u8],
    ) -> Result<Self> {
        // read font data from archive
        let mut archive: Archive<&[u8]> = Archive::new(source);

        let mut atlas_source = vec![];
        let mut image_source = vec![];

        for file in archive.entries()? {
            let mut file = file?;

            let path = file.header().path()?.into_owned();

            if path.ends_with("atlas.json") {
                file.read_to_end(&mut atlas_source)?;
            }
            if path.ends_with("atlas.img") {
                file.read_to_end(&mut image_source)?;
            }
        }

        // decode font atlas metrics
        let atlas: JsonAtlasMetrics = serde_json::from_slice(&atlas_source)?;

        // create font atlas texture
        let texture = resources.add_texture(Texture::new(
            device,
            uniform,
            TextureOptions {
                data: &image_source,
                width: atlas.atlas_size,
                height: atlas.atlas_size,
                ..Default::default()
            },
        )?);

        let margin = atlas.margin as f32 / atlas.sdf_size as f32;

        // generate mesh for each character in atlas
        let mut char_data = HashMap::new();
        for (c, metrics) in atlas.char_metrics {
            // UV relative metrics
            let size_norm = atlas.sdf_size as f32 / atlas.atlas_size as f32;
            let u_min = metrics.x as f32 / atlas.atlas_size as f32;
            let v_min = metrics.y as f32 / atlas.atlas_size as f32;
            let u_max = u_min + size_norm;
            let v_max = v_min + size_norm;

            // vertex relative metrics
            let advance = metrics.advance as f32 / atlas.sdf_size as f32;
            let bearing = metrics.bearing as f32 / atlas.sdf_size as f32;

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

    pub(crate) fn char_mesh(&self, c: char) -> IdRef {
        match self.char_data.get(&c) {
            Some(data) => data.mesh.id_ref(),
            None => self.char_data.get(&'?').unwrap().mesh.id_ref(),
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

    pub(crate) fn texture(&self) -> IdRef {
        self.texture.id_ref()
    }
}
