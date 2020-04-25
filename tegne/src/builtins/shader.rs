use std::collections::HashMap;
use std::rc::Rc;

use crate::instance::Device;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;

macro_rules! include_shader {
    ($path:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/shaders/", $path, ".spv")).as_ref()
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
    pass: &RenderPass,
    layout: &ShaderLayout,
) -> HashMap<BuiltinShader, Shader> {
    let mut map = HashMap::new();

    let world_vert = include_shader!("world.vert");
    let passthru_vert = include_shader!("passthru.vert");
    let shadow_vert = include_shader!("shadow.vert");
    let phong_frag = include_shader!("phong.frag");
    let wireframe_frag = include_shader!("wireframe.frag");
    let passthru_frag = include_shader!("passthru.frag");
    let shadow_frag = include_shader!("shadow.frag");

    map.insert(
        BuiltinShader::Phong,
        Shader::builder(&device, &pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(phong_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Unshaded,
        Shader::builder(&device, &pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(passthru_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Passthru,
        Shader::builder(&device, &pass, &layout)
            .with_vert_source(passthru_vert)
            .with_frag_source(passthru_frag)
            .with_no_depth()
            .build(),
    );
    map.insert(
        BuiltinShader::Shadow,
        Shader::builder(&device, &pass, &layout)
            .with_vert_source(shadow_vert)
            .with_frag_source(shadow_frag)
            .build(),
    );
    map.insert(
        BuiltinShader::Wireframe,
        Shader::builder(&device, &pass, &layout)
            .with_vert_source(world_vert)
            .with_frag_source(wireframe_frag)
            .with_lines()
            .with_no_depth()
            .build(),
    );

    map
}
