// Oliver Berzs
// https://github.com/oberzs/duku

mod builtins;
mod handle;

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

pub(crate) use builtins::create_cube;
pub(crate) use builtins::create_ico_sphere;
pub(crate) use builtins::create_uv_sphere;
pub(crate) use builtins::Builtins;
pub use handle::Handle;
pub use handle::ReadGuard;
pub use handle::WriteGuard;

#[derive(Default)]
pub(crate) struct Resources {
    shaders: Vec<Handle<Shader>>,
    fonts: Vec<Handle<Font>>,
    textures: Vec<Handle<Texture>>,
    cubemaps: Vec<Handle<Cubemap>>,
    canvases: Vec<Handle<Canvas>>,
    materials: Vec<Handle<Material>>,
    meshes: Vec<Handle<Mesh>>,
    models: Vec<Handle<Model>>,
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
        for h in &mut self.meshes {
            if h.mutated() {
                h.get_mut().update(device);
            }
        }

        // update materials
        for h in &mut self.materials {
            if h.mutated() {
                h.get_mut().update();
            }
        }

        // update canvases
        for h in &mut self.canvases {
            if h.mutated() {
                h.get_mut().update(device, uniforms);
            }
        }

        // update textures
        for h in &mut self.textures {
            if h.mutated() {
                h.get_mut().update(device);
            }
        }
    }
}

fn add<T>(handles: &mut Vec<Handle<T>>, value: T) -> Handle<T> {
    let handle = Handle::new(value);
    handles.push(handle.clone());
    handle
}

fn clear_unused<T>(handles: &mut Vec<Handle<T>>, mut clear_fn: impl FnMut(&T)) {
    handles.retain(|h| {
        if h.count() == 0 {
            clear_fn(&h.read());
            false
        } else {
            true
        }
    });
}

fn clear<T>(handles: &mut Vec<Handle<T>>, mut clear_fn: impl FnMut(&T)) {
    for h in handles.iter() {
        clear_fn(&h.read());
    }
    handles.clear();
}
