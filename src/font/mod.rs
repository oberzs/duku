// Oliver Berzs
// https://github.com/oberzs/draw-it

// Font - struct for a renderable font

mod fira_mono;

use std::collections::HashMap;

use crate::device::Device;
use crate::image::ImageFormat;
use crate::image::Size;
use crate::image::Texture;
use crate::math::Vector4;
use crate::pipeline::ShaderImages;

pub struct Font {
    char_data: HashMap<char, CharData>,
    texture: Texture,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct CharData {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) uvs: Vector4,
    pub(crate) x_offset: f32,
    pub(crate) y_offset: f32,
    pub(crate) advance: f32,
}

impl Font {
    pub(crate) fn fira_mono(device: &Device, shader_images: &mut ShaderImages) -> Self {
        let atlas_width = fira_mono::ATLAS_WIDTH;
        let atlas_height = fira_mono::ATLAS_HEIGHT;
        let line_height = fira_mono::LINE_HEIGHT;

        let texture = Texture::new(
            device,
            shader_images,
            fira_mono::DATA.to_vec(),
            Size::new(atlas_width, atlas_height),
            ImageFormat::Gray,
        );

        let mut char_data = HashMap::new();
        for (c, metrics) in fira_mono::metrics() {
            let u_min = metrics.x as f32 / atlas_width as f32;
            let v_min = metrics.y as f32 / atlas_height as f32;
            let u_max = u_min + (metrics.width as f32 / atlas_width as f32);
            let v_max = v_min + (metrics.height as f32 / atlas_height as f32);
            let uvs = Vector4::new(u_min, v_min, u_max, v_max);

            let width = metrics.width as f32 / line_height as f32;
            let height = metrics.height as f32 / line_height as f32;

            let x_offset = metrics.xo as f32 / line_height as f32;
            let y_offset = metrics.yo as f32 / line_height as f32;
            let advance = metrics.advance as f32 / line_height as f32;

            char_data.insert(
                c,
                CharData {
                    width,
                    height,
                    uvs,
                    x_offset,
                    y_offset,
                    advance,
                },
            );
        }

        Self { char_data, texture }
    }

    pub(crate) const fn texture(&self) -> &Texture {
        &self.texture
    }

    pub(crate) fn char_data(&self, c: char) -> CharData {
        match self.char_data.get(&c) {
            Some(data) => *data,
            None => *self.char_data.get(&'?').expect("bad default"),
        }
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.texture.destroy(device);
    }
}
