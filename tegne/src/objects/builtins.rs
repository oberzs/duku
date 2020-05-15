use log::debug;
use std::sync::Arc;

use super::BuiltinFonts;
use super::BuiltinMaterials;
use super::BuiltinMeshes;
use super::BuiltinShaders;
use super::BuiltinTextures;
use super::Objects;
use crate::error::Result;
use crate::instance::Device;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPasses;
use crate::shaders::ShaderLayout;

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
        uniforms: &ImageUniforms,
        objects: &Objects,
    ) -> Result<Self> {
        debug!("creating builtin meshes");
        let meshes = BuiltinMeshes::new(device, objects)?;

        debug!("creating builtin shaders");
        let shaders = BuiltinShaders::new(device, passes, layout, objects)?;

        debug!("creating builtin textures");
        let textures = BuiltinTextures::new(device, uniforms, objects)?;

        debug!("creating builtin materials");
        let materials = BuiltinMaterials::new(device, layout, objects)?;

        debug!("creating builtin fonts");
        let fonts = BuiltinFonts::new(device, uniforms, objects)?;

        Ok(Self {
            meshes,
            shaders,
            textures,
            materials,
            fonts,
        })
    }
}
