use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::error::Result;
use crate::instance::Device;
use crate::shaders::Material;
use crate::shaders::ShaderLayout;

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
