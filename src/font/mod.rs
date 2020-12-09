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

/// Font data and metrics.
///
/// Used to construct a new font
#[derive(Debug)]
pub struct FontData<'a> {
    /// the height of the font
    pub height: f32,
    /// the gap between 2 different lines
    pub line_gap: f32,
    /// the highest point above the baseline
    pub ascender: f32,
    /// the lowest point below the baseline (negative)
    pub descender: f32,
    /// data for all loaded characters
    pub char_data: HashMap<char, CharData>,
    /// grayscale bytes for creating the font atlas texture
    pub texture_data: &'a [u8],
    /// atlas texture's width
    pub texture_width: u32,
    /// atlas texture's height
    pub texture_height: u32,
}

/// Character data and metrics.
///
/// Used to construct a character
/// in a font
#[derive(Debug, Copy, Clone)]
pub struct CharData {
    /// top-left and bottom-right corner for character's texture
    pub uvs: Vec4,
    /// width and height of the character's texture
    pub bounds: Vec2,
    /// offset of the character
    pub bearing: Vec2,
    /// advance between this and the next character in a string
    pub advance: f32,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FontMetrics {
    pub(crate) height: f32,
    pub(crate) line_gap: f32,
    pub(crate) ascender: f32,
    pub(crate) descender: f32,
    pub(crate) space_width: f32,
}

impl Font {
    pub(crate) fn fira_mono(device: &Device, uniforms: &mut Uniforms) -> Result<Self> {
        Self::new(device, uniforms, fira_mono())
    }

    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        data: FontData<'_>,
    ) -> Result<Self> {
        let texture = Texture::new(
            device,
            uniforms,
            data.texture_data.to_vec(),
            data.texture_width,
            data.texture_height,
            Format::Gray,
            Mips::Zero,
        )?;

        Ok(Self {
            metrics: FontMetrics {
                height: data.height,
                line_gap: data.line_gap,
                ascender: data.ascender,
                descender: data.descender,
                space_width: 1.0 / 3.0,
            },
            char_data: data.char_data,
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
            None => *self
                .char_data
                .values()
                .next()
                .expect("font has no characters"),
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
