use image::GenericImageView;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use tar::Archive;
use tegne_math::Vector2;
use tegne_math::Vector3;

use super::Texture;
use crate::instance::Device;
use crate::mesh::Mesh;
use crate::shaders::ImageUniforms;
use crate::utils::OrError;

pub struct Font {
    texture: Texture,
    char_data: HashMap<char, CharData>,
}

pub struct FontBuilder<'uni> {
    source: Vec<u8>,
    image_uniforms: &'uni ImageUniforms,
    device: Rc<Device>,
}

struct CharData {
    mesh: Mesh,
    advance: f32,
}

#[derive(Deserialize)]
struct JsonAtlasMetrics {
    sdf_size: u32,
    atlas_size: u32,
    char_count: u32,
    char_metrics: HashMap<char, JsonCharMetrics>,
}

#[derive(Deserialize)]
struct JsonCharMetrics {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offset_x: u32,
    offset_y: u32,
}

impl Font {
    pub(crate) fn builder<'uni>(
        device: &Rc<Device>,
        image_uniforms: &'uni ImageUniforms,
    ) -> FontBuilder<'uni> {
        FontBuilder {
            source: vec![],
            image_uniforms,
            device: Rc::clone(device),
        }
    }

    pub fn char_mesh(&self, c: char) -> &Mesh {
        &self
            .char_data
            .get(&c)
            .expect("char data does not exist")
            .mesh
    }

    pub fn char_advance(&self, c: char) -> f32 {
        self.char_data
            .get(&c)
            .expect("char data does not exist")
            .advance
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.texture.image_index()
    }
}

impl FontBuilder<'_> {
    pub fn build(&self) -> Font {
        let mut archive: Archive<&[u8]> = Archive::new(self.source.as_ref());

        let mut atlas_source = vec![];
        let mut image_source = vec![];

        for file in archive.entries().or_error("invalid shader file") {
            let mut file = file.or_error("invalid shader file");

            let path = file
                .header()
                .path()
                .or_error("invalid shader file")
                .to_str()
                .or_error("invalid shader file")
                .to_string();

            if path == "atlas.json" {
                file.read_to_end(&mut atlas_source)
                    .or_error("cannot read atlas");
            }
            if path == "atlas.img" {
                file.read_to_end(&mut image_source)
                    .or_error("cannot read image");
            }
        }

        let atlas: JsonAtlasMetrics =
            serde_json::from_slice(&atlas_source).or_error("invalid font atlas format");

        let texture = Texture::from_raw_rgba(
            &self.device,
            &image_source,
            atlas.atlas_size,
            atlas.atlas_size,
            self.image_uniforms,
        );

        let mut char_data = HashMap::new();
        let atlas_size = atlas.atlas_size as f32;
        for (c, metrics) in atlas.char_metrics {
            let size_norm = atlas.sdf_size as f32 / atlas_size;
            let u_min = metrics.x as f32 / atlas_size;
            let v_min = metrics.y as f32 / atlas_size;
            let u_max = u_min + size_norm;
            let v_max = v_min + size_norm;

            let vertices = vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
            ];

            let uvs = vec![
                Vector2::new(u_min, v_max),
                Vector2::new(u_max, v_max),
                Vector2::new(u_min, v_min),
                Vector2::new(u_max, v_min),
            ];

            let triangles = vec![0, 2, 3, 0, 3, 1];

            let mesh = Mesh::builder(&self.device)
                .with_vertices(&vertices)
                .with_triangles(&triangles)
                .with_uvs(&uvs)
                .with_smooth_normals()
                .build();

            let data = CharData { mesh, advance: 1.0 };

            char_data.insert(c, data);
        }

        Font { texture, char_data }
    }

    pub fn with_source(&mut self, source: &[u8]) -> &mut Self {
        self.source = source.to_vec();
        self
    }
}
