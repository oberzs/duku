// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ResourceManager - resource manager

mod builtin;
mod reference;

use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::mesh::Mesh;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;

pub(crate) use builtin::Builtins;
pub use reference::Ref;

pub(crate) struct ResourceManager {
    textures: Vec<Ref<Texture>>,
    materials: Vec<Ref<Material>>,
    meshes: Vec<Ref<Mesh>>,
    shaders: Vec<Ref<Shader>>,
    fonts: Vec<Ref<Font>>,
    framebuffers: Vec<Ref<Framebuffer>>,
}

impl ResourceManager {
    pub(crate) fn new() -> Self {
        profile_scope!("new");

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
        let reference = Ref::new(texture);
        self.textures.push(reference.clone());
        reference
    }

    pub(crate) fn add_material(&mut self, material: Material) -> Ref<Material> {
        let reference = Ref::new(material);
        self.materials.push(reference.clone());
        reference
    }

    pub(crate) fn add_mesh(&mut self, mesh: Mesh) -> Ref<Mesh> {
        let reference = Ref::new(mesh);
        self.meshes.push(reference.clone());
        reference
    }

    pub(crate) fn add_shader(&mut self, shader: Shader) -> Ref<Shader> {
        let reference = Ref::new(shader);
        self.shaders.push(reference.clone());
        reference
    }

    pub(crate) fn add_font(&mut self, font: Font) -> Ref<Font> {
        let reference = Ref::new(font);
        self.fonts.push(reference.clone());
        reference
    }

    pub(crate) fn add_framebuffer(&mut self, framebuffer: Framebuffer) -> Ref<Framebuffer> {
        let reference = Ref::new(framebuffer);
        self.framebuffers.push(reference.clone());
        reference
    }

    pub(crate) fn clean_unused(&mut self, uniform: &ImageUniform) {
        self.fonts.retain(|r| r.count() != 1);
        self.meshes.retain(|r| r.count() != 1);
        self.materials.retain(|r| r.count() != 1);
        self.shaders.retain(|r| r.count() != 1);
        self.framebuffers.retain(|r| r.count() != 1);
        self.textures
            .drain_filter(|r| r.count() == 1)
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
