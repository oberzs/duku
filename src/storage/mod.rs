// Oliver Berzs
// https://github.com/oberzs/draw-it

// Storage - Vulkan resource storage

mod builtin;
mod handle;

use std::collections::HashMap;

use crate::device::Device;
use crate::font::Font;
use crate::image::Cubemap;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderImages;

pub(crate) use builtin::create_cube;
pub(crate) use builtin::create_ico_sphere;
pub(crate) use builtin::create_uv_sphere;
pub(crate) use builtin::Builtins;

pub use handle::Handle;

pub(crate) struct Storage {
    pub(crate) shaders: Store<Shader>,
    pub(crate) fonts: Store<Font>,
    pub(crate) textures: Store<Texture>,
    pub(crate) cubemaps: Store<Cubemap>,
    pub(crate) framebuffers: Store<Framebuffer>,
    pub(crate) materials: Store<Material>,
    pub(crate) meshes: Store<Mesh>,
    next_id: u32,
}

pub(crate) struct Store<T> {
    stored: HashMap<Handle<T>, T>,
}

impl Storage {
    pub(crate) fn new() -> Self {
        Self {
            shaders: Store::new(),
            fonts: Store::new(),
            textures: Store::new(),
            cubemaps: Store::new(),
            framebuffers: Store::new(),
            materials: Store::new(),
            meshes: Store::new(),
            next_id: 0,
        }
    }

    pub(crate) fn add_shader(&mut self, shader: Shader) -> Handle<Shader> {
        let id = self.next_id;
        self.next_id += 1;
        self.shaders.add(shader, id)
    }

    pub(crate) fn add_font(&mut self, font: Font) -> Handle<Font> {
        let id = self.next_id;
        self.next_id += 1;
        self.fonts.add(font, id)
    }

    pub(crate) fn add_texture(&mut self, texture: Texture) -> Handle<Texture> {
        let id = texture.shader_index();
        self.textures.add(texture, id)
    }

    pub(crate) fn add_cubemap(&mut self, cubemap: Cubemap) -> Handle<Cubemap> {
        let id = cubemap.shader_index();
        self.cubemaps.add(cubemap, id)
    }

    pub(crate) fn add_framebuffer(&mut self, framebuffer: Framebuffer) -> Handle<Framebuffer> {
        let id = framebuffer.shader_index();
        self.framebuffers.add(framebuffer, id)
    }

    pub(crate) fn add_material(&mut self, material: Material) -> Handle<Material> {
        let id = self.next_id;
        self.next_id += 1;
        self.materials.add(material, id)
    }

    pub(crate) fn add_mesh(&mut self, mesh: Mesh) -> Handle<Mesh> {
        let id = self.next_id;
        self.next_id += 1;
        self.meshes.add(mesh, id)
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
        for unused in self.shaders.clear_unused() {
            unused.destroy(device);
        }
        for unused in self.framebuffers.clear_unused() {
            shader_images.remove_image(unused.shader_index());
            unused.destroy(device);
        }
        for unused in self.textures.clear_unused() {
            shader_images.remove_image(unused.shader_index());
            unused.destroy(device);
        }
        for unused in self.cubemaps.clear_unused() {
            shader_images.remove_cubemap(unused.shader_index());
            unused.destroy(device);
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
        for unused in self.shaders.clear() {
            unused.destroy(device);
        }
        for unused in self.framebuffers.clear() {
            shader_images.remove_image(unused.shader_index());
            unused.destroy(device);
        }
        for unused in self.textures.clear() {
            shader_images.remove_image(unused.shader_index());
            unused.destroy(device);
        }
        for unused in self.cubemaps.clear() {
            shader_images.remove_cubemap(unused.shader_index());
            unused.destroy(device);
        }
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device, shader_images: &mut ShaderImages) {
        // update meshes
        for value in self.meshes.stored.values_mut() {
            value.update_if_needed(device);
        }

        // update materials
        for value in self.materials.stored.values_mut() {
            value.update_if_needed(device);
        }

        // update framebuffers
        for value in self.framebuffers.stored.values_mut() {
            value.update_if_needed(device, shader_images);
        }
    }
}

impl<T> Store<T> {
    fn new() -> Self {
        Self {
            stored: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, handle: &Handle<T>) -> &T {
        self.stored.get(handle).expect("bad index")
    }

    pub(crate) fn get_mut(&mut self, handle: &Handle<T>) -> &mut T {
        self.stored.get_mut(handle).expect("bad index")
    }

    fn clear_unused(&mut self) -> impl Iterator<Item = T> {
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

    fn clear(&mut self) -> impl Iterator<Item = T> {
        self.stored
            .drain()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn add(&mut self, value: T, id: u32) -> Handle<T> {
        let handle = Handle::new(id);
        self.stored.insert(handle.clone(), value);
        handle
    }
}
