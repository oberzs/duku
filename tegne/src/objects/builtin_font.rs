use std::collections::HashMap;
use std::rc::Rc;

use crate::images::Font;
use crate::instance::Device;
use crate::shaders::ImageUniforms;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinFont {
    NotoSans,
}

pub(crate) fn builtin_fonts(
    device: &Rc<Device>,
    uniforms: &ImageUniforms,
) -> HashMap<BuiltinFont, Font> {
    let mut map = HashMap::new();

    let noto_sans = include_bytes!("../../assets/fonts/RobotoMono-Regular.font");

    map.insert(
        BuiltinFont::NotoSans,
        Font::new(device, uniforms, noto_sans),
    );

    map
}
