mod material;
mod mesh;
mod shader;
mod texture;

use log::debug;
use std::collections::HashMap;
use std::rc::Rc;

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
use material::builtin_materials;
pub(crate) use material::BuiltinMaterial;
use mesh::builtin_meshes;
pub(crate) use mesh::BuiltinMesh;
use shader::builtin_shaders;
pub(crate) use shader::BuiltinShader;
use texture::builtin_textures;
pub(crate) use texture::BuiltinTexture;

pub(crate) struct Builtins {
    meshes: HashMap<BuiltinMesh, Mesh>,
    shaders: HashMap<BuiltinShader, Shader>,
    textures: HashMap<BuiltinTexture, Texture>,
    materials: HashMap<BuiltinMaterial, Material>,
}

impl Builtins {
    pub(crate) fn new(
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
        let materials = builtin_materials(device, layout, &shaders, &textures);

        Self {
            meshes,
            shaders,
            textures,
            materials,
        }
    }

    pub(crate) fn get_mesh(&self, mesh: BuiltinMesh) -> &Mesh {
        self.meshes.get(&mesh).or_error("mesh builtins not setup")
    }

    pub(crate) fn get_material(&self, material: BuiltinMaterial) -> &Material {
        self.materials
            .get(&material)
            .or_error("material builtins not setup")
    }

    pub(crate) fn get_shader(&self, shader: BuiltinShader) -> &Shader {
        self.shaders
            .get(&shader)
            .or_error("shader builtins not setup")
    }

    pub(crate) fn get_texture(&self, texture: BuiltinTexture) -> &Texture {
        self.textures
            .get(&texture)
            .or_error("texture builtins not setup")
    }
}
