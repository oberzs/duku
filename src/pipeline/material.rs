// Oliver Berzs
// https://github.com/oberzs/duku

use super::Descriptor;
use super::ShaderMaterial;
use super::Uniforms;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Vector4;
use crate::renderer::Color;
use crate::resources::Handle;

pub struct Material {
    pub a: Vector4,
    pub b: Vector4,
    pub c: Vector4,
    pub d: Vector4,
    pub e: Vector4,
    pub f: Vector4,
    pub g: Vector4,
    pub h: Vector4,
    pub textures: Vec<Handle<Texture>>,

    descriptor: Descriptor,
    buffer: Buffer<ShaderMaterial>,
}

impl Material {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms) -> Result<Self> {
        let buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let descriptor = uniforms.material_set(device, &buffer)?;

        Ok(Self {
            a: Vector4::default(),
            b: Vector4::default(),
            c: Vector4::default(),
            d: Vector4::default(),
            e: Vector4::default(),
            f: Vector4::default(),
            g: Vector4::default(),
            h: Vector4::default(),
            textures: vec![],
            buffer,
            descriptor,
        })
    }

    pub fn albedo_color(&mut self, color: impl Into<Color>) {
        let temp = self.a[3];
        self.a = Vector4::from(color.into().to_rgba_norm());
        self.a[3] = temp;
    }

    pub fn albedo_texture(&mut self, texture: Handle<Texture>) {
        self.a[3] = texture.shader_index() as f32;
        self.textures.push(texture);
    }

    pub fn albedo_framebuffer(&mut self, f: &Handle<Framebuffer>) {
        self.a[3] = f.shader_index() as f32;
    }

    pub fn metalness(&mut self, value: f32) {
        self.b[0] = value;
    }

    pub fn roughness(&mut self, value: f32) {
        self.b[1] = value;
    }

    pub fn emissive(&mut self, color: impl Into<Color>) {
        let temp = self.d[3];
        self.d = Vector4::from(color.into().to_rgba_norm());
        self.d[3] = temp;
    }

    pub fn metalness_roughness_texture(&mut self, texture: Handle<Texture>) {
        self.b[2] = texture.shader_index() as f32;
        self.textures.push(texture);
    }

    pub fn ambient_occlusion_texture(&mut self, texture: Handle<Texture>) {
        self.b[3] = texture.shader_index() as f32;
        self.textures.push(texture);
    }

    pub fn normal_texture(&mut self, texture: Handle<Texture>) {
        self.c[0] = texture.shader_index() as f32;
        self.textures.push(texture);
    }

    pub fn emissive_texture(&mut self, texture: Handle<Texture>) {
        self.c[1] = texture.shader_index() as f32;
        self.textures.push(texture);
    }

    pub fn fix_albedo_color_space(&mut self) {
        let old = Color::from([self.a[0], self.a[1], self.a[2]]);
        let new = old.to_linear();
        let temp = self.a[3];
        self.a = Vector4::from(new.to_rgba_norm());
        self.a[3] = temp;
    }

    pub(crate) fn update(&mut self) {
        self.buffer.copy_from_data(&[ShaderMaterial {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            g: self.g,
            h: self.h,
        }]);
    }

    pub(crate) const fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.buffer.destroy(device);
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}
