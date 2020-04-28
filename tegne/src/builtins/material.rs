use std::collections::HashMap;
use std::rc::Rc;

use super::BuiltinShader;
use super::BuiltinTexture;
use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::Material;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinMaterial {
    Wireframe,
    Shadow,
    White,
}

pub(crate) fn builtin_materials(
    device: &Rc<Device>,
    layout: &ShaderLayout,
    shaders: &HashMap<BuiltinShader, Shader>,
    textures: &HashMap<BuiltinTexture, Texture>,
) -> HashMap<BuiltinMaterial, Material> {
    let mut map = HashMap::new();

    let phong_s = shaders
        .get(&BuiltinShader::Phong)
        .or_error("shaders not setup");
    let wireframe_s = shaders
        .get(&BuiltinShader::Wireframe)
        .or_error("shaders not setup");
    let shadow_s = shaders
        .get(&BuiltinShader::Shadow)
        .or_error("shaders not setup");
    let white_t = textures
        .get(&BuiltinTexture::White)
        .or_error("textures not setup");

    map.insert(
        BuiltinMaterial::White,
        Material::builder(&device, &phong_s, &white_t, layout).build(),
    );
    map.insert(
        BuiltinMaterial::Wireframe,
        Material::builder(&device, &wireframe_s, &white_t, layout).build(),
    );
    map.insert(
        BuiltinMaterial::Shadow,
        Material::builder(&device, &shadow_s, &white_t, layout).build(),
    );

    map
}
