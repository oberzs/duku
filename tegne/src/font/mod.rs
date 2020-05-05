mod json;

use image::GenericImageView;
use std::rc::Rc;

use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::ImageUniforms;
use crate::utils::OrError;
use json::Atlas;
use json::Props;

struct Font {
    texture: Texture,
}

struct FontBuilder<'a> {
    atlas: Vec<u8>,
    props: Vec<u8>,
    image: Vec<u8>,
    image_uniforms: &'a ImageUniforms,
    device: &'a Rc<Device>,
}

impl FontBuilder<'_> {
    pub fn build(&self) -> Font {
        let props: Props =
            serde_json::from_slice(&self.props).or_error("invalid font props format");
        let atlas: Atlas =
            serde_json::from_slice(&self.atlas).or_error("invalid font atlas format");

        let img = image::load_from_memory(&self.image).or_error("invalid atlas image");
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();
        let texture =
            Texture::from_raw_rgba(self.device, &data, width, height, self.image_uniforms);

        Font { texture }
    }
}
