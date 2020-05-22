mod builtin_fonts;
mod builtin_materials;
mod builtin_meshes;
mod builtin_shaders;
mod builtin_textures;
mod builtins;

use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use crate::images::Font;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::instance::IN_FLIGHT_FRAME_COUNT;
use crate::mesh::Mesh;
use crate::shaders::Material;
use crate::shaders::Shader;
use builtin_fonts::BuiltinFonts;
use builtin_materials::BuiltinMaterials;
use builtin_meshes::BuiltinMeshes;
use builtin_shaders::BuiltinShaders;
use builtin_textures::BuiltinTextures;
pub(crate) use builtins::Builtins;

type Storage<T> = Mutex<HashMap<Id<T>, T>>;

pub(crate) struct Objects {
    textures: Storage<Texture>,
    materials: Storage<Material>,
    meshes: Storage<Mesh>,
    shaders: Storage<Shader>,
    fonts: Storage<Font>,
    framebuffers: Storage<Framebuffer>,
    unused_shaders: Mutex<Vec<Vec<Shader>>>,
    max_id: AtomicU32,
}

#[derive(Debug)]
pub struct Id<T>(u32, PhantomData<*const T>);

impl Objects {
    pub(crate) fn new() -> Self {
        let unused_shaders = (0..IN_FLIGHT_FRAME_COUNT)
            .map(|_| vec![])
            .collect::<Vec<_>>();

        Self {
            textures: Mutex::new(HashMap::new()),
            materials: Mutex::new(HashMap::new()),
            meshes: Mutex::new(HashMap::new()),
            shaders: Mutex::new(HashMap::new()),
            fonts: Mutex::new(HashMap::new()),
            framebuffers: Mutex::new(HashMap::new()),
            unused_shaders: Mutex::new(unused_shaders),
            max_id: AtomicU32::new(0),
        }
    }

    pub(crate) fn add_texture(&self, texture: Texture) -> Id<Texture> {
        let id = Id(self.get_id(), PhantomData);
        self.textures.lock().unwrap().insert(id, texture);
        id
    }

    pub(crate) fn add_material(&self, material: Material) -> Id<Material> {
        let id = Id(self.get_id(), PhantomData);
        self.materials.lock().unwrap().insert(id, material);
        id
    }

    pub(crate) fn add_mesh(&self, mesh: Mesh) -> Id<Mesh> {
        let id = Id(self.get_id(), PhantomData);
        self.meshes.lock().unwrap().insert(id, mesh);
        id
    }

    pub(crate) fn add_shader(&self, shader: Shader) -> Id<Shader> {
        let id = Id(self.get_id(), PhantomData);
        self.shaders.lock().unwrap().insert(id, shader);
        id
    }

    pub(crate) fn add_font(&self, font: Font) -> Id<Font> {
        let id = Id(self.get_id(), PhantomData);
        self.fonts.lock().unwrap().insert(id, font);
        id
    }

    pub(crate) fn add_framebuffer(&self, framebuffer: Framebuffer) -> Id<Framebuffer> {
        let id = Id(self.get_id(), PhantomData);
        self.framebuffers.lock().unwrap().insert(id, framebuffer);
        id
    }

    pub(crate) fn with_texture<F, R>(&self, id: Id<Texture>, fun: F) -> Option<R>
    where
        F: FnOnce(&Texture) -> R,
    {
        match self.textures.lock().unwrap().get(&id) {
            Some(texture) => Some(fun(texture)),
            None => None,
        }
    }

    pub(crate) fn with_material<F, R>(&self, id: Id<Material>, fun: F) -> Option<R>
    where
        F: FnOnce(&mut Material) -> R,
    {
        match self.materials.lock().unwrap().get_mut(&id) {
            Some(material) => Some(fun(material)),
            None => None,
        }
    }

    pub(crate) fn with_mesh<F, R>(&self, id: Id<Mesh>, fun: F) -> Option<R>
    where
        F: FnOnce(&Mesh) -> R,
    {
        match self.meshes.lock().unwrap().get(&id) {
            Some(mesh) => Some(fun(mesh)),
            None => None,
        }
    }

    pub(crate) fn with_shader<F, R>(&self, id: Id<Shader>, fun: F) -> Option<R>
    where
        F: FnOnce(&Shader) -> R,
    {
        match self.shaders.lock().unwrap().get(&id) {
            Some(shader) => Some(fun(shader)),
            None => None,
        }
    }

    pub(crate) fn with_font<F, R>(&self, id: Id<Font>, fun: F) -> Option<R>
    where
        F: FnOnce(&Font) -> R,
    {
        match self.fonts.lock().unwrap().get(&id) {
            Some(font) => Some(fun(font)),
            None => None,
        }
    }

    pub(crate) fn with_framebuffer<F, R>(&self, id: Id<Framebuffer>, fun: F) -> Option<R>
    where
        F: FnOnce(&Framebuffer) -> R,
    {
        match self.framebuffers.lock().unwrap().get(&id) {
            Some(framebuffer) => Some(fun(framebuffer)),
            None => None,
        }
    }

    pub(crate) fn replace_shader(&self, id: Id<Shader>, shader: Shader, frame: usize) {
        if let Some(replaced) = self.shaders.lock().unwrap().insert(id, shader) {
            self.unused_shaders.lock().unwrap()[frame].push(replaced);
        }
    }

    pub(crate) fn clean_unused(&self, frame: usize) {
        self.unused_shaders.lock().unwrap()[frame].clear();
    }

    fn get_id(&self) -> u32 {
        let id = self.max_id.load(Ordering::Relaxed);
        self.max_id.store(id + 1, Ordering::Relaxed);
        id
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
        state.finish();
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0, PhantomData)
    }
}

impl<T> Eq for Id<T> {}
impl<T> Copy for Id<T> {}
unsafe impl<T> Send for Id<T> {}
