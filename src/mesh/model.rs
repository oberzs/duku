// Oliver Berzs
// https://github.com/oberzs/duku

// Model - loaded glTF file structure

#![cfg(feature = "gltf")]

use gltf::buffer;
use gltf::image;
use gltf::mesh::Mode;
use gltf::Gltf;
use gltf::Node;
use std::collections::HashMap;

use super::Mesh;
use crate::device::Device;
use crate::error::Error;
use crate::error::Result;
use crate::image::ColorSpace;
use crate::image::Mips;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::pipeline::Material;
use crate::pipeline::Uniforms;
use crate::storage::Handle;
use crate::storage::Storage;

pub struct Model {
    draw_nodes: Vec<ModelNode>,
    _textures: HashMap<usize, Handle<Texture>>,
}

pub(crate) struct ModelNode {
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<Material>>,
    matrix: Matrix4,
    children: Vec<Self>,
}

impl Model {
    pub(crate) fn from_gltf_bytes(
        device: &Device,
        uniforms: &mut Uniforms,
        storage: &mut Storage,
        bytes: &[u8],
    ) -> Result<Self> {
        let gltf = Gltf::from_slice(bytes).map_err(|_| Error::InvalidGltf)?;

        // load buffers
        let mut buffers = vec![];
        for buffer in gltf.buffers() {
            match buffer.source() {
                // external data
                buffer::Source::Uri(uri) => {
                    println!("uri: '{}'", uri);
                    unimplemented!();
                }

                // internal data
                buffer::Source::Bin => {
                    let blob = gltf.blob.as_deref().ok_or(Error::InvalidGltf)?;
                    buffers.push(blob.to_vec());
                }
            }
        }

        // load meshes
        let mut meshes = HashMap::new();
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                if primitive.mode() != Mode::Triangles {
                    return Err(Error::UnsupportedPrimitive);
                }

                let mut vertices = vec![];
                let mut normals = vec![];
                let mut uvs = vec![];
                let mut indices = vec![];

                if let Some(ps) = reader.read_positions() {
                    vertices.extend(ps.map(|p| Vector3::new(p[0], p[1], -p[2])));
                }

                if let Some(ns) = reader.read_normals() {
                    normals.extend(ns.map(|n| Vector3::new(n[0], n[1], -n[2])));
                }

                if let Some(ts) = reader.read_tex_coords(0) {
                    uvs.extend(ts.into_f32().map(|t| Vector2::new(t[0], t[1])));
                }

                if let Some(is) = reader.read_indices() {
                    let ccw: Vec<_> = is.into_u32().collect();
                    for chunk in ccw.chunks(3) {
                        indices.extend(&[chunk[0], chunk[2], chunk[1]]);
                    }
                }

                let mut m = Mesh::new(device);
                m.set_vertices(vertices);
                m.set_uvs(uvs);
                m.set_indices(indices);

                if normals.is_empty() {
                    m.calculate_normals();
                } else {
                    m.set_normals(normals);
                    m.calculate_tangents();
                }

                m.update_if_needed(device);

                meshes.insert((mesh.index(), primitive.index()), storage.add_mesh(m));
            }
        }

        // load textures
        let mut texture_data = HashMap::new();
        for texture in gltf.textures() {
            if let image::Source::View { view, mime_type } = texture.source().source() {
                let start = view.offset() as usize;
                let end = start + view.length() as usize;
                let buffer = &buffers[view.buffer().index()][start..end];

                texture_data.insert(texture.index(), (mime_type, buffer));
            }
        }

        // load materials
        let mut textures = HashMap::new();
        let mut materials = HashMap::new();
        for material in gltf.materials() {
            let pbr = material.pbr_metallic_roughness();

            // factors
            let albedo = pbr.base_color_factor();
            let metalness = pbr.metallic_factor();
            let roughness = pbr.roughness_factor();
            let emissive = material.emissive_factor();

            // textures
            let albedo_tex = pbr
                .base_color_texture()
                .map(|info| {
                    let index = info.texture().index();
                    load_texture(
                        device,
                        uniforms,
                        &mut textures,
                        &mut texture_data,
                        storage,
                        ColorSpace::Srgb,
                        index,
                    )
                })
                .unwrap_or(Ok(0.0))?;
            let emissive_tex = material
                .emissive_texture()
                .map(|info| {
                    let index = info.texture().index();
                    load_texture(
                        device,
                        uniforms,
                        &mut textures,
                        &mut texture_data,
                        storage,
                        ColorSpace::Srgb,
                        index,
                    )
                })
                .unwrap_or(Ok(0.0))?;
            let met_rough_tex = pbr
                .metallic_roughness_texture()
                .map(|info| {
                    let index = info.texture().index();
                    load_texture(
                        device,
                        uniforms,
                        &mut textures,
                        &mut texture_data,
                        storage,
                        ColorSpace::Linear,
                        index,
                    )
                })
                .unwrap_or(Ok(0.0))?;
            let normal_tex = material
                .normal_texture()
                .map(|info| {
                    let index = info.texture().index();
                    load_texture(
                        device,
                        uniforms,
                        &mut textures,
                        &mut texture_data,
                        storage,
                        ColorSpace::Linear,
                        index,
                    )
                })
                .unwrap_or(Ok(0.0))?;
            let occlusion_tex = material
                .occlusion_texture()
                .map(|info| {
                    let index = info.texture().index();
                    load_texture(
                        device,
                        uniforms,
                        &mut textures,
                        &mut texture_data,
                        storage,
                        ColorSpace::Linear,
                        index,
                    )
                })
                .unwrap_or(Ok(0.0))?;

            let mut mat = Material::new(device, uniforms);
            mat.set_arg_1([albedo[0], albedo[1], albedo[2], albedo_tex]);
            mat.set_arg_2([metalness, roughness, met_rough_tex, occlusion_tex]);
            mat.set_arg_3([normal_tex, emissive_tex, 0.0, 0.0]);
            mat.set_arg_4([emissive[0], emissive[1], emissive[2], 0.0]);
            mat.update_if_needed();

            materials.insert(
                material.index().ok_or(Error::InvalidGltf)?,
                storage.add_material(mat),
            );
        }

        // load scenes
        let mut draw_nodes = vec![];
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                draw_nodes.push(ModelNode::new(&node, &meshes, &materials)?);
            }
        }

        Ok(Self {
            _textures: textures,
            draw_nodes,
        })
    }

    #[allow(clippy::unused_self)]
    pub fn fix_color_space(&mut self) {}

    pub(crate) fn nodes(&self) -> impl Iterator<Item = &ModelNode> {
        self.draw_nodes.iter()
    }
}

