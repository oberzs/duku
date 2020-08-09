// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Material - struct to pass additional data to shader

use std::sync::Arc;

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
    should_update: bool,
}

pub struct Arg(Vector4);

impl Material {
    pub(crate) fn new(device: &Arc<Device>, shader_layout: &ShaderLayout) -> Result<Self> {
        let uniform = MaterialUniform::new(device, shader_layout)?;

        Ok(Self {
            data: MaterialData::default(),
            should_update: true,
            uniform,
        })
    }

    pub(crate) fn update_if_needed(&mut self) -> Result<()> {
        if self.should_update {
            self.uniform.update(self.data)?;
            self.should_update = false;
        }
        Ok(())
    }

    pub(crate) fn uniform(&self) -> &MaterialUniform {
        &self.uniform
    }
}

impl Ref<Material> {
    pub fn set_phong_color(&self, color: impl Into<Color>) {
        self.with(|m| {
            let c = color.into().to_rgb_norm_vec();
            m.data.arg_1.x = c.x;
            m.data.arg_1.y = c.y;
            m.data.arg_1.z = c.z;
            m.should_update = true;
        });
    }

    pub fn set_font_color(&self, color: impl Into<Color>) {
        self.with(|m| {
            let c = color.into().to_rgb_norm_vec();
            m.data.arg_1.x = c.x;
            m.data.arg_1.y = c.y;
            m.data.arg_1.z = c.z;
            m.should_update = true;
        });
    }

    pub fn set_font_width(&self, width: f32) {
        self.with(|m| {
            m.data.arg_1.w = width;
            m.should_update = true;
        });
    }

    pub fn set_font_border_color(&self, color: impl Into<Color>) {
        self.with(|m| {
            let c = color.into().to_rgb_norm_vec();
            m.data.arg_2.x = c.x;
            m.data.arg_2.y = c.y;
            m.data.arg_2.z = c.z;
            m.should_update = true;
        });
    }

    pub fn set_font_edge(&self, edge: f32) {
        self.with(|m| {
            m.data.arg_2.w = edge;
            m.should_update = true;
        });
    }

    pub fn set_font_border_offset(&self, offset: impl Into<Vector2>) {
        self.with(|m| {
            let v = offset.into();
            m.data.arg_3.x = v.x;
            m.data.arg_3.y = v.y;
            m.should_update = true;
        });
    }

    pub fn set_font_border_width(&self, width: f32) {
        self.with(|m| {
            m.data.arg_3.z = width;
            m.should_update = true;
        });
    }

    pub fn set_font_border_edge(&self, edge: f32) {
        self.with(|m| {
            m.data.arg_3.w = edge;
            m.should_update = true;
        });
    }

    pub fn set_arg_1(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_1 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_2(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_2 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_3(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_3 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_4(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_4 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_5(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_5 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_6(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_6 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_7(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_7 = arg.into().0;
            m.should_update = true;
        });
    }

    pub fn set_arg_8(&self, arg: impl Into<Arg>) {
        self.with(|m| {
            m.data.arg_8 = arg.into().0;
            m.should_update = true;
        });
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.uniform == other.uniform
    }
}

impl From<i32> for Arg {
    fn from(i: i32) -> Self {
        Self(Vector4::new(i as f32, 0.0, 0.0, 0.0))
    }
}

impl From<Color> for Arg {
    fn from(color: Color) -> Self {
        Self(color.to_rgba_norm_vec())
    }
}

impl From<Vector2> for Arg {
    fn from(v: Vector2) -> Self {
        Self(v.extend(0.0).extend(0.0))
    }
}

impl From<Vector3> for Arg {
    fn from(v: Vector3) -> Self {
        Self(v.extend(0.0))
    }
}

impl From<&Ref<Texture>> for Arg {
    fn from(t: &Ref<Texture>) -> Self {
        let index = t.with(|tex| tex.image_index()) as f32;
        Self(Vector4::new(index, 0.0, 0.0, 0.0))
    }
}
