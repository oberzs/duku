use std::collections::HashMap;
use std::rc::Rc;

use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::ImageUniforms;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinTexture {
    White,
}

pub(crate) fn builtin_textures(
    device: &Rc<Device>,
    uniforms: &ImageUniforms,
) -> HashMap<BuiltinTexture, Texture> {
    let mut map = HashMap::new();
    map.insert(
        BuiltinTexture::White,
        Texture::from_raw_rgba(device, &[255, 255, 255, 255], 1, 1, uniforms),
    );
    map
}
