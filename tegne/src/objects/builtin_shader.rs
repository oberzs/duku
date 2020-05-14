use std::collections::HashMap;
use std::sync::Arc;

use crate::instance::Device;
use crate::instance::RenderPassType;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::shaders::ShaderOptions;
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
    device: &Arc<Device>,
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
        Shader::new(device, color_pass, layout, phong, Default::default()),
    );
    map.insert(
        BuiltinShader::Unshaded,
        Shader::new(device, color_pass, layout, unshaded, Default::default()),
    );
    map.insert(
        BuiltinShader::Shadow,
        Shader::new(device, depth_pass, layout, shadow, Default::default()),
    );
    map.insert(
        BuiltinShader::Font,
        Shader::new(device, color_pass, layout, font, Default::default()),
    );
    map.insert(
        BuiltinShader::Passthru,
        Shader::new(
            device,
            color_pass,
            layout,
            passthru,
            ShaderOptions {
                has_depth_test: false,
                ..Default::default()
            },
        ),
    );
    map.insert(
        BuiltinShader::Wireframe,
        Shader::new(
            device,
            color_pass,
            layout,
            wireframe,
            ShaderOptions {
                has_lines: true,
                has_depth_test: false,
                ..Default::default()
            },
        ),
    );

    map
}
