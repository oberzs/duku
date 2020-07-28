// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Font - struct for a renderable SDF font

mod format;

use std::collections::HashMap;
use std::sync::Arc;

use crate::device::Device;
use crate::error::Result;
use crate::image::ImageFormat;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::ImageUniform;
use format::CharMetrics;
use format::FontFile;

pub struct Font {
    bitmap_data: HashMap<u32, FontData>,
    sdf_data: FontData,
}

#[derive(Copy, Clone)]
pub(crate) struct CharData {
    pub(crate) advance: f32,
    pub(crate) offset: u32,
}

struct FontData {
    texture: Texture,
    mesh: Mesh,
    margin: f32,
    char_data: HashMap<char, CharData>,
}

impl Font {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &mut ImageUniform,
        source: &[u8],
    ) -> Result<Self> {
        let FontFile {
            sdf_font,
            bitmap_fonts,
        } = bincode::deserialize(source)?;

        let mut bitmap_data = HashMap::new();
        for font in &bitmap_fonts {
            bitmap_data.insert(
                font.font_size,
                create_font(
                    device,
                    uniform,
                    &font.bitmap,
                    font.bitmap_size,
                    font.font_size,
                    0,
                    &font.char_metrics,
                )?,
            );
        }

        let sdf_data = create_font(
            device,
            uniform,
            &sdf_font.bitmap,
            sdf_font.bitmap_size,
            sdf_font.font_size,
            sdf_font.margin,
            &sdf_font.char_metrics,
        )?;

        Ok(Font {
            bitmap_data,
            sdf_data,
        })
    }

    pub(crate) fn is_bitmap(&self, font_size: u32) -> bool {
        self.bitmap_data.contains_key(&font_size)
    }

    pub(crate) fn char_data(&self, font_size: u32, c: char) -> CharData {
        // if has bitmap font of that size, choose bitmap
        // otherwise choose sdf font
        let char_data = match self.bitmap_data.get(&font_size) {
            Some(data) => &data.char_data,
            None => &self.sdf_data.char_data,
        };
        match char_data.get(&c) {
            Some(data) => *data,
            None => *char_data.get(&'?').expect("bad default"),
        }
    }

    pub(crate) fn texture(&self, font_size: u32) -> &Texture {
        // if has bitmap font of that size, choose bitmap
        // otherwise choose sdf font
        match self.bitmap_data.get(&font_size) {
            Some(data) => &data.texture,
            None => &self.sdf_data.texture,
        }
    }

    pub(crate) fn mesh(&self, font_size: u32) -> &Mesh {
        // if has bitmap font of that size, choose bitmap
        // otherwise choose sdf font
        match self.bitmap_data.get(&font_size) {
            Some(data) => &data.mesh,
            None => &self.sdf_data.mesh,
        }
    }

    pub(crate) fn margin(&self, font_size: u32) -> f32 {
        // if has bitmap font of that size, choose bitmap
        // otherwise choose sdf font
        match self.bitmap_data.get(&font_size) {
            Some(data) => data.margin,
            None => self.sdf_data.margin,
        }
    }
}

fn create_font(
    device: &Arc<Device>,
    uniform: &mut ImageUniform,
    bitmap: &[u8],
    bitmap_size: u32,
    font_size: u32,
    margin: u32,
    char_metrics: &HashMap<char, CharMetrics>,
) -> Result<FontData> {
    let texture = Texture::new(
        device,
        uniform,
        TextureOptions {
            data: bitmap,
            width: bitmap_size,
            height: bitmap_size,
            format: ImageFormat::Gray,
        },
    )?;

    let mut vertices = vec![];
    let mut uvs = vec![];
    let mut indices = vec![];
    let mut offset = 0;

    let norm_margin = margin as f32 / font_size as f32;

    let mut char_data = HashMap::new();
    for (c, metrics) in char_metrics {
        // UV relative metrics
        let size_norm = font_size as f32 / bitmap_size as f32;
        let u_min = metrics.x as f32 / bitmap_size as f32;
        let v_min = (metrics.y as f32 + margin as f32) / bitmap_size as f32;
        let u_max = u_min + size_norm;
        let v_max = v_min + size_norm;

        // vertex relative metrics
        let advance = metrics.advance as f32 / font_size as f32;

        let o = vertices.len() as u32;

        vertices.extend(&[
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, -1.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        ]);
        uvs.extend(&[
            Vector2::new(u_min, v_min),
            Vector2::new(u_max, v_min),
            Vector2::new(u_max, v_max),
            Vector2::new(u_min, v_max),
        ]);
        indices.extend(&[o, o + 1, o + 2, o, o + 2, o + 3]);

        char_data.insert(*c, CharData { advance, offset });
        offset += 6;
    }

    let mut mesh = Mesh::new(
        device,
        MeshOptions {
            vertices: &vertices,
            indices: &indices,
            uvs: &uvs,
            ..Default::default()
        },
    )?;
    mesh.update_if_needed()?;

    Ok(FontData {
        margin: norm_margin,
        char_data,
        texture,
        mesh,
    })
}
