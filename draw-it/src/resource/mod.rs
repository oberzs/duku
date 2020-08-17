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
use crate::pipeline::CoreMaterial;
use crate::pipeline::ImageUniform;
use crate::pipeline::Shader;
use storage::Storage;

pub(crate) use builtin::Builtins;
pub(crate) use index::Index;
pub(crate) use index::NewIndex;
pub use storage::Ref;

pub(crate) struct ResourceManager {
    textures: Vec<Storage<Texture>>,
    shaders: Vec<Storage<Shader>>,
    fonts: Vec<Storage<Font>>,

    pub(crate) framebuffers: Resource<CoreFramebuffer, FramebufferUpdateData>,
    materials: HashMap<Index, CoreMaterial>,
    meshes: HashMap<Index, CoreMesh>,
    next_index: u32,
}

pub(crate) struct Resource<T, U> {
    stored: HashMap<NewIndex, T>,
    sender: Sender<(NewIndex, U)>,
    receiver: Receiver<(NewIndex, U)>,
    next_index: u32,
}

impl ResourceManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: vec![],
            shaders: vec![],
            fonts: vec![],
            framebuffers: Resource::new(),
            materials: HashMap::new(),
            meshes: HashMap::new(),
            next_index: 0,
        }
    }

    pub(crate) fn add_texture(&mut self, texture: Texture) -> Ref<Texture> {
        let storage = Storage::new(texture);
        let reference = storage.as_ref();
        self.textures.push(storage);
        reference
    }

    pub(crate) fn add_material(&mut self, material: CoreMaterial) -> Index {
        let index = Index::new(self.next_index);
        self.next_index += 1;
        self.materials.insert(index.clone(), material);
        index
    }

    pub(crate) fn add_mesh(&mut self, mesh: CoreMesh) -> Index {
        let index = Index::new(self.next_index);
        self.next_index += 1;
        self.meshes.insert(index.clone(), mesh);
        index
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

    pub(crate) fn material(&self, index: &Index) -> &CoreMaterial {
        self.materials.get(index).expect("bad index")
    }

    pub(crate) fn material_mut(&mut self, index: &Index) -> &mut CoreMaterial {
        self.materials.get_mut(index).expect("bad index")
    }

    pub(crate) fn mesh(&self, index: &Index) -> &CoreMesh {
        self.meshes.get(index).expect("bad index")
    }

    pub(crate) fn mesh_mut(&mut self, index: &Index) -> &mut CoreMesh {
        self.meshes.get_mut(index).expect("bad index")
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

    pub(crate) fn add(&mut self, value: T) -> (NewIndex, Sender<(NewIndex, U)>) {
        let index = NewIndex::new(self.next_index);
        self.next_index += 1;
        self.stored.insert(index.clone(), value);
        (index, self.sender.clone())
    }

    pub(crate) fn get(&self, index: &NewIndex) -> &T {
        self.stored.get(index).expect("bad index")
    }

    pub(crate) fn get_mut(&mut self, index: &NewIndex) -> &mut T {
        self.stored.get_mut(index).expect("bad index")
    }
}
