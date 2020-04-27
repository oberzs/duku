use ash::vk::Pipeline;
use std::rc::Rc;
use tegne_math::Vector4;

use crate::images::Framebuffer;
use crate::images::Texture;
use crate::instance::Device;
use crate::shaders::MaterialObject;
use crate::shaders::MaterialUniforms;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;

pub struct Material {
    pipeline: Pipeline,
    albedo_index: i32,
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
}

pub struct MaterialBuilder {
    pipeline: Pipeline,
    albedo_index: i32,
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
}

impl Material {
    pub(crate) fn builder(
        device: &Rc<Device>,
        default_shader: &Shader,
        default_albedo: &Texture,
        shader_layout: &ShaderLayout,
    ) -> MaterialBuilder {
        MaterialBuilder {
            pipeline: default_shader.pipeline(),
            albedo_index: default_albedo.image_index(),
            albedo_tint: Vector4::new(1.0, 1.0, 1.0, 1.0),
            uniforms: MaterialUniforms::new(device, shader_layout),
        }
    }

    pub(crate) fn pipeline(&self) -> Pipeline {
        self.pipeline
    }

    pub(crate) fn albedo_index(&self) -> i32 {
        self.albedo_index
    }

    pub(crate) fn uniforms(&self) -> &MaterialUniforms {
        &self.uniforms
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        let shaders = self.pipeline == other.pipeline;
        let albedoes = self.albedo_index == other.albedo_index;
        let tints = self.albedo_tint == other.albedo_tint;

        shaders && albedoes && tints
    }
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        self.uniforms.update(MaterialObject {
            albedo_tint: self.albedo_tint,
        });
        let material = Material {
            pipeline: self.pipeline,
            albedo_index: self.albedo_index,
            albedo_tint: self.albedo_tint,
            uniforms: self.uniforms,
        };
        material
    }

    pub fn with_shader(mut self, shader: &Shader) -> MaterialBuilder {
        self.pipeline = shader.pipeline();
        self
    }

    pub fn with_albedo(mut self, texture: &Texture) -> MaterialBuilder {
        self.albedo_index = texture.image_index();
        self
    }

    pub fn with_albedo_framebuffer(mut self, framebuffer: &Framebuffer) -> MaterialBuilder {
        self.albedo_index = framebuffer.image_index();
        self
    }

    pub fn with_albedo_tint(mut self, tint: impl Into<Vector4>) -> MaterialBuilder {
        self.albedo_tint = tint.into();
        self
    }
}
