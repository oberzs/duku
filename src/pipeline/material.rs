// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Material - struct to pass additional data to shader

use std::rc::Rc;
use std::sync::mpsc::Sender;

use super::Descriptor;
use super::ShaderLayout;
use super::ShaderMaterial;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Color;
use crate::device::Device;
use crate::image::Texture;
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

    updater: Sender<(Index, ShaderMaterial)>,
}

pub struct MaterialBuilder {
    material: Material,
}

// data storage for a material
pub(crate) struct CoreMaterial {
    descriptor: Descriptor,
    buffer: Buffer<ShaderMaterial>,
}

impl Material {
    pub(crate) fn new(index: Index, updater: Sender<(Index, ShaderMaterial)>) -> Self {
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

    pub fn set_phong_texture(&mut self, texture: &Texture) {
        self.arg_1.w = texture.shader_index as f32;
    }

    pub fn update(&self) {
        let data = ShaderMaterial {
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

impl MaterialBuilder {
    pub(crate) fn new(material: Material) -> Self {
        Self { material }
    }

    pub fn phong_color<C: Into<Color>>(mut self, color: C) -> Self {
        let c = color.into().to_rgb_norm_vec();
        self.material.arg_1.x = c.x;
        self.material.arg_1.y = c.y;
        self.material.arg_1.z = c.z;
        self
    }

    pub fn phong_texture(mut self, texture: &Texture) -> Self {
        self.material.arg_1.w = texture.shader_index as f32;
        self
    }

    pub fn build(self) -> Material {
        self.material.update();
        self.material
    }
}

impl CoreMaterial {
    pub(crate) fn new(device: &Rc<Device>, shader_layout: &ShaderLayout) -> Self {
        let buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let descriptor = shader_layout.material_set(&buffer);

        Self { buffer, descriptor }
    }

    pub(crate) fn update(&mut self, data: ShaderMaterial) {
        self.buffer.copy_from_data(&[data]);
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
