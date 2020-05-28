// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ResourceManager - resource manager

mod builtin;

use log::error;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::mesh::Mesh;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;

pub(crate) use builtin::create_builtins;

pub(crate) struct ResourceManager {
    // storage
    textures: Storage<Texture>,
    materials: Storage<Material>,
    meshes: Storage<Mesh>,
    shaders: Storage<Shader>,
    fonts: Storage<Font>,
    framebuffers: Storage<Framebuffer>,
    // builtins
    builtin_textures: Builtins<Texture>,
    builtin_materials: Builtins<Material>,
    builtin_meshes: Builtins<Mesh>,
    builtin_shaders: Builtins<Shader>,
    builtin_fonts: Builtins<Font>,
    // id counter
    max_id: AtomicU32,
}

#[derive(Debug)]
pub struct Id<T>(Arc<u32>, PhantomData<*const T>);

pub(crate) type IdRef = u32;

type Storage<T> = Mutex<HashMap<Id<T>, T>>;
type Builtins<T> = Mutex<HashMap<String, Id<T>>>;

impl ResourceManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: Mutex::new(HashMap::new()),
            materials: Mutex::new(HashMap::new()),
            meshes: Mutex::new(HashMap::new()),
            shaders: Mutex::new(HashMap::new()),
            fonts: Mutex::new(HashMap::new()),
            framebuffers: Mutex::new(HashMap::new()),
            builtin_textures: Mutex::new(HashMap::new()),
            builtin_materials: Mutex::new(HashMap::new()),
            builtin_meshes: Mutex::new(HashMap::new()),
            builtin_shaders: Mutex::new(HashMap::new()),
            builtin_fonts: Mutex::new(HashMap::new()),
            max_id: AtomicU32::new(0),
        }
    }

    pub(crate) fn add_texture(&self, texture: Texture, builtin: Option<&str>) -> Id<Texture> {
        let id = Id(self.get_id(), PhantomData);
        self.textures.lock().unwrap().insert(id.clone(), texture);
        if let Some(name) = builtin {
            self.builtin_textures
                .lock()
                .unwrap()
                .insert(name.to_string(), id.clone());
        }
        id
    }

    pub(crate) fn add_material(&self, material: Material, builtin: Option<&str>) -> Id<Material> {
        let id = Id(self.get_id(), PhantomData);
        self.materials.lock().unwrap().insert(id.clone(), material);
        if let Some(name) = builtin {
            self.builtin_materials
                .lock()
                .unwrap()
                .insert(name.to_string(), id.clone());
        }
        id
    }

    pub(crate) fn add_mesh(&self, mesh: Mesh, builtin: Option<&str>) -> Id<Mesh> {
        let id = Id(self.get_id(), PhantomData);
        self.meshes.lock().unwrap().insert(id.clone(), mesh);
        if let Some(name) = builtin {
            self.builtin_meshes
                .lock()
                .unwrap()
                .insert(name.to_string(), id.clone());
        }
        id
    }

    pub(crate) fn add_shader(&self, shader: Shader, builtin: Option<&str>) -> Id<Shader> {
        let id = Id(self.get_id(), PhantomData);
        self.shaders.lock().unwrap().insert(id.clone(), shader);
        if let Some(name) = builtin {
            self.builtin_shaders
                .lock()
                .unwrap()
                .insert(name.to_string(), id.clone());
        }
        id
    }

    pub(crate) fn add_font(&self, font: Font, builtin: Option<&str>) -> Id<Font> {
        let id = Id(self.get_id(), PhantomData);
        self.fonts.lock().unwrap().insert(id.clone(), font);
        if let Some(name) = builtin {
            self.builtin_fonts
                .lock()
                .unwrap()
                .insert(name.to_string(), id.clone());
        }
        id
    }

    pub(crate) fn builtin(&self, name: impl AsRef<str>) -> IdRef {
        let name = name.as_ref();
        if let Some(id) = self.builtin_textures.lock().unwrap().get(name) {
            return id.id_ref();
        }
        if let Some(id) = self.builtin_materials.lock().unwrap().get(name) {
            return id.id_ref();
        }
        if let Some(id) = self.builtin_meshes.lock().unwrap().get(name) {
            return id.id_ref();
        }
        if let Some(id) = self.builtin_shaders.lock().unwrap().get(name) {
            return id.id_ref();
        }
        if let Some(id) = self.builtin_fonts.lock().unwrap().get(name) {
            return id.id_ref();
        }

        panic!(error!("builtin not found '{}'", name));
    }

    pub(crate) fn add_framebuffer(&self, framebuffer: Framebuffer) -> Id<Framebuffer> {
        let id = Id(self.get_id(), PhantomData);
        self.framebuffers
            .lock()
            .unwrap()
            .insert(id.clone(), framebuffer);
        id
    }

    pub(crate) fn with_texture<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&Texture) -> R,
    {
        let map = self.textures.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn with_material<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&mut Material) -> R,
    {
        let mut map = self.materials.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get_mut(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn with_mesh<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&mut Mesh) -> R,
    {
        let mut map = self.meshes.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get_mut(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn with_shader<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&Shader) -> R,
    {
        let map = self.shaders.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn with_font<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&Font) -> R,
    {
        let map = self.fonts.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn with_framebuffer<F, R>(&self, id: IdRef, fun: F) -> Option<R>
    where
        F: FnOnce(&Framebuffer) -> R,
    {
        let map = self.framebuffers.lock().unwrap();
        find_key(&map, id)
            .map(|k| map.get(&k).unwrap())
            .map(|v| fun(v))
    }

    pub(crate) fn replace_shader(&self, id: IdRef, shader: Shader) {
        let mut map = self.shaders.lock().unwrap();
        if let Some(key) = find_key(&map, id) {
            map.insert(key, shader);
        }
    }

    pub(crate) fn clean_unused(&self, uniform: &ImageUniform) {
        remove_unused(&mut self.framebuffers.lock().unwrap());
        remove_unused(&mut self.fonts.lock().unwrap());
        remove_unused(&mut self.meshes.lock().unwrap());
        remove_unused(&mut self.materials.lock().unwrap());
        remove_unused(&mut self.shaders.lock().unwrap());
        remove_unused(&mut self.textures.lock().unwrap())
            .iter()
            .for_each(|tex| uniform.remove(tex.image_index()));
    }

    fn get_id(&self) -> Arc<u32> {
        let id = self.max_id.load(Ordering::Relaxed);
        self.max_id.store(id + 1, Ordering::Relaxed);
        Arc::new(id)
    }
}

impl<T> Id<T> {
    pub(crate) fn id_ref(&self) -> IdRef {
        *self.0
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(*self.0);
        state.finish();
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        *self.0 == *other.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0.clone(), PhantomData)
    }
}

impl<T> Eq for Id<T> {}
unsafe impl<T> Send for Id<T> {}

fn find_key<T>(map: &MutexGuard<'_, HashMap<Id<T>, T>>, id: IdRef) -> Option<Id<T>> {
    for key in map.keys() {
        if *key.0 == id {
            return Some(key.clone());
        }
    }
    None
}

fn remove_unused<T>(map: &mut MutexGuard<'_, HashMap<Id<T>, T>>) -> Vec<T> {
    let unused = map
        .keys()
        .filter(|key| Arc::strong_count(&key.0) == 1)
        .cloned()
        .collect::<Vec<_>>();

    unused.iter().map(|key| map.remove(&key).unwrap()).collect()
}
