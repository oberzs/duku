use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::error::Result;
use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::ImageUniforms;

pub(crate) struct BuiltinTextures {
    pub(crate) white: Id<Texture>,
}

impl BuiltinTextures {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniforms: &ImageUniforms,
        objects: &Objects,
    ) -> Result<Self> {
        let white = objects.add_texture(Texture::from_raw_rgba(
            device,
            &[255, 255, 255, 255],
            1,
            1,
            uniforms,
        )?);

        Ok(Self { white })
    }
}
