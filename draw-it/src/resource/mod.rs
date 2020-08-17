// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ResourceManager - resource manager

mod builtin;
mod index;
mod storage;

pub(crate) mod hash;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::font::Font;
use crate::image::CoreFramebuffer;
use crate::image::FramebufferUpdateData;
use crate::image::Texture;
use crate::mesh::CoreMesh;
use crate::mesh::MeshUpdateData;
use crate::pipeline::CoreMaterial;
use crate::pipeline::ImageUniform;
use crate::pipeline::MaterialUpdateData;
use crate::pipeline::Shader;
use storage::Storage;

pub(crate) use builtin::Builtins;
pub(crate) use index::Index;
pub use storage::Ref;

pub(crate) struct ResourceManager {
    textures: Vec<Storage<Texture>>,
    shaders: Vec<Storage<Shader>>,
    fonts: Vec<Storage<Font>>,

    pub(crate) framebuffers: Resource<CoreFramebuffer, FramebufferUpdateData>,
    pub(crate) materials: Resource<CoreMaterial, MaterialUpdateData>,
    pub(crate) meshes: Resource<CoreMesh, MeshUpdateData>,
}

pub(crate) struct Resource<T, U> {
    stored: HashMap<Index, T>,
    sender: Sender<(Index, U)>,
    receiver: Receiver<(Index, U)>,
    next_index: u32,
}

impl ResourceManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: vec![],
            shaders: vec![],
            fonts: vec![],
            framebuffers: Resource::new(),
            materials: Resource::new(),
            meshes: Resource::new(),
        }
    }

    pub(crate) fn add_texture(&mut self, texture: Texture) -> Ref<Texture> {
        let storage = Storage::new(texture);
        let reference = storage.as_ref();
        self.textures.push(storage);
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

    pub(crate) fn clean_unused(&mut self, uniform: &mut ImageUniform) {
        self.fonts.retain(|r| r.count() != 0);
        // self.meshes.retain(|r| r.count() != 0);
        // self.materials.retain(|r| r.count() != 0);
        self.shaders.retain(|r| r.count() != 0);
        // self.framebuffers.retain(|r| r.count() != 0);
        self.textures
            .drain_filter(|r| r.count() == 0)
            .for_each(|r| uniform.remove(r.with(|t| t.image_index())));
    }

    pub(crate) fn update_if_needed(&mut self, image_uniform: &mut ImageUniform) {
        // update meshes
        for (i, data) in self.meshes.receiver.try_iter() {
            self.meshes
                .stored
                .get_mut(&i)
                .expect("bad index")
                .update(data);
        }

        // update materials
        for (i, data) in self.materials.receiver.try_iter() {
            self.materials
                .stored
                .get_mut(&i)
                .expect("bad index")
                .update(data);
        }

        // update framebuffers
        for (i, data) in self.framebuffers.receiver.try_iter() {
            self.framebuffers
                .stored
                .get_mut(&i)
                .expect("bad index")
                .update(image_uniform, data);
        }
    }
}

impl<T, U> Resource<T, U> {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        Self {
            stored: HashMap::new(),
            next_index: 0,
            sender,
            receiver,
        }
    }

    pub(crate) fn add(&mut self, value: T) -> (Index, Sender<(Index, U)>) {
        let index = Index::new(self.next_index);
        self.next_index += 1;
        self.stored.insert(index.clone(), value);
        (index, self.sender.clone())
    }

    pub(crate) fn get(&self, index: &Index) -> &T {
        self.stored.get(index).expect("bad index")
    }

    pub(crate) fn get_mut(&mut self, index: &Index) -> &mut T {
        self.stored.get_mut(index).expect("bad index")
    }
}