impl ModelNode {
    fn new(
        node: &Node<'_>,
        meshes: &HashMap<(usize, usize), Handle<Mesh>>,
        materials: &HashMap<usize, Handle<Material>>,
    ) -> Result<Self> {
        // get transform matrix
        let matrix = {
            let m = node.transform().matrix();
            Matrix4::from_columns(m[0], m[1], m[2], m[3])
        };

        // get mesh and material
        let mut ms = vec![];
        let mut mats = vec![];
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let m = meshes
                    .get(&(mesh.index(), primitive.index()))
                    .cloned()
                    .ok_or(Error::InvalidGltf)?;
                let mat = materials
                    .get(&primitive.material().index().ok_or(Error::InvalidGltf)?)
                    .cloned()
                    .ok_or(Error::InvalidGltf)?;
                ms.push(m);
                mats.push(mat);
            }
        }

        // get children
        let children: Vec<_> = node
            .children()
            .map(|n| Self::new(&n, meshes, materials))
            .collect::<Result<_>>()?;

        Ok(Self {
            meshes: ms,
            materials: mats,
            children,
            matrix,
        })
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = (&Handle<Mesh>, &Handle<Material>)> {
        self.meshes.iter().zip(self.materials.iter())
    }

    pub(crate) fn children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }

    pub(crate) const fn matrix(&self) -> Matrix4 {
        self.matrix
    }
}

fn load_texture(
    device: &Device,
    uniforms: &mut Uniforms,
    textures: &mut HashMap<usize, Handle<Texture>>,
    texture_data: &mut HashMap<usize, (&str, &[u8])>,
    storage: &mut Storage,
    color_space: ColorSpace,
    index: usize,
) -> Result<f32> {
    if let Some(tex) = textures.get(&index) {
        Ok(tex.id() as f32)
    } else {
        let (mime_type, data) = texture_data.remove(&index).ok_or(Error::InvalidGltf)?;

        let tex = match mime_type {
            #[cfg(feature = "png")]
            "image/png" => {
                Texture::from_png_bytes(device, uniforms, data, color_space, Mips::Log2)?
            }
            _ => return Err(Error::UnsupportedMimeType(mime_type.to_string())),
        };

        let id = tex.shader_index() as f32;
        textures.insert(index, storage.add_texture(tex));
        Ok(id)
    }
}
