// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// BuiltinMaterials - tegne materials that can be used without extra code

use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Material;
use crate::pipeline::ShaderLayout;

pub(crate) struct BuiltinMaterials {
    pub(crate) white: Id<Material>,
}

impl BuiltinMaterials {
    pub(crate) fn new(
        device: &Arc<Device>,
        layout: &ShaderLayout,
        objects: &Objects,
    ) -> Result<Self> {
        let white = objects.add_material(Material::new(device, layout, Default::default())?);

        Ok(Self { white })
    }
}
