use std::cell::Cell;
use std::rc::Rc;
use tegne_math::Vector4;

use super::Descriptor;
use super::MaterialObject;
use super::MaterialUniforms;
use super::ShaderLayout;
use crate::instance::Device;

pub struct Material {
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
    should_update: Cell<bool>,
}

pub struct MaterialBuilder {
    albedo_tint: Vector4,
    uniforms: MaterialUniforms,
}

impl Material {
    pub(crate) fn builder(device: &Rc<Device>, shader_layout: &ShaderLayout) -> MaterialBuilder {
        MaterialBuilder {
            albedo_tint: Vector4::new(1.0, 1.0, 1.0, 1.0),
            uniforms: MaterialUniforms::new(device, shader_layout),
        }
    }

    pub fn set_albedo_tint(&mut self, tint: impl Into<Vector4>) {
        self.albedo_tint = tint.into();
        self.should_update.set(true);
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        if self.should_update.get() {
            self.uniforms.update(MaterialObject {
                albedo_tint: self.albedo_tint,
            });
        }
        self.uniforms.descriptor()
    }
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        self.uniforms.update(MaterialObject {
            albedo_tint: self.albedo_tint,
        });
        Material {
            albedo_tint: self.albedo_tint,
            uniforms: self.uniforms,
            should_update: Cell::new(false),
        }
    }

    pub fn with_albedo_tint(mut self, tint: impl Into<Vector4>) -> MaterialBuilder {
        self.albedo_tint = tint.into();
        self
    }
}
