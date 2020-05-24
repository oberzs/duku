use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::error::Result;
use crate::font::Font;
use crate::instance::Device;
use crate::shaders::ImageUniforms;

pub(crate) struct BuiltinFonts {
    pub(crate) roboto_mono: Id<Font>,
}

impl BuiltinFonts {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniforms: &ImageUniforms,
        objects: &Objects,
    ) -> Result<Self> {
        let roboto_mono_src = include_bytes!("../../assets/fonts/RobotoMono-Regular.font");

        let roboto_mono = objects.add_font(Font::new(device, uniforms, objects, roboto_mono_src)?);

        Ok(Self { roboto_mono })
    }
}
