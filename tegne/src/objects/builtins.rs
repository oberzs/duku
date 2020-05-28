// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Builtins - builtin resource manager

use log::debug;
use std::sync::Arc;

use super::BuiltinFonts;
use super::BuiltinMaterials;
use super::BuiltinMeshes;
use super::BuiltinShaders;
use super::BuiltinTextures;
use super::Id;
use super::Objects;
use crate::device::Device;
use crate::error::Result;
use crate::image::Texture;
use crate::pipeline::ImageUniform;
use crate::pipeline::RenderPasses;
use crate::pipeline::ShaderLayout;

pub(crate) struct Builtins {
    pub(crate) textures: BuiltinTextures,
    pub(crate) shaders: BuiltinShaders,
    pub(crate) materials: BuiltinMaterials,
    pub(crate) meshes: BuiltinMeshes,
    pub(crate) fonts: BuiltinFonts,
}

impl Builtins {
    pub(crate) fn new(
        device: &Arc<Device>,
        passes: &RenderPasses,
        layout: &ShaderLayout,
        uniform: &ImageUniform,
        objects: &Objects,
        #[cfg(feature = "ui")] ui_texture: Id<Texture>,
    ) -> Result<Self> {
        debug!("creating builtin meshes");
        let meshes = BuiltinMeshes::new(device, objects)?;

        debug!("creating builtin shaders");
        let shaders = BuiltinShaders::new(device, passes, layout, objects)?;

        debug!("creating builtin textures");
        let textures = BuiltinTextures::new(device, uniform, objects, ui_texture)?;

        debug!("creating builtin materials");
        let materials = BuiltinMaterials::new(device, layout, objects)?;

        debug!("creating builtin fonts");
        let fonts = BuiltinFonts::new(device, uniform, objects)?;

        Ok(Self {
            meshes,
            shaders,
            textures,
            materials,
            fonts,
        })
    }
}
