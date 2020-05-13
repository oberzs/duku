use std::cell::Cell;
use std::rc::Rc;
use tegne_math::Vector2;
use tegne_math::Vector3;
use tegne_math::Vector4;

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
    arg_1: Vector4,
    arg_2: Vector4,
    arg_3: Vector4,
    arg_4: Vector4,
    uniforms: MaterialUniforms,
    should_update: Cell<bool>,
}

#[derive(Debug, Copy, Clone)]
pub struct MaterialOptions {
    pub albedo_tint: Vector3,
    pub font_width: f32,
    pub font_edge: f32,
    pub font_border_width: f32,
    pub font_border_edge: f32,
    pub font_border_tint: Vector3,
    pub font_border_offset: Vector2,
    pub arg_1: Vector4,
    pub arg_2: Vector4,
    pub arg_3: Vector4,
    pub arg_4: Vector4,
}

impl Material {
    pub(crate) fn new(
        device: &Rc<Device>,
        shader_layout: &ShaderLayout,
        options: MaterialOptions,
    ) -> Self {
        let uniforms = MaterialUniforms::new(device, shader_layout);

        Self {
            albedo_tint: options.albedo_tint,
            font_width: options.font_width,
            font_edge: options.font_edge,
            font_border_width: options.font_border_width,
            font_border_edge: options.font_border_edge,
            font_border_tint: options.font_border_tint,
            font_border_offset: options.font_border_offset,
            arg_1: options.arg_1,
            arg_2: options.arg_2,
            arg_3: options.arg_3,
            arg_4: options.arg_4,
            uniforms,
            should_update: Cell::new(true),
        }
    }

    pub fn set_albedo_tint(&mut self, tint: impl Into<Vector3>) {
        self.albedo_tint = tint.into();
        self.should_update.set(true);
    }

    pub fn set_font_width(&mut self, width: f32) {
        self.font_width = width;
        self.should_update.set(true);
    }

    pub fn set_font_edge(&mut self, edge: f32) {
        self.font_edge = edge;
        self.should_update.set(true);
    }

    pub fn set_font_border_width(&mut self, width: f32) {
        self.font_border_width = width;
        self.should_update.set(true);
    }

    pub fn set_font_border_edge(&mut self, edge: f32) {
        self.font_border_edge = edge;
        self.should_update.set(true);
    }

    pub fn set_font_border_tint(&mut self, tint: impl Into<Vector3>) {
        self.font_border_tint = tint.into();
        self.should_update.set(true);
    }

    pub fn set_font_border_offset(&mut self, offset: impl Into<Vector2>) {
        self.font_border_offset = offset.into();
        self.should_update.set(true);
    }

    pub fn set_arg_1(&mut self, arg: impl Into<Vector4>) {
        self.arg_1 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_2(&mut self, arg: impl Into<Vector4>) {
        self.arg_2 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_3(&mut self, arg: impl Into<Vector4>) {
        self.arg_3 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_4(&mut self, arg: impl Into<Vector4>) {
        self.arg_4 = arg.into();
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
                arg_1: self.arg_1,
                arg_2: self.arg_2,
                arg_3: self.arg_3,
                arg_4: self.arg_4,
            });
        }
        self.uniforms.descriptor()
    }
}

impl Default for MaterialOptions {
    fn default() -> Self {
        Self {
            albedo_tint: Vector3::new(1.0, 1.0, 1.0),
            font_width: 0.5,
            font_edge: 0.1,
            font_border_width: 0.0,
            font_border_edge: 0.1,
            font_border_tint: Vector3::default(),
            font_border_offset: Vector2::default(),
            arg_1: Vector4::default(),
            arg_2: Vector4::default(),
            arg_3: Vector4::default(),
            arg_4: Vector4::default(),
        }
    }
}
