use std::cell::Cell;
use std::rc::Rc;
use tegne_math::Vector2;
use tegne_math::Vector3;

use super::Descriptor;
use super::MaterialObject;
use super::MaterialUniforms;
use super::ShaderLayout;
use crate::instance::Device;

pub struct Material {
    albedo_tint: Vector3,
    font_width: f32,
    font_edge: f32,
    font_border_width: f32,
    font_border_edge: f32,
    font_border_tint: Vector3,
    font_border_offset: Vector2,
    uniforms: MaterialUniforms,
    should_update: Cell<bool>,
}

pub struct MaterialBuilder {
    albedo_tint: Vector3,
    font_width: f32,
    font_edge: f32,
    font_border_width: f32,
    font_border_edge: f32,
    font_border_tint: Vector3,
    font_border_offset: Vector2,
    uniforms: MaterialUniforms,
}

impl Material {
    pub(crate) fn builder(device: &Rc<Device>, shader_layout: &ShaderLayout) -> MaterialBuilder {
        MaterialBuilder {
            albedo_tint: Vector3::new(1.0, 1.0, 1.0),
            font_width: 0.5,
            font_edge: 0.1,
            font_border_width: 0.0,
            font_border_edge: 0.0,
            font_border_tint: Vector3::new(1.0, 1.0, 1.0),
            font_border_offset: Vector2::new(0.0, 0.0),
            uniforms: MaterialUniforms::new(device, shader_layout),
        }
    }

    pub fn set_albedo_tint(&mut self, tint: impl Into<Vector3>) {
        self.albedo_tint = tint.into();
        self.should_update.set(true);
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        if self.should_update.get() {
            self.uniforms.update(MaterialObject {
                albedo_tint: self.albedo_tint,
                font_width: self.font_width,
                font_edge: self.font_edge,
                font_border_width: self.font_border_width,
                font_border_edge: self.font_border_edge,
                font_border_tint: self.font_border_tint,
                font_border_offset: self.font_border_offset,
            });
        }
        self.uniforms.descriptor()
    }
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        self.uniforms.update(MaterialObject {
            albedo_tint: self.albedo_tint,
            font_width: self.font_width,
            font_edge: self.font_edge,
            font_border_width: self.font_border_width,
            font_border_edge: self.font_border_edge,
            font_border_tint: self.font_border_tint,
            font_border_offset: self.font_border_offset,
        });
        Material {
            albedo_tint: self.albedo_tint,
            font_width: self.font_width,
            font_edge: self.font_edge,
            font_border_width: self.font_border_width,
            font_border_edge: self.font_border_edge,
            font_border_tint: self.font_border_tint,
            font_border_offset: self.font_border_offset,
            uniforms: self.uniforms,
            should_update: Cell::new(false),
        }
    }

    pub fn with_albedo_tint(mut self, tint: impl Into<Vector3>) -> MaterialBuilder {
        self.albedo_tint = tint.into();
        self
    }
}
