// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Storage - Vulkan resource storage

mod builtin;
mod index;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::font::CoreFont;
use crate::image::CoreFramebuffer;
use crate::image::CoreTexture;
use crate::image::Size;
use crate::mesh::CoreMesh;
use crate::mesh::MeshData;
use crate::pipeline::CoreMaterial;
use crate::pipeline::CoreShader;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderMaterial;

pub(crate) use builtin::Builtins;
pub(crate) use index::Index;

pub(crate) struct Storage {
    pub(crate) shaders: Store<CoreShader>,
    pub(crate) fonts: Store<CoreFont>,
    pub(crate) textures: Store<CoreTexture>,
    pub(crate) framebuffers: Store<CoreFramebuffer, Size>,
    pub(crate) materials: Store<CoreMaterial, ShaderMaterial>,
    pub(crate) meshes: Store<CoreMesh, MeshData>,
}

pub(crate) struct Store<T, U = ()> {
    stored: HashMap<Index, T>,
    sender: Sender<(Index, U)>,
    receiver: Receiver<(Index, U)>,
    next_index: u32,
}

impl Storage {
    pub(crate) fn new() -> Self {
        Self {
            shaders: Store::new(),
            fonts: Store::new(),
            textures: Store::new(),
            framebuffers: Store::new(),
            materials: Store::new(),
            meshes: Store::new(),
        }
    }

    pub(crate) fn clean_unused(&mut self, shader_images: &mut ShaderImages) {
        self.fonts.stored.retain(|i, _| i.count() > 1);
        self.meshes.stored.retain(|i, _| i.count() > 1);
        self.materials.stored.retain(|i, _| i.count() > 1);
        self.shaders.stored.retain(|i, _| i.count() > 1);
        self.framebuffers.stored.retain(|i, f| {
            if i.count() == 1 {
                shader_images.remove(f.shader_index());
            }
            i.count() > 1
        });
        self.textures.stored.retain(|i, t| {
            if i.count() == 1 {
                shader_images.remove(t.shader_index());
            }
            i.count() > 1
        });
    }

    pub(crate) fn update_if_needed(&mut self, shader_images: &mut ShaderImages) {
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
                .update(shader_images, data);
        }
    }
}

impl<T, U> Store<T, U> {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<(Index, U)>();

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
