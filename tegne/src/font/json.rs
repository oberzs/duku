// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// JSON structs for font format reading

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct JsonAtlasMetrics {
    pub(crate) sdf_size: u32,
    pub(crate) atlas_size: u32,
    pub(crate) margin: u32,
    pub(crate) char_metrics: HashMap<char, JsonCharMetrics>,
}

#[derive(Deserialize)]
pub(crate) struct JsonCharMetrics {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) advance: u32,
    pub(crate) bearing: u32,
}
