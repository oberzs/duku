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
use std::rc::Rc;

use crate::images::Font;
use crate::images::Texture;
use crate::instance::Device;
use crate::instance::RenderPassType;
use crate::mesh::Mesh;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;
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
        device: &Rc<Device>,
        passes: &HashMap<RenderPassType, RenderPass>,
        layout: &ShaderLayout,
        uniforms: &ImageUniforms,
    ) -> Self {
        let builtins = Builtins::new(device, passes, layout, uniforms);

        Self {
            builtins,
            textures: RefCell::new(HashMap::new()),
            max_texture_id: Cell::new(0),
            materials: RefCell::new(HashMap::new()),
            max_material_id: Cell::new(0),
            meshes: RefCell::new(HashMap::new()),
            max_mesh_id: Cell::new(0),
            shaders: RefCell::new(HashMap::new()),
            max_shader_id: Cell::new(0),
        }
    }

    pub(crate) fn add_texture(&self, texture: Texture) -> Id<Texture> {
        let max_id = self.max_texture_id.get();
        let id = Id(max_id, PhantomData);

        self.textures.borrow_mut().insert(id, texture);
        self.max_texture_id.set(max_id + 1);

        id
    }

    pub(crate) fn texture(&self, id: Id<Texture>) -> RefMut<'_, Texture> {
        RefMut::map(self.textures.borrow_mut(), |ts| {
            ts.get_mut(&id).expect("texture does not exist")
        })
    }

    pub(crate) fn add_material(&self, material: Material) -> Id<Material> {
        let max_id = self.max_material_id.get();
        let id = Id(max_id, PhantomData);

        self.materials.borrow_mut().insert(id, material);
        self.max_material_id.set(max_id + 1);

        id
    }

    pub(crate) fn material(&self, id: Id<Material>) -> RefMut<'_, Material> {
        RefMut::map(self.materials.borrow_mut(), |ts| {
            ts.get_mut(&id).expect("material does not exist")
        })
    }

    pub(crate) fn add_mesh(&self, mesh: Mesh) -> Id<Mesh> {
        let max_id = self.max_mesh_id.get();
        let id = Id(max_id, PhantomData);

        self.meshes.borrow_mut().insert(id, mesh);
        self.max_mesh_id.set(max_id + 1);

        id
    }

    pub(crate) fn mesh(&self, id: Id<Mesh>) -> RefMut<'_, Mesh> {
        RefMut::map(self.meshes.borrow_mut(), |ts| {
            ts.get_mut(&id).expect("mesh does not exist")
        })
    }

    pub(crate) fn add_shader(&self, shader: Shader) -> Id<Shader> {
        let max_id = self.max_shader_id.get();
        let id = Id(max_id, PhantomData);

        self.shaders.borrow_mut().insert(id, shader);
        self.max_shader_id.set(max_id + 1);

        id
    }

    pub(crate) fn replace_shader(&self, id: Id<Shader>, shader: Shader) {
        self.shaders.borrow_mut().insert(id, shader);
    }

    pub(crate) fn shader(&self, id: Id<Shader>) -> RefMut<'_, Shader> {
        RefMut::map(self.shaders.borrow_mut(), |ts| {
            ts.get_mut(&id).expect("shader does not exist")
        })
    }

    pub(crate) fn builtins(&self) -> &Builtins {
        &self.builtins
    }
}

impl Builtins {
    fn new(
        device: &Rc<Device>,
        passes: &HashMap<RenderPassType, RenderPass>,
        layout: &ShaderLayout,
        uniforms: &ImageUniforms,
    ) -> Self {
        debug!("creating builtin meshes");
        let meshes = builtin_meshes(device);

        debug!("creating builtin shaders");
        let shaders = builtin_shaders(device, passes, layout);

        debug!("creating builtin textures");
        let textures = builtin_textures(device, uniforms);

        debug!("creating builtin materials");
        let materials = builtin_materials(device, layout);

        debug!("creating builtin fonts");
        let fonts = builtin_fonts(device, uniforms);

        Self {
            meshes,
            shaders,
            textures,
            materials,
            fonts,
        }
    }

    pub(crate) fn mesh(&self, mesh: BuiltinMesh) -> &Mesh {
        self.meshes.get(&mesh).or_error("mesh builtins not setup")
    }

    pub(crate) fn material(&self, material: BuiltinMaterial) -> &Material {
        self.materials
            .get(&material)
            .or_error("material builtins not setup")
    }

    pub(crate) fn shader(&self, shader: BuiltinShader) -> &Shader {
        self.shaders
            .get(&shader)
            .or_error("shader builtins not setup")
    }

    pub(crate) fn texture(&self, texture: BuiltinTexture) -> &Texture {
        self.textures
            .get(&texture)
            .or_error("texture builtins not setup")
    }

    pub(crate) fn font(&self, font: BuiltinFont) -> &Font {
        self.fonts.get(&font).or_error("font builtins not setup")
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
