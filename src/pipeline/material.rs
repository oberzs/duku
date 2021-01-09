// Oliver Berzs
// https://github.com/oberzs/duku

use super::Descriptor;
use super::ShaderMaterial;
use super::Uniforms;
use crate::buffer::Buffer;
use crate::buffer::BufferUsage;
use crate::color::Rgbf;
use crate::device::Device;
use crate::image::Canvas;
use crate::image::Texture;
use crate::math::Vec4;
use crate::resources::Handle;

/// Material parameters to use in a shader.
///
/// # Examples
///
/// ```no_run
/// # use duku::Duku;
/// # let (mut duku, _) = Duku::windowed(1, 1).unwrap();
/// let material = duku.create_material_pbr().unwrap();
/// material.write().roughness(0.5);
///
/// # duku.draw(None, |t| {
/// // when drawing
/// t.material(&material);
/// t.cube([1.0, 1.0, 1.0]);
/// # });
/// ```
pub struct Material {
    /// parameter A
    pub a: Vec4,
    /// parameter B
    pub b: Vec4,
    /// parameter C
    pub c: Vec4,
    /// parameter D
    pub d: Vec4,
    /// parameter E
    pub e: Vec4,
    /// parameter F
    pub f: Vec4,
    /// parameter G
    pub g: Vec4,
    /// parameter H
    pub h: Vec4,
    /// texture storage for that are used
    /// in the material
    pub textures: Vec<Handle<Texture>>,

    descriptor: Descriptor,
    buffer: Buffer<ShaderMaterial>,
}

impl Material {
    pub(crate) fn new(device: &Device, uniforms: &mut Uniforms) -> Self {
        let buffer = Buffer::dynamic(device, BufferUsage::Uniform, 1);
        let descriptor = uniforms.material_set(device, &buffer);

        Self {
            a: Vec4::default(),
            b: Vec4::default(),
            c: Vec4::default(),
            d: Vec4::default(),
            e: Vec4::default(),
            f: Vec4::default(),
            g: Vec4::default(),
            h: Vec4::default(),
            textures: vec![],
            buffer,
            descriptor,
        }
    }

    /// Set albedo color for the PBR and other various shaders
    pub fn albedo_color(&mut self, color: impl Into<Rgbf>) {
        let temp = self.a[3];
        self.a = color.into().into();
        self.a[3] = temp;
    }

    /// Set albedo texture for the PBR and other various shaders
    pub fn albedo_texture(&mut self, texture: Handle<Texture>) {
        self.a[3] = texture.read().shader_index() as f32;
        self.textures.push(texture);
    }

    /// Set albedo canvas for the PBR and other various shaders
    pub fn albedo_canvas(&mut self, f: &Handle<Canvas>) {
        self.a[3] = f.read().shader_index(0).expect("invalid canvas") as f32;
    }

    /// Set metalness factor for the PBR shader
    pub fn metalness(&mut self, value: f32) {
        self.b[0] = value;
    }

    /// Set roughness factor for the PBR shader
    pub fn roughness(&mut self, value: f32) {
        self.b[1] = value;
    }

    /// Set emissive color for the PBR shader
    pub fn emissive(&mut self, color: impl Into<Rgbf>) {
        let temp = self.d[3];
        self.d = color.into().into();
        self.d[3] = temp;
    }

    /// Set metalness-roughness texture for the PBR shader
    pub fn metalness_roughness_texture(&mut self, texture: Handle<Texture>) {
        self.b[2] = texture.read().shader_index() as f32;
        self.textures.push(texture);
    }

    /// Set ambient occlusion texture for the PBR shader
    pub fn ambient_occlusion_texture(&mut self, texture: Handle<Texture>) {
        self.b[3] = texture.read().shader_index() as f32;
        self.textures.push(texture);
    }

    /// Set normal texture for the PBR shader
    pub fn normal_texture(&mut self, texture: Handle<Texture>) {
        self.c[0] = texture.read().shader_index() as f32;
        self.textures.push(texture);
    }

    /// Set emissive texture for the PBR shader
    pub fn emissive_texture(&mut self, texture: Handle<Texture>) {
        self.c[1] = texture.read().shader_index() as f32;
        self.textures.push(texture);
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
