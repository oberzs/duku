// Oliver Berzs
// https://github.com/oberzs/duku

use std::ops::Index;
use std::ops::IndexMut;

use super::Descriptor;
use super::ShaderMaterial;
use super::Uniforms;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::renderer::Color;
use crate::storage::Handle;
use crate::storage::Storage;

pub struct Material {
    a: MaterialParam,
    b: MaterialParam,
    c: MaterialParam,
    d: MaterialParam,
    e: MaterialParam,
    f: MaterialParam,
    g: MaterialParam,
    h: MaterialParam,

    should_update: bool,

    descriptor: Descriptor,
    buffer: Buffer<ShaderMaterial>,
    pub(crate) textures: Vec<Handle<Texture>>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MaterialParam(pub [f32; 4]);

pub struct MaterialBuilder<'s> {
    pub(crate) storage: &'s mut Storage,
    pub(crate) material: Material,
}

impl Material {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms) -> Result<Self> {
        let buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let descriptor = uniforms.material_set(device, &buffer)?;

        Ok(Self {
            a: MaterialParam::default(),
            b: MaterialParam::default(),
            c: MaterialParam::default(),
            d: MaterialParam::default(),
            e: MaterialParam::default(),
            f: MaterialParam::default(),
            g: MaterialParam::default(),
            h: MaterialParam::default(),
            textures: vec![],
            should_update: true,
            buffer,
            descriptor,
        })
    }

    pub fn set_albedo_color(&mut self, color: impl Into<Color>) {
        let temp = self.a[3];
        self.a = MaterialParam::from(color.into());
        self.a[3] = temp;
        self.should_update = true;
    }

    pub fn set_albedo_texture(&mut self, texture: Handle<Texture>) {
        self.a[3] = texture.id() as f32;
        self.textures.push(texture);
        self.should_update = true;
    }

    pub fn set_albedo_framebuffer(&mut self, f: &Handle<Framebuffer>) {
        self.a[3] = f.id() as f32;
        self.should_update = true;
    }

    pub fn set_metalness(&mut self, value: f32) {
        self.b[0] = value;
        self.should_update = true;
    }

    pub fn set_roughness(&mut self, value: f32) {
        self.b[1] = value;
        self.should_update = true;
    }

    pub fn set_emissive(&mut self, color: impl Into<Color>) {
        let temp = self.d[3];
        self.d = MaterialParam::from(color.into());
        self.d[3] = temp;
        self.should_update = true;
    }

    pub fn set_metalness_roughness_texture(&mut self, texture: Handle<Texture>) {
        self.b[2] = texture.id() as f32;
        self.textures.push(texture);
        self.should_update = true;
    }

    pub fn set_ambient_occlusion_texture(&mut self, texture: Handle<Texture>) {
        self.b[3] = texture.id() as f32;
        self.textures.push(texture);
        self.should_update = true;
    }

    pub fn set_normal_texture(&mut self, texture: Handle<Texture>) {
        self.c[0] = texture.id() as f32;
        self.textures.push(texture);
        self.should_update = true;
    }

    pub fn set_emissive_texture(&mut self, texture: Handle<Texture>) {
        self.c[1] = texture.id() as f32;
        self.textures.push(texture);
        self.should_update = true;
    }

    pub fn fix_albedo_color_space(&mut self) {
        let old = Color::from([self.a[0], self.a[1], self.a[2]]);
        let new = old.to_linear();
        let temp = self.a[3];
        self.a = MaterialParam::from(new);
        self.a[3] = temp;
        self.should_update = true;
    }

    pub fn set_a(&mut self, param: impl Into<MaterialParam>) {
        self.a = param.into();
        self.should_update = true;
    }

    pub fn set_b(&mut self, param: impl Into<MaterialParam>) {
        self.b = param.into();
        self.should_update = true;
    }

    pub fn set_c(&mut self, param: impl Into<MaterialParam>) {
        self.c = param.into();
        self.should_update = true;
    }

    pub fn set_d(&mut self, param: impl Into<MaterialParam>) {
        self.d = param.into();
        self.should_update = true;
    }

    pub fn set_e(&mut self, param: impl Into<MaterialParam>) {
        self.e = param.into();
        self.should_update = true;
    }

    pub fn set_f(&mut self, param: impl Into<MaterialParam>) {
        self.f = param.into();
        self.should_update = true;
    }

    pub fn set_g(&mut self, param: impl Into<MaterialParam>) {
        self.g = param.into();
        self.should_update = true;
    }

    pub fn set_h(&mut self, param: impl Into<MaterialParam>) {
        self.h = param.into();
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self) {
        if self.should_update {
            self.buffer.copy_from_data(&[ShaderMaterial {
                a: self.a.0.into(),
                b: self.b.0.into(),
                c: self.c.0.into(),
                d: self.d.0.into(),
                e: self.e.0.into(),
                f: self.f.0.into(),
                g: self.g.0.into(),
                h: self.h.0.into(),
            }]);
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

    pub fn albedo_texture(mut self, texture: Handle<Texture>) -> Self {
        self.material.set_albedo_texture(texture);
        self
    }

    pub fn albedo_framebuffer(mut self, framebuffer: &Handle<Framebuffer>) -> Self {
        self.material.set_albedo_framebuffer(framebuffer);
        self
    }

    pub fn metalness(mut self, value: f32) -> Self {
        self.material.set_metalness(value);
        self
    }

    pub fn roughness(mut self, value: f32) -> Self {
        self.material.set_roughness(value);
        self
    }

    pub fn emissive(mut self, color: impl Into<Color>) -> Self {
        self.material.set_emissive(color);
        self
    }

    pub fn metalness_roughness_texture(mut self, texture: Handle<Texture>) -> Self {
        self.material.set_metalness_roughness_texture(texture);
        self
    }

    pub fn ambient_occlusion_texture(mut self, texture: Handle<Texture>) -> Self {
        self.material.set_ambient_occlusion_texture(texture);
        self
    }

    pub fn normal_texture(mut self, texture: Handle<Texture>) -> Self {
        self.material.set_normal_texture(texture);
        self
    }

    pub fn emissive_texture(mut self, texture: Handle<Texture>) -> Self {
        self.material.set_emissive_texture(texture);
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

impl Index<usize> for MaterialParam {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for MaterialParam {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl From<&Handle<Texture>> for MaterialParam {
    fn from(h: &Handle<Texture>) -> Self {
        Self([h.id() as f32, 0.0, 0.0, 0.0])
    }
}

impl From<&Handle<Framebuffer>> for MaterialParam {
    fn from(h: &Handle<Framebuffer>) -> Self {
        Self([h.id() as f32, 0.0, 0.0, 0.0])
    }
}

impl From<[f32; 4]> for MaterialParam {
    fn from(v: [f32; 4]) -> Self {
        Self(v)
    }
}

impl From<[f32; 3]> for MaterialParam {
    fn from(v: [f32; 3]) -> Self {
        Self([v[0], v[1], v[2], 0.0])
    }
}

impl From<[f32; 2]> for MaterialParam {
    fn from(v: [f32; 2]) -> Self {
        Self([v[0], v[1], 0.0, 0.0])
    }
}

impl From<f32> for MaterialParam {
    fn from(v: f32) -> Self {
        Self([v, 0.0, 0.0, 0.0])
    }
}

impl From<Vector3> for MaterialParam {
    fn from(v: Vector3) -> Self {
        Self([v.x, v.y, v.z, 0.0])
    }
}

impl From<Vector2> for MaterialParam {
    fn from(v: Vector2) -> Self {
        Self([v.x, v.y, 0.0, 0.0])
    }
}

impl From<Color> for MaterialParam {
    fn from(c: Color) -> Self {
        Self(c.to_rgba_norm())
    }
}
