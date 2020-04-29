use std::collections::HashMap;
use std::rc::Rc;
use tegne_macro::include_shader;

use crate::instance::Device;
use crate::instance::RenderPassType;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

macro_rules! include_builtin_shader {
    ($path:expr) => {
        include_shader!("src/shaders/glsl/", $path)
    };
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinShader {
    Phong,
    Unshaded,
    Passthru,
    Wireframe,
    Shadow,
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

    let world_vert = include_builtin_shader!("world.vert");
    let passthru_vert = include_builtin_shader!("passthru.vert");
    let shadow_vert = include_builtin_shader!("shadow.vert");
    let phong_frag = include_builtin_shader!("phong.frag");
    let wireframe_frag = include_builtin_shader!("wireframe.frag");
    let passthru_frag = include_builtin_shader!("passthru.frag");
    let shadow_frag = include_builtin_shader!("shadow.frag");

    map.insert(
        BuiltinShader::Phong,
        Shader::builder(&device, &color_pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(phong_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Unshaded,
        Shader::builder(&device, &color_pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(passthru_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Passthru,
        Shader::builder(&device, &color_pass, &layout)
            .with_vert_source(passthru_vert)
            .with_frag_source(passthru_frag)
            .with_no_depth()
            .build(),
    );
    map.insert(
        BuiltinShader::Shadow,
        Shader::builder(&device, &depth_pass, &layout)
            .with_vert_source(shadow_vert)
            .with_frag_source(shadow_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Wireframe,
        Shader::builder(&device, &color_pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(wireframe_frag)
            .with_lines()
            .with_no_depth()
            .build(),
    );

    map
}
