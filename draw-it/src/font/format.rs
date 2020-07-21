// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// structs for font format deserialization

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct FontFile {
    pub(crate) bitmap_fonts: Vec<BitmapFont>,
    pub(crate) sdf_font: SdfFont,
}

#[derive(Deserialize)]
pub(crate) struct BitmapFont {
    pub(crate) bitmap_size: u32,
    pub(crate) font_size: u32,
    pub(crate) char_metrics: HashMap<char, CharMetrics>,
    pub(crate) bitmap: Vec<u8>,
}

#[derive(Deserialize)]
pub(crate) struct SdfFont {
    pub(crate) bitmap_size: u32,
    pub(crate) font_size: u32,
    pub(crate) margin: u32,
    pub(crate) char_metrics: HashMap<char, CharMetrics>,
    pub(crate) bitmap: Vec<u8>,
}

#[derive(Deserialize)]
pub(crate) struct CharMetrics {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) advance: u32,
}
