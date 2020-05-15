mod builtin_fonts;
mod builtin_materials;
mod builtin_meshes;
mod builtin_shaders;
mod builtin_textures;
mod builtins;

use std::cell::Cell;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use crate::images::Font;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::shaders::Material;
use crate::shaders::Shader;
use builtin_fonts::BuiltinFonts;
use builtin_materials::BuiltinMaterials;
use builtin_meshes::BuiltinMeshes;
use builtin_shaders::BuiltinShaders;
use builtin_textures::BuiltinTextures;
pub(crate) use builtins::Builtins;

pub(crate) struct Objects {
    textures: RefCell<HashMap<Id<Texture>, Texture>>,
    max_texture_id: Cell<u32>,
    materials: RefCell<HashMap<Id<Material>, Material>>,
    max_material_id: Cell<u32>,
    meshes: RefCell<HashMap<Id<Mesh>, Mesh>>,
    max_mesh_id: Cell<u32>,
    shaders: RefCell<HashMap<Id<Shader>, Shader>>,
    max_shader_id: Cell<u32>,
    fonts: RefCell<HashMap<Id<Font>, Font>>,
    max_font_id: Cell<u32>,
}

#[derive(Debug)]
pub struct Id<T>(u32, PhantomData<*const T>);

impl Objects {
    pub(crate) fn new() -> Self {
        Self {
            textures: RefCell::new(HashMap::new()),
            max_texture_id: Cell::new(0),
            materials: RefCell::new(HashMap::new()),
            max_material_id: Cell::new(0),
            meshes: RefCell::new(HashMap::new()),
            max_mesh_id: Cell::new(0),
            shaders: RefCell::new(HashMap::new()),
            max_shader_id: Cell::new(0),
            fonts: RefCell::new(HashMap::new()),
            max_font_id: Cell::new(0),
        }
    }

    pub(crate) fn add_texture(&self, texture: Texture) -> Id<Texture> {
        let max_id = self.max_texture_id.get();
        let id = Id(max_id, PhantomData);

        self.textures.borrow_mut().insert(id, texture);
        self.max_texture_id.set(max_id + 1);

        id
    }

    pub(crate) fn texture(&self, id: Id<Texture>) -> Option<RefMut<'_, Texture>> {
        ref_mut_filter_map(self.textures.borrow_mut(), |ts| ts.get_mut(&id))
    }

    pub(crate) fn add_material(&self, material: Material) -> Id<Material> {
        let max_id = self.max_material_id.get();
        let id = Id(max_id, PhantomData);

        self.materials.borrow_mut().insert(id, material);
        self.max_material_id.set(max_id + 1);

        id
    }

    pub(crate) fn material(&self, id: Id<Material>) -> Option<RefMut<'_, Material>> {
        ref_mut_filter_map(self.materials.borrow_mut(), |ms| ms.get_mut(&id))
    }

    pub(crate) fn add_mesh(&self, mesh: Mesh) -> Id<Mesh> {
        let max_id = self.max_mesh_id.get();
        let id = Id(max_id, PhantomData);

        self.meshes.borrow_mut().insert(id, mesh);
        self.max_mesh_id.set(max_id + 1);

        id
    }

    pub(crate) fn mesh(&self, id: Id<Mesh>) -> Option<RefMut<'_, Mesh>> {
        ref_mut_filter_map(self.meshes.borrow_mut(), |ms| ms.get_mut(&id))
    }

    pub(crate) fn add_shader(&self, shader: Shader) -> Id<Shader> {
        let max_id = self.max_shader_id.get();
        let id = Id(max_id, PhantomData);

        self.shaders.borrow_mut().insert(id, shader);
        self.max_shader_id.set(max_id + 1);

        id
    }

    // pub(crate) fn replace_shader(&self, id: Id<Shader>, shader: Shader) {
    //     self.shaders.borrow_mut().insert(id, shader);
    // }

    pub(crate) fn shader(&self, id: Id<Shader>) -> Option<RefMut<'_, Shader>> {
        ref_mut_filter_map(self.shaders.borrow_mut(), |ss| ss.get_mut(&id))
    }

    pub(crate) fn add_font(&self, font: Font) -> Id<Font> {
        let max_id = self.max_font_id.get();
        let id = Id(max_id, PhantomData);

        self.fonts.borrow_mut().insert(id, font);
        self.max_font_id.set(max_id + 1);

        id
    }

    pub(crate) fn font(&self, id: Id<Font>) -> Option<RefMut<'_, Font>> {
        ref_mut_filter_map(self.fonts.borrow_mut(), |ms| ms.get_mut(&id))
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

// temporary
fn ref_mut_filter_map<T: ?Sized, U: ?Sized, F: FnOnce(&mut T) -> Option<&mut U>>(
    mut orig: RefMut<'_, T>,
    f: F,
) -> Option<RefMut<'_, U>> {
    f(&mut orig)
        .map(|new| new as *mut U)
        .map(|raw| RefMut::map(orig, |_| unsafe { &mut *raw }))
}
