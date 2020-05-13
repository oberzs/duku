use std::collections::HashMap;
use std::rc::Rc;

use crate::instance::Device;
use crate::instance::RenderPassType;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

macro_rules! include_shader {
    ($path:expr) => {
        include_bytes!(concat!("../../assets/shaders/", $path))
    };
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinShader {
    Phong,
    Unshaded,
    Passthru,
    Wireframe,
    Shadow,
    Font,
}

pub(crate) fn builtin_shaders(
    device: &Rc<Device>,
    passes: &HashMap<RenderPassType, RenderPass>,
    layout: &ShaderLayout,
) -> HashMap<BuiltinShader, Shader> {
    let mut map = HashMap::new();

    let color_pass = passes
        .get(&RenderPassType::Color)
        .or_error("render passes not setup");
    let depth_pass = passes
        .get(&RenderPassType::Depth)
        .or_error("render passes not setup");

    let phong = include_shader!("phong.shader");
    let unshaded = include_shader!("unshaded.shader");
    let passthru = include_shader!("passthru.shader");
    let shadow = include_shader!("shadow.shader");
    let wireframe = include_shader!("wireframe.shader");
    let font = include_shader!("font.shader");

    map.insert(
        BuiltinShader::Phong,
        Shader::builder(&device, &color_pass, &layout)
            .with_source(phong)
            .build(),
    );
    map.insert(
        BuiltinShader::Unshaded,
        Shader::builder(&device, &color_pass, &layout)
            .with_source(unshaded)
            .build(),
    );
    map.insert(
        BuiltinShader::Passthru,
        Shader::builder(&device, &color_pass, &layout)
            .with_source(passthru)
            .with_no_depth()
            .build(),
    );
    map.insert(
        BuiltinShader::Shadow,
        Shader::builder(&device, &depth_pass, &layout)
            .with_source(shadow)
            .build(),
    );
    map.insert(
        BuiltinShader::Wireframe,
        Shader::builder(&device, &color_pass, &layout)
            .with_source(wireframe)
            .with_lines()
            .with_no_depth()
            .build(),
    );
    map.insert(
        BuiltinShader::Font,
        Shader::builder(&device, &color_pass, &layout)
            .with_source(font)
            .build(),
    );

    map
}
