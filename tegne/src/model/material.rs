use log::debug;
use log::info;
use std::cmp::Ordering;
use std::rc::Rc;
use tegne_math::Vector4;

use crate::images::Texture;
use crate::shaders::MaterialObject;
use crate::shaders::MaterialUniforms;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::surface::Framebuffer;
use crate::tegne::Device;
use crate::utils::error;

#[derive(PartialEq, PartialOrd)]
pub enum ShaderOption {
    Passthru,
    Texture,
    Unshaded,
    Shader(Shader),
}

#[derive(PartialEq)]
pub enum TextureOption {
    White,
    Texture(Texture),
    Framebuffer(Framebuffer),
}

pub struct Material {
    shader: ShaderOption,
    albedo: TextureOption,
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
}

pub struct MaterialBuilder {
    shader: ShaderOption,
    albedo: TextureOption,
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
    _device: Rc<Device>,
}

impl Material {
    pub fn from_shader(device: &Rc<Device>, shader_layout: &ShaderLayout, shader: Shader) -> Self {
        let uniforms = MaterialUniforms::new(device, shader_layout);
        let albedo_tint = Vector4::new(1.0, 1.0, 1.0, 1.0);
        uniforms.update(MaterialObject { albedo_tint });

        Self {
            shader: ShaderOption::Shader(shader),
            albedo: TextureOption::White,
            albedo_tint,
            uniforms,
        }
    }

    pub fn builder(device: &Rc<Device>, shader_layout: &ShaderLayout) -> MaterialBuilder {
        MaterialBuilder {
            shader: ShaderOption::Texture,
            albedo: TextureOption::White,
            albedo_tint: Vector4::new(1.0, 1.0, 1.0, 1.0),
            uniforms: MaterialUniforms::new(device, shader_layout),
            _device: Rc::clone(device),
        }
    }

    pub fn shader(&self) -> &ShaderOption {
        &self.shader
    }

    pub fn albedo(&self) -> &TextureOption {
        &self.albedo
    }

    pub fn albedo_tint(&self) -> Vector4 {
        self.albedo_tint
    }

    pub fn uniforms(&self) -> &MaterialUniforms {
        &self.uniforms
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        match &self.albedo {
            TextureOption::Framebuffer(framebuffer) => &framebuffer,
            _ => error("albedo is not a framebuffer"),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        let shaders = self.shader == other.shader;
        let albedoes = self.albedo == other.albedo;
        let tints = self.albedo_tint == other.albedo_tint;

        shaders && albedoes && tints
    }
}

impl PartialOrd for Material {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.shader == ShaderOption::Passthru {
            if self.shader == other.shader {
                return Some(Ordering::Equal);
            } else {
                return Some(Ordering::Greater);
            }
        }
        self.shader.partial_cmp(&other.shader)
    }
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        debug!("build material");
        self.uniforms.update(MaterialObject {
            albedo_tint: self.albedo_tint,
        });
        let material = Material {
            shader: self.shader,
            albedo: self.albedo,
            albedo_tint: self.albedo_tint,
            uniforms: self.uniforms,
        };
        info!("material built");
        material
    }

    pub fn with_passthru_shader(mut self) -> MaterialBuilder {
        self.shader = ShaderOption::Passthru;
        self
    }

    pub fn with_unshaded_shader(mut self) -> MaterialBuilder {
        self.shader = ShaderOption::Unshaded;
        self
    }

    pub fn with_albedo(mut self, texture: Texture) -> MaterialBuilder {
        self.albedo = TextureOption::Texture(texture);
        self
    }

    pub fn with_albedo_tint(mut self, tint: impl Into<Vector4>) -> MaterialBuilder {
        self.albedo_tint = tint.into();
        self
    }
}
