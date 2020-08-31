// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Material - struct to pass additional data to shader

use std::rc::Rc;
use std::sync::mpsc::Sender;

use super::Descriptor;
use super::ShaderLayout;
use crate::buffer::BufferUsage;
use crate::buffer::DynamicBuffer;
use crate::color::Color;
use crate::device::Device;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::storage::Index;

// user facing Material data
#[derive(Debug)]
pub struct Material {
    pub arg_1: Vector4,
    pub arg_2: Vector4,
    pub arg_3: Vector4,
    pub arg_4: Vector4,
    pub arg_5: Vector4,
    pub arg_6: Vector4,
    pub arg_7: Vector4,
    pub arg_8: Vector4,

    pub(crate) index: Index,

    updater: Sender<(Index, MaterialUpdateData)>,
}

pub struct Arg(Vector4);

// GPU data storage for a material
pub(crate) struct CoreMaterial {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct MaterialUpdateData {
    pub(crate) arg_1: Vector4,
    pub(crate) arg_2: Vector4,
    pub(crate) arg_3: Vector4,
    pub(crate) arg_4: Vector4,
    pub(crate) arg_5: Vector4,
    pub(crate) arg_6: Vector4,
    pub(crate) arg_7: Vector4,
    pub(crate) arg_8: Vector4,
}

impl Material {
    pub(crate) fn new(index: Index, updater: Sender<(Index, MaterialUpdateData)>) -> Self {
        Self {
            arg_1: Vector4::ZERO,
            arg_2: Vector4::ZERO,
            arg_3: Vector4::ZERO,
            arg_4: Vector4::ZERO,
            arg_5: Vector4::ZERO,
            arg_6: Vector4::ZERO,
            arg_7: Vector4::ZERO,
            arg_8: Vector4::ZERO,
            updater,
            index,
        }
    }

    pub fn set_phong_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.arg_1.x = c.x;
        self.arg_1.y = c.y;
        self.arg_1.z = c.z;
    }

    pub fn set_font_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.arg_1.x = c.x;
        self.arg_1.y = c.y;
        self.arg_1.z = c.z;
    }

    pub fn set_arg_1(&mut self, arg: impl Into<Arg>) {
        self.arg_1 = arg.into().0;
    }

    pub fn set_arg_2(&mut self, arg: impl Into<Arg>) {
        self.arg_2 = arg.into().0;
    }

    pub fn set_arg_3(&mut self, arg: impl Into<Arg>) {
        self.arg_3 = arg.into().0;
    }

    pub fn set_arg_4(&mut self, arg: impl Into<Arg>) {
        self.arg_4 = arg.into().0;
    }

    pub fn set_arg_5(&mut self, arg: impl Into<Arg>) {
        self.arg_5 = arg.into().0;
    }

    pub fn set_arg_6(&mut self, arg: impl Into<Arg>) {
        self.arg_6 = arg.into().0;
    }

    pub fn set_arg_7(&mut self, arg: impl Into<Arg>) {
        self.arg_7 = arg.into().0;
    }

    pub fn set_arg_8(&mut self, arg: impl Into<Arg>) {
        self.arg_8 = arg.into().0;
    }

    pub fn update(&self) {
        let data = MaterialUpdateData {
            arg_1: self.arg_1,
            arg_2: self.arg_2,
            arg_3: self.arg_3,
            arg_4: self.arg_4,
            arg_5: self.arg_5,
            arg_6: self.arg_6,
            arg_7: self.arg_7,
            arg_8: self.arg_8,
        };
        self.updater
            .send((self.index.clone(), data))
            .expect("bad receiver");
    }
}

impl CoreMaterial {
    pub(crate) fn new(device: &Rc<Device>, shader_layout: &ShaderLayout) -> Self {
        let buffer = DynamicBuffer::new::<MaterialUpdateData>(device, BufferUsage::Uniform, 1);
        let descriptor = shader_layout.material_set(&buffer);

        Self { buffer, descriptor }
    }

    pub(crate) fn update(&mut self, data: MaterialUpdateData) {
        self.buffer.update_data(&[data]);
    }

    pub(crate) const fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl PartialEq for CoreMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
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
