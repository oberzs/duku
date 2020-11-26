// Oliver Berzs
// https://github.com/oberzs/duku

#![allow(clippy::vec_box)]

mod builtins;
mod resource;

use crate::device::Device;
use crate::font::Font;
use crate::image::Canvas;
use crate::image::Cubemap;
use crate::image::Texture;
use crate::mesh::Mesh;
use crate::mesh::Model;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::Uniforms;
use resource::Resource;

pub(crate) use builtins::create_cube;
pub(crate) use builtins::create_ico_sphere;
pub(crate) use builtins::create_uv_sphere;
pub(crate) use builtins::Builtins;
pub use resource::Handle;

#[derive(Default)]
pub(crate) struct Resources {
    shaders: Vec<Box<Resource<Shader>>>,
    fonts: Vec<Box<Resource<Font>>>,
    textures: Vec<Box<Resource<Texture>>>,
    cubemaps: Vec<Box<Resource<Cubemap>>>,
    canvases: Vec<Box<Resource<Canvas>>>,
    materials: Vec<Box<Resource<Material>>>,
    meshes: Vec<Box<Resource<Mesh>>>,
    models: Vec<Box<Resource<Model>>>,
}

impl Resources {
    pub(crate) fn add_shader(&mut self, value: Shader) -> Handle<Shader> {
        add(&mut self.shaders, value)
    }

    pub(crate) fn add_font(&mut self, value: Font) -> Handle<Font> {
        add(&mut self.fonts, value)
    }

    pub(crate) fn add_texture(&mut self, value: Texture) -> Handle<Texture> {
        add(&mut self.textures, value)
    }

    pub(crate) fn add_cubemap(&mut self, value: Cubemap) -> Handle<Cubemap> {
        add(&mut self.cubemaps, value)
    }

    pub(crate) fn add_canvas(&mut self, value: Canvas) -> Handle<Canvas> {
        add(&mut self.canvases, value)
    }

    pub(crate) fn add_material(&mut self, value: Material) -> Handle<Material> {
        add(&mut self.materials, value)
    }

    pub(crate) fn add_mesh(&mut self, value: Mesh) -> Handle<Mesh> {
        add(&mut self.meshes, value)
    }

    pub(crate) fn add_model(&mut self, value: Model) -> Handle<Model> {
        add(&mut self.models, value)
    }

    pub(crate) fn clear_unused(&mut self, device: &Device, uniforms: &mut Uniforms) {
        clear_unused(&mut self.models, |_| {});
        clear_unused(&mut self.fonts, |v| v.destroy(device, uniforms));
        clear_unused(&mut self.shaders, |v| v.destroy(device));
        clear_unused(&mut self.textures, |v| v.destroy(device, uniforms));
        clear_unused(&mut self.cubemaps, |v| v.destroy(device, uniforms));
        clear_unused(&mut self.canvases, |v| v.destroy(device, uniforms));
        clear_unused(&mut self.materials, |v| v.destroy(device));
        clear_unused(&mut self.meshes, |v| v.destroy(device));
    }

    pub(crate) fn clear(&mut self, device: &Device, uniforms: &mut Uniforms) {
        clear(&mut self.models, |_| {});
        clear(&mut self.fonts, |v| v.destroy(device, uniforms));
        clear(&mut self.shaders, |v| v.destroy(device));
        clear(&mut self.textures, |v| v.destroy(device, uniforms));
        clear(&mut self.cubemaps, |v| v.destroy(device, uniforms));
        clear(&mut self.canvases, |v| v.destroy(device, uniforms));
        clear(&mut self.materials, |v| v.destroy(device));
        clear(&mut self.meshes, |v| v.destroy(device));
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device, uniforms: &mut Uniforms) {
        // update meshes
        for r in &mut self.meshes {
            if r.mutated {
                r.value.update(device);
            }
            r.mutated = false;
        }

        // update materials
        for r in &mut self.materials {
            if r.mutated {
                r.value.update();
            }
            r.mutated = false;
        }

        // update canvases
        for r in &mut self.canvases {
            if r.mutated {
                r.value.update(device, uniforms);
            }
            r.mutated = false;
        }

        // update textures
        for r in &mut self.textures {
            if r.mutated {
                r.value.update(device);
            }
            r.mutated = false;
        }
    }
}

fn add<T>(resources: &mut Vec<Box<Resource<T>>>, value: T) -> Handle<T> {
    let mut resource = Box::new(Resource::new(value));
    let handle = resource.handle();
    resources.push(resource);
    handle
}

fn clear_unused<T>(resources: &mut Vec<Box<Resource<T>>>, mut clear_fn: impl FnMut(&T)) {
    resources.retain(|r| {
        if r.count == 0 {
            clear_fn(&r.value);
            false
        } else {
            true
        }
    });
}

fn clear<T>(resources: &mut Vec<Box<Resource<T>>>, mut clear_fn: impl FnMut(&T)) {
    for r in resources.iter() {
        clear_fn(&r.value);
    }
    resources.clear();
}
