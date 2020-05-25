use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::device::Device;
use crate::error::Result;
use crate::font::Font;
use crate::pipeline::ImageUniform;

pub(crate) struct BuiltinFonts {
    pub(crate) roboto_mono: Id<Font>,
}

impl BuiltinFonts {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        objects: &Objects,
    ) -> Result<Self> {
        let roboto_mono_src = include_bytes!("../../assets/fonts/RobotoMono-Regular.font");

        let roboto_mono = objects.add_font(Font::new(device, uniform, objects, roboto_mono_src)?);

        Ok(Self { roboto_mono })
    }
}
