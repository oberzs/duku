// Oliver Berzs
// https://github.com/oberzs/duku

//! Optional feature `otf` module for OTF file support.

#![cfg(feature = "otf")]

use otf_dep::Font as _;
use otf_dep::FontRef;
use otf_dep::PxScale;
use otf_dep::ScaleFont;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::duku::Duku;
use crate::error::Error;
use crate::error::Result;
use crate::font::CharData;
use crate::font::Font;
use crate::font::FontData;
use crate::math::Vec2;
use crate::math::Vec4;
use crate::resources::Handle;

/// Character set to load from a font.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CharSet<'a> {
    /// the basic ASCII character set
    Ascii,
    /// custom character set including utf-8
    Custom(&'a str),
}

impl Duku {
    /// Create font from OTF file
    pub fn create_font_otf(
        &mut self,
        path: impl AsRef<Path>,
        px_size: u32,
        options: Option<CharSet<'_>>,
    ) -> Result<Handle<Font>> {
        let bytes = fs::read(path.as_ref())?;
        self.create_font_otf_bytes(&bytes, px_size, options)
    }

    /// Create font from OTF bytes
    pub fn create_font_otf_bytes(
        &mut self,
        bytes: &[u8],
        px_size: u32,
        options: Option<CharSet<'_>>,
    ) -> Result<Handle<Font>> {
        let charset = options.unwrap_or(CharSet::Ascii);
        let chars = match charset {
            CharSet::Custom(s) => s,
            CharSet::Ascii => "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
        };
        let margin = 1;

        let scale = PxScale::from(px_size as f32);
        let font = FontRef::try_from_slice(bytes)
            .map_err(|_| Error::InvalidOtf)?
            .into_scaled(scale);

        for c in chars.chars() {
            let glyph = font.scaled_glyph(c);
            if font.outline_glyph(glyph).is_none() {
                return Err(Error::UnsupportedChar(c));
            }
        }

        let texture_width = chars.chars().fold(0.0, |acc, c| {
            let glyph = font.scaled_glyph(c);
            let w = font
                .outline_glyph(glyph)
                .expect("bad font")
                .px_bounds()
                .width();
            acc + w
        }) as usize
            + (chars.chars().count() as usize + 1) * margin;
        let texture_height = chars
            .chars()
            .map(|c| {
                let glyph = font.scaled_glyph(c);
                font.outline_glyph(glyph)
                    .expect("bad font")
                    .px_bounds()
                    .height()
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .expect("bad font") as usize
            + 2 * margin;
        let height = font.height();
        let line_gap = font.line_gap();
        let ascender = font.ascent();
        let descender = font.descent();

        let mut font_data = FontData {
            height: height / px_size as f32,
            line_gap: line_gap / px_size as f32,
            ascender: ascender / px_size as f32,
            descender: descender / px_size as f32,
            char_data: HashMap::new(),
            texture_data: &[],
            texture_width: texture_width as u32,
            texture_height: texture_height as u32,
        };

        let mut texture_data = vec![0; texture_width * texture_height];
        let mut x_offset = margin;
        for c in chars.chars() {
            let glyph_id = font.glyph_id(c);
            let glyph = font.scaled_glyph(c);
            let outline = font.outline_glyph(glyph).expect("bad font");
            let bounds = outline.px_bounds();

            let x_advance = font.h_advance(glyph_id).round();
            let x_bearing = font.h_side_bearing(glyph_id).round();
            let y_bearing = bounds.max.y.round();
            let x_bound = bounds.width().round();
            let y_bound = bounds.height().round();

            let v_margin = margin as f32 / texture_height as f32;
            let u_offset = x_offset as f32 / texture_width as f32;
            let u_bound = x_bound / texture_width as f32;
            let v_bound = y_bound / texture_height as f32;

            let u1 = u_offset;
            let u2 = u_offset + u_bound;
            let v1 = v_margin;
            let v2 = v_margin + v_bound;

            font_data.char_data.insert(
                c,
                CharData {
                    uvs: Vec4::new(u1, v1, u2, v2),
                    bounds: Vec2::new(x_bound / px_size as f32, y_bound / px_size as f32),
                    bearing: Vec2::new(x_bearing / px_size as f32, y_bearing / px_size as f32),
                    advance: x_advance / px_size as f32,
                },
            );

            outline.draw(|x, y, v| {
                let xx = x as usize + x_offset;
                let yy = y as usize + margin;
                let vv = (v * 255.0).round() as u8;
                texture_data[xx + texture_width * yy] = vv;
            });

            x_offset += x_bound as usize + margin;
        }

        font_data.texture_data = &texture_data;

        self.create_font(font_data)
    }
}
