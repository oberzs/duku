use std::collections::HashMap;
use std::rc::Rc;

use crate::instance::Device;
use crate::shaders::Material;
use crate::shaders::ShaderLayout;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinMaterial {
    White,
}

pub(crate) fn builtin_materials(
    device: &Rc<Device>,
    layout: &ShaderLayout,
) -> HashMap<BuiltinMaterial, Material> {
    let mut map = HashMap::new();

    map.insert(
        BuiltinMaterial::White,
        Material::new(device, layout, Default::default()),
    );

    map
}
