// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ResourceManager - resource manager

mod builtin;
mod reference;

use std::sync::Mutex;
use std::sync::MutexGuard;

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
    textures: Storage<Texture>,
    materials: Storage<Material>,
    meshes: Storage<Mesh>,
    shaders: Storage<Shader>,
    fonts: Storage<Font>,
    framebuffers: Storage<Framebuffer>,
}

type Storage<T> = Mutex<Vec<Ref<T>>>;

impl ResourceManager {
    pub(crate) fn new() -> Self {
        profile_scope!("new");

        Self {
            textures: Mutex::new(vec![]),
            materials: Mutex::new(vec![]),
            meshes: Mutex::new(vec![]),
            shaders: Mutex::new(vec![]),
            fonts: Mutex::new(vec![]),
            framebuffers: Mutex::new(vec![]),
        }
    }

    pub(crate) fn add_texture(&self, texture: Texture) -> Ref<Texture> {
        let reference = Ref::new(texture);
        self.textures.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn add_material(&self, material: Material) -> Ref<Material> {
        let reference = Ref::new(material);
        self.materials.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn add_mesh(&self, mesh: Mesh) -> Ref<Mesh> {
        let reference = Ref::new(mesh);
        self.meshes.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn add_shader(&self, shader: Shader) -> Ref<Shader> {
        let reference = Ref::new(shader);
        self.shaders.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn add_font(&self, font: Font) -> Ref<Font> {
        let reference = Ref::new(font);
        self.fonts.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn add_framebuffer(&self, framebuffer: Framebuffer) -> Ref<Framebuffer> {
        let reference = Ref::new(framebuffer);
        self.framebuffers.lock().unwrap().push(reference.clone());
        reference
    }

    pub(crate) fn clean_unused(&self, uniform: &ImageUniform) {
        remove_unused(&mut self.fonts.lock().unwrap());
        remove_unused(&mut self.meshes.lock().unwrap());
        remove_unused(&mut self.materials.lock().unwrap());
        remove_unused(&mut self.shaders.lock().unwrap());
        remove_unused(&mut self.textures.lock().unwrap())
            .iter()
            .for_each(|tex| uniform.remove(tex.with(|t| t.image_index())));
        remove_unused(&mut self.framebuffers.lock().unwrap());
    }
}

fn remove_unused<T>(storage: &mut MutexGuard<'_, Vec<Ref<T>>>) -> Vec<Ref<T>> {
    storage.drain_filter(|r| r.count() == 1).collect()
}
