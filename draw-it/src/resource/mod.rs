// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ResourceManager - resource manager

mod builtin;
mod storage;

use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::mesh::Mesh;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use storage::Storage;

pub(crate) use builtin::Builtins;
pub use storage::Ref;

pub(crate) struct ResourceManager {
    textures: Vec<Storage<Texture>>,
    materials: Vec<Storage<Material>>,
    meshes: Vec<Storage<Mesh>>,
    shaders: Vec<Storage<Shader>>,
    fonts: Vec<Storage<Font>>,
    framebuffers: Vec<Storage<Framebuffer>>,
}

impl ResourceManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: vec![],
            materials: vec![],
            meshes: vec![],
            shaders: vec![],
            fonts: vec![],
            framebuffers: vec![],
        }
    }

    pub(crate) fn add_texture(&mut self, texture: Texture) -> Ref<Texture> {
        let storage = Storage::new(texture);
        let reference = storage.as_ref();
        self.textures.push(storage);
        reference
    }

    pub(crate) fn add_material(&mut self, material: Material) -> Ref<Material> {
        let storage = Storage::new(material);
        let reference = storage.as_ref();
        self.materials.push(storage);
        reference
    }

    pub(crate) fn add_mesh(&mut self, mesh: Mesh) -> Ref<Mesh> {
        let storage = Storage::new(mesh);
        let reference = storage.as_ref();
        self.meshes.push(storage);
        reference
    }

    pub(crate) fn add_shader(&mut self, shader: Shader) -> Ref<Shader> {
        let storage = Storage::new(shader);
        let reference = storage.as_ref();
        self.shaders.push(storage);
        reference
    }

    pub(crate) fn add_font(&mut self, font: Font) -> Ref<Font> {
        let storage = Storage::new(font);
        let reference = storage.as_ref();
        self.fonts.push(storage);
        reference
    }

    pub(crate) fn add_framebuffer(&mut self, framebuffer: Framebuffer) -> Ref<Framebuffer> {
        let storage = Storage::new(framebuffer);
        let reference = storage.as_ref();
        self.framebuffers.push(storage);
        reference
    }

    pub(crate) fn clean_unused(&mut self, uniform: &mut ImageUniform) {
        self.fonts.retain(|r| r.count() != 0);
        self.meshes.retain(|r| r.count() != 0);
        self.materials.retain(|r| r.count() != 0);
        self.shaders.retain(|r| r.count() != 0);
        self.framebuffers.retain(|r| r.count() != 0);
        self.textures
            .drain_filter(|r| r.count() == 0)
            .for_each(|r| uniform.remove(r.with(|t| t.image_index())));
    }

    pub(crate) fn update_if_needed(&self) -> Result<()> {
        for mesh in &self.meshes {
            mesh.with(|m| m.update_if_needed())?;
        }
        for material in &self.materials {
            material.with(|m| m.update_if_needed())?;
        }
        Ok(())
    }
}
