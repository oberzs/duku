// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// BuiltinTextures - tegne textures that can be used without extra files

use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::device::Device;
use crate::error::Result;
use crate::image::Texture;
use crate::pipeline::ImageUniform;

pub(crate) struct BuiltinTextures {
    pub(crate) white: Id<Texture>,
}

impl BuiltinTextures {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        objects: &Objects,
    ) -> Result<Self> {
        let white = objects.add_texture(Texture::from_raw_rgba(
            device,
            uniform,
            &[255, 255, 255, 255],
            1,
            1,
        )?);

        Ok(Self { white })
    }
}
