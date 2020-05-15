use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::error::Result;
use crate::instance::Device;
use crate::shaders::RenderPasses;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::shaders::ShaderOptions;

macro_rules! include_shader {
    ($path:expr) => {
        include_bytes!(concat!("../../assets/shaders/", $path))
    };
}

pub(crate) struct BuiltinShaders {
    pub(crate) phong: Id<Shader>,
    // pub(crate) unshaded: Id<Shader>,
    // pub(crate) passthru: Id<Shader>,
    pub(crate) wireframe: Id<Shader>,
    pub(crate) shadow: Id<Shader>,
    pub(crate) font: Id<Shader>,
}

impl BuiltinShaders {
    pub(crate) fn new(
        device: &Arc<Device>,
        passes: &RenderPasses,
        layout: &ShaderLayout,
        objects: &Objects,
    ) -> Result<Self> {
        let color_pass = passes.color();
        let depth_pass = passes.depth();

        let phong_spv = include_shader!("phong.shader");
        let unshaded_spv = include_shader!("unshaded.shader");
        let passthru_spv = include_shader!("passthru.shader");
        let shadow_spv = include_shader!("shadow.shader");
        let wireframe_spv = include_shader!("wireframe.shader");
        let font_spv = include_shader!("font.shader");

        let phong = objects.add_shader(Shader::new(
            device,
            color_pass,
            layout,
            phong_spv,
            Default::default(),
        )?);

        let _unshaded = objects.add_shader(Shader::new(
            device,
            color_pass,
            layout,
            unshaded_spv,
            Default::default(),
        )?);

        let shadow = objects.add_shader(Shader::new(
            device,
            depth_pass,
            layout,
            shadow_spv,
            Default::default(),
        )?);

        let font = objects.add_shader(Shader::new(
            device,
            color_pass,
            layout,
            font_spv,
            Default::default(),
        )?);

        let _passthru = objects.add_shader(Shader::new(
            device,
            color_pass,
            layout,
            passthru_spv,
            ShaderOptions {
                has_depth_test: false,
                ..Default::default()
            },
        )?);

        let wireframe = objects.add_shader(Shader::new(
            device,
            color_pass,
            layout,
            wireframe_spv,
            ShaderOptions {
                has_lines: true,
                has_depth_test: false,
                ..Default::default()
            },
        )?);

        Ok(Self {
            phong,
            // unshaded,
            shadow,
            font,
            // passthru,
            wireframe,
        })
    }
}
