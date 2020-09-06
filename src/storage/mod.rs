// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Storage - Vulkan resource storage

mod builtin;
mod index;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::device::Device;
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

    pub(crate) fn clear_unused(&mut self, device: &Device, shader_images: &mut ShaderImages) {
        for unused in self.fonts.clear_unused() {
            unused.destroy(device);
        }
        for unused in self.meshes.clear_unused() {
            unused.destroy(device);
        }
        for unused in self.materials.clear_unused() {
            unused.destroy(device);
        }
        for _unused in self.shaders.clear_unused() {
            // unused.destroy(device);
        }
        for unused in self.framebuffers.clear_unused() {
            shader_images.remove(unused.shader_index());
        }
        for unused in self.textures.clear_unused() {
            shader_images.remove(unused.shader_index());
        }
    }

    pub(crate) fn clear(&mut self, device: &Device, shader_images: &mut ShaderImages) {
        for unused in self.fonts.clear() {
            unused.destroy(device);
        }
        for unused in self.meshes.clear() {
            unused.destroy(device);
        }
        for unused in self.materials.clear() {
            unused.destroy(device);
        }
        for _unused in self.shaders.clear() {
            // unused.destroy(device);
        }
        for unused in self.framebuffers.clear() {
            shader_images.remove(unused.shader_index());
        }
        for unused in self.textures.clear() {
            shader_images.remove(unused.shader_index());
        }
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device, shader_images: &mut ShaderImages) {
        // update meshes
        for (i, data) in self.meshes.receiver.try_iter() {
            self.meshes
                .stored
                .get_mut(&i)
                .expect("bad index")
                .update(device, data);
        }

        // update materials
        for (i, data) in self.materials.receiver.try_iter() {
            self.materials
                .stored
                .get_mut(&i)
                .expect("bad index")
                .update(device, data);
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

    pub(crate) fn clear_unused(&mut self) -> impl Iterator<Item = T> {
        let mut removed = vec![];
        let stored: Vec<_> = self.stored.drain().collect();
        for (k, v) in stored {
            if k.count() == 1 {
                removed.push(v);
            } else {
                self.stored.insert(k, v);
            }
        }
        removed.into_iter()
    }

    pub(crate) fn clear(&mut self) -> impl Iterator<Item = T> {
        self.stored
            .drain()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
