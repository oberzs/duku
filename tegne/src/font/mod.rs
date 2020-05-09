use image::GenericImageView;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use tar::Archive;

use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::ImageUniforms;
use crate::utils::OrError;

struct Font {
    texture: Texture,
}

struct FontBuilder<'uni> {
    source: Vec<u8>,
    image_uniforms: &'uni ImageUniforms,
    device: Rc<Device>,
}

#[derive(Deserialize)]
struct AtlasMetrics {
    sdf_size: u32,
    atlas_size: u32,
    char_count: u32,
    char_metrics: HashMap<char, CharMetrics>,
}

#[derive(Deserialize)]
struct CharMetrics {
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

        let atlas: AtlasMetrics =
            serde_json::from_slice(&atlas_source).or_error("invalid font atlas format");
        let img = image::load_from_memory(&image_source).or_error("invalid atlas image");

        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();
        let texture =
            Texture::from_raw_rgba(&self.device, &data, width, height, self.image_uniforms);

        Font { texture }
    }

    pub fn with_source(&mut self, source: &[u8]) -> &mut Self {
        self.source = source.to_vec();
        self
    }
}
