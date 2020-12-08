// Oliver Berzs
// https://github.com/oberzs/duku

mod fira_mono;

use std::collections::HashMap;

use crate::device::Device;
use crate::error::Result;
use crate::image::Format;
use crate::image::Mips;
use crate::image::Texture;
use crate::math::Vec2;
use crate::math::Vec4;
use crate::pipeline::Uniforms;
use fira_mono::fira_mono;

/// Font for text drawing.
pub struct Font {
    metrics: FontMetrics,
    char_data: HashMap<char, CharData>,
    texture: Texture,
}

#[derive(Debug)]
pub struct FontData<'a> {
    pub height: f32,
    pub line_gap: f32,
    pub ascender: f32,
    pub descender: f32,
    pub char_data: HashMap<char, CharData>,
    pub texture_data: &'a [u8],
    pub texture_width: u32,
    pub texture_height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct FontMetrics {
    pub(crate) height: f32,
    pub(crate) line_gap: f32,
    pub(crate) ascender: f32,
    pub(crate) descender: f32,
    pub(crate) space_width: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct CharData {
    pub uvs: Vec4,
    pub bounds: Vec2,
    pub bearing: Vec2,
    pub advance: f32,
}

impl Font {
    pub(crate) fn fira_mono(device: &Device, uniforms: &mut Uniforms) -> Result<Self> {
        let font_data = fira_mono();

        let texture = Texture::new(
            device,
            uniforms,
            font_data.texture_data.to_vec(),
            font_data.texture_width,
            font_data.texture_height,
            Format::Gray,
            Mips::Zero,
        )?;

        Ok(Self {
            metrics: FontMetrics {
                height: font_data.height,
                line_gap: font_data.line_gap,
                ascender: font_data.ascender,
                descender: font_data.descender,
                space_width: 1.0 / 3.0,
            },
            char_data: font_data.char_data,
            texture,
        })
    }

    pub(crate) const fn texture(&self) -> &Texture {
        &self.texture
    }

    pub(crate) const fn metrics(&self) -> FontMetrics {
        self.metrics
    }

    pub(crate) fn char_data(&self, c: char) -> CharData {
        match self.char_data.get(&c) {
            Some(data) => *data,
            None => *self.char_data.get(&'?').expect("bad default"),
        }
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        self.texture.destroy(device, uniforms);
    }
}

impl FontMetrics {
    pub(crate) fn scaled(self, px: u32) -> Self {
        let scale = px as f32;
        Self {
            height: self.height * scale,
            line_gap: self.line_gap * scale,
            ascender: self.ascender * scale,
            descender: self.descender * scale,
            space_width: self.space_width * scale,
        }
    }
}

impl CharData {
    pub(crate) fn scaled(self, px: u32) -> Self {
        let scale = px as f32;
        Self {
            uvs: self.uvs,
            bounds: self.bounds * scale,
            bearing: self.bearing * scale,
            advance: self.advance * scale,
        }
    }
}
