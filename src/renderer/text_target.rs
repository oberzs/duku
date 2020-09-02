// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects text draw calls in a batch

use crate::color::Color;
use crate::mesh::Vertex;

pub struct TextTarget {
    pub font_size: u32,
    pub color: Color,

    batch: Vec<Vertex>,
}

impl TextTarget {
    pub(crate) fn new() -> Self {
        Self {
            font_size: 24,
            color: Color::BLACK,
            batch: vec![],
        }
    }
}
