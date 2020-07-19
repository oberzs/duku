// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// structs for font format deserialization

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct FontFile {
    pub(crate) sdf_size: u32,
    pub(crate) atlas_size: u32,
    pub(crate) margin: u32,
    pub(crate) char_metrics: HashMap<char, FontCharMetrics>,
    pub(crate) atlas: Vec<u8>,
}

#[derive(Deserialize)]
pub(crate) struct FontCharMetrics {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) advance: u32,
    pub(crate) bearing: u32,
}
