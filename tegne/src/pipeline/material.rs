// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Material - struct to pass additional data to shader

use std::cell::Cell;
use std::sync::Arc;

use super::Descriptor;
use super::MaterialData;
use super::MaterialUniform;
use super::ShaderLayout;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::resource::Ref;

pub struct Material {
    data: MaterialData,
    uniform: MaterialUniform,
    should_update: Cell<bool>,
}

pub type Arg = Vector4;

impl Material {
    pub(crate) fn new(device: &Arc<Device>, shader_layout: &ShaderLayout) -> Result<Self> {
        let uniform = MaterialUniform::new(device, shader_layout)?;

        Ok(Self {
            data: MaterialData::default(),
            should_update: Cell::new(true),
            uniform,
        })
    }

    pub fn set_phong_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.data.arg_1.x = c.x;
        self.data.arg_1.y = c.y;
        self.data.arg_1.z = c.z;
        self.should_update.set(true);
    }

    pub fn set_font_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.data.arg_1.x = c.x;
        self.data.arg_1.y = c.y;
        self.data.arg_1.z = c.z;
        self.should_update.set(true);
    }

    pub fn set_font_width(&mut self, width: f32) {
        self.data.arg_1.w = width;
        self.should_update.set(true);
    }

    pub fn set_font_border_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.data.arg_2.x = c.x;
        self.data.arg_2.y = c.y;
        self.data.arg_2.z = c.z;
        self.should_update.set(true);
    }

    pub fn set_font_edge(&mut self, edge: f32) {
        self.data.arg_2.w = edge;
        self.should_update.set(true);
    }

    pub fn set_font_border_offset(&mut self, offset: impl Into<Vector2>) {
        let v = offset.into();
        self.data.arg_3.x = v.x;
        self.data.arg_3.y = v.y;
        self.should_update.set(true);
    }

    pub fn set_font_border_width(&mut self, width: f32) {
        self.data.arg_3.z = width;
        self.should_update.set(true);
    }

    pub fn set_font_border_edge(&mut self, edge: f32) {
        self.data.arg_3.w = edge;
        self.should_update.set(true);
    }

    pub fn set_arg_1(&mut self, arg: impl Into<Arg>) {
        self.data.arg_1 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_2(&mut self, arg: impl Into<Arg>) {
        self.data.arg_2 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_3(&mut self, arg: impl Into<Arg>) {
        self.data.arg_3 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_4(&mut self, arg: impl Into<Arg>) {
        self.data.arg_4 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_5(&mut self, arg: impl Into<Arg>) {
        self.data.arg_5 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_6(&mut self, arg: impl Into<Arg>) {
        self.data.arg_6 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_7(&mut self, arg: impl Into<Arg>) {
        self.data.arg_7 = arg.into();
        self.should_update.set(true);
    }

    pub fn set_arg_8(&mut self, arg: impl Into<Arg>) {
        self.data.arg_8 = arg.into();
        self.should_update.set(true);
    }

    pub(crate) fn descriptor(&self) -> Result<Descriptor> {
        // update material uniform if data has changed
        if self.should_update.get() {
            self.uniform.update(self.data)?;
            self.should_update.set(false);
        }
        Ok(self.uniform.descriptor())
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.uniform == other.uniform
    }
}

impl From<i32> for Arg {
    fn from(i: i32) -> Self {
        Vector4::new(i as f32, 0.0, 0.0, 0.0)
    }
}

impl From<Color> for Arg {
    fn from(color: Color) -> Self {
        color.to_rgba_norm_vec()
    }
}

impl From<Vector2> for Arg {
    fn from(v: Vector2) -> Self {
        v.extend(0.0).extend(0.0)
    }
}

impl From<Vector3> for Arg {
    fn from(v: Vector3) -> Self {
        v.extend(0.0)
    }
}

impl From<&Ref<Texture>> for Arg {
    fn from(t: &Ref<Texture>) -> Self {
        let index = t.with(|tex| tex.image_index()) as f32;
        Vector4::new(index, 0.0, 0.0, 0.0)
    }
}
