// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects text draw calls in a batch

use crate::color::Color;
use crate::font::Font;
use crate::math::Transform;
use crate::storage::Builtins;
use crate::storage::Handle;

pub struct TextTarget {
    pub font_size: u32,
    pub color: Color,

    font: Handle<Font>,
    orders: Vec<TextOrder>,
}

pub(crate) struct TextOrder {
    pub(crate) font_size: u32,
    pub(crate) _color: Color,
    pub(crate) font: Handle<Font>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
}

impl TextTarget {
    pub(crate) fn new(builtins: &Builtins) -> Self {
        Self {
            font_size: 24,
            color: Color::BLACK,
            orders: vec![],
            font: builtins.fira_font.clone(),
        }
    }

    pub fn draw<S, T>(&mut self, text: S, transform: T)
    where
        S: AsRef<str>,
        T: Into<Transform>,
    {
        self.orders.push(TextOrder {
            font_size: self.font_size,
            _color: self.color,
            font: self.font.clone(),
            text: text.as_ref().to_string(),
            transform: transform.into(),
        });
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = &TextOrder> {
        self.orders.iter()
    }
}
