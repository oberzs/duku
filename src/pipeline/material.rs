// Oliver Berzs
// https://github.com/oberzs/draw-it

// Material - struct to pass additional data to shader

use super::Descriptor;
use super::ShaderLayout;
use super::ShaderMaterial;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Color;
use crate::device::Device;
use crate::image::Texture;
use crate::math::Vector4;
use crate::storage::Handle;
use crate::storage::Storage;

pub struct Material {
    arg_1: Vector4,
    arg_2: Vector4,
    arg_3: Vector4,
    arg_4: Vector4,
    arg_5: Vector4,
    arg_6: Vector4,
    arg_7: Vector4,
    arg_8: Vector4,

    should_update: bool,

    descriptor: Descriptor,
    buffer: Buffer<ShaderMaterial>,
}

pub struct MaterialBuilder<'s> {
    pub(crate) storage: &'s mut Storage,
    pub(crate) material: Material,
}

impl Material {
    pub(crate) fn new(device: &Device, shader_layout: &ShaderLayout) -> Self {
        let buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let descriptor = shader_layout.material_set(device, &buffer);

        Self {
            arg_1: Vector4::ZERO,
            arg_2: Vector4::ZERO,
            arg_3: Vector4::ZERO,
            arg_4: Vector4::ZERO,
            arg_5: Vector4::ZERO,
            arg_6: Vector4::ZERO,
            arg_7: Vector4::ZERO,
            arg_8: Vector4::ZERO,
            should_update: true,
            buffer,
            descriptor,
        }
    }

    pub fn set_albedo_color(&mut self, color: impl Into<Color>) {
        let c = color.into().to_rgb_norm_vec();
        self.arg_1.x = c.x;
        self.arg_1.y = c.y;
        self.arg_1.z = c.z;
        self.should_update = true;
    }

    pub fn set_albedo_texture(&mut self, texture: &Handle<Texture>) {
        self.arg_1.w = texture.id() as f32;
        self.should_update = true;
    }

    pub fn set_arg_1<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_1 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_2<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_2 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_3<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_3 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_4<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_4 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_5<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_5 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_6<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_6 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_7<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_7 = arg.into();
        self.should_update = true;
    }

    pub fn set_arg_8<V: Into<Vector4>>(&mut self, arg: V) {
        self.arg_8 = arg.into();
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device) {
        if self.should_update {
            self.buffer.copy_from_data(
                device,
                &[ShaderMaterial {
                    arg_1: self.arg_1,
                    arg_2: self.arg_2,
                    arg_3: self.arg_3,
                    arg_4: self.arg_4,
                    arg_5: self.arg_5,
                    arg_6: self.arg_6,
                    arg_7: self.arg_7,
                    arg_8: self.arg_8,
                }],
            );
            self.should_update = false;
        }
    }

    pub(crate) const fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.buffer.destroy(device);
    }
}

impl MaterialBuilder<'_> {
    pub fn albedo_color(mut self, color: impl Into<Color>) -> Self {
        self.material.set_albedo_color(color);
        self
    }

    pub fn albedo_texture(mut self, texture: &Handle<Texture>) -> Self {
        self.material.set_albedo_texture(texture);
        self
    }

    pub fn build(self) -> Handle<Material> {
        self.storage.add_material(self.material)
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}
