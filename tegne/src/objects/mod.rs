mod builtin_font;
mod builtin_material;
mod builtin_mesh;
mod builtin_shader;
mod builtin_texture;

use log::debug;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::error::Result;
use crate::images::Font;
use crate::images::Texture;
use crate::instance::Device;
use crate::mesh::Mesh;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::RenderPasses;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use builtin_font::builtin_fonts;
pub(crate) use builtin_font::BuiltinFont;
use builtin_material::builtin_materials;
pub(crate) use builtin_material::BuiltinMaterial;
use builtin_mesh::builtin_meshes;
pub(crate) use builtin_mesh::BuiltinMesh;
use builtin_shader::builtin_shaders;
pub(crate) use builtin_shader::BuiltinShader;
use builtin_texture::builtin_textures;
pub(crate) use builtin_texture::BuiltinTexture;

pub(crate) struct Objects {
    builtins: Builtins,
    textures: RefCell<HashMap<Id<Texture>, Texture>>,
    max_texture_id: Cell<u32>,
    materials: RefCell<HashMap<Id<Material>, Material>>,
    max_material_id: Cell<u32>,
    meshes: RefCell<HashMap<Id<Mesh>, Mesh>>,
    max_mesh_id: Cell<u32>,
    shaders: RefCell<HashMap<Id<Shader>, Shader>>,
    max_shader_id: Cell<u32>,
}

pub(crate) struct Builtins {
    textures: HashMap<BuiltinTexture, Texture>,
    shaders: HashMap<BuiltinShader, Shader>,
    materials: HashMap<BuiltinMaterial, Material>,
    meshes: HashMap<BuiltinMesh, Mesh>,
    fonts: HashMap<BuiltinFont, Font>,
}

#[derive(Debug)]
pub struct Id<T>(u32, PhantomData<*const T>);

impl Objects {
    pub(crate) fn new(
        device: &Arc<Device>,
        passes: &RenderPasses,
        layout: &ShaderLayout,
        uniforms: &ImageUniforms,
    ) -> Result<Self> {
        let builtins = Builtins::new(device, passes, layout, uniforms)?;

        Ok(Self {
            builtins,
            textures: RefCell::new(HashMap::new()),
            max_texture_id: Cell::new(0),
            materials: RefCell::new(HashMap::new()),
            max_material_id: Cell::new(0),
            meshes: RefCell::new(HashMap::new()),
            max_mesh_id: Cell::new(0),
            shaders: RefCell::new(HashMap::new()),
            max_shader_id: Cell::new(0),
        })
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

    pub(crate) fn builtins(&self) -> &Builtins {
        &self.builtins
    }
}

impl Builtins {
    fn new(
        device: &Arc<Device>,
        passes: &RenderPasses,
        layout: &ShaderLayout,
        uniforms: &ImageUniforms,
    ) -> Result<Self> {
        debug!("creating builtin meshes");
        let meshes = builtin_meshes(device)?;

        debug!("creating builtin shaders");
        let shaders = builtin_shaders(device, passes, layout)?;

        debug!("creating builtin textures");
        let textures = builtin_textures(device, uniforms)?;

        debug!("creating builtin materials");
        let materials = builtin_materials(device, layout)?;

        debug!("creating builtin fonts");
        let fonts = builtin_fonts(device, uniforms)?;

        Ok(Self {
            meshes,
            shaders,
            textures,
            materials,
            fonts,
        })
    }

    pub(crate) fn mesh(&self, mesh: BuiltinMesh) -> Option<&Mesh> {
        self.meshes.get(&mesh)
    }

    pub(crate) fn material(&self, material: BuiltinMaterial) -> Option<&Material> {
        self.materials.get(&material)
    }

    pub(crate) fn shader(&self, shader: BuiltinShader) -> Option<&Shader> {
        self.shaders.get(&shader)
    }

    pub(crate) fn texture(&self, texture: BuiltinTexture) -> Option<&Texture> {
        self.textures.get(&texture)
    }

    pub(crate) fn font(&self, font: BuiltinFont) -> Option<&Font> {
        self.fonts.get(&font)
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
