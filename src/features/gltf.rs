// Oliver Berzs
// https://github.com/oberzs/duku

#![cfg(feature = "gltf")]

use gltf_dep::buffer;
use gltf_dep::image;
use gltf_dep::mesh::Mode;
use gltf_dep::Gltf;
use gltf_dep::Material as GltfMaterial;
use gltf_dep::Node;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::duku::Duku;
use crate::error::Error;
use crate::error::Result;
use crate::image::ColorSpace;
use crate::image::Mips;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Quaternion;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::Model;
use crate::mesh::ModelNode;
use crate::pipeline::Material;
use crate::resources::Handle;

impl Duku {
    /// Create a model from a GLTF file
    pub fn create_model_gltf(&mut self, path: impl AsRef<Path>) -> Result<Handle<Model>> {
        let p = path.as_ref();
        let bytes = fs::read(p)?;
        self.create_model_gltf_bytes(
            &bytes,
            p.parent()
                .unwrap_or_else(|| Path::new("./"))
                .to_str()
                .expect("bad path"),
        )
    }

    /// Create a model from GLTF bytes
    ///
    /// `root` is used for relative file path loading
    pub fn create_model_gltf_bytes(&mut self, bytes: &[u8], root: &str) -> Result<Handle<Model>> {
        let gltf = Gltf::from_slice(bytes).map_err(|_| Error::InvalidGltf)?;

        // load buffers
        let mut buffers = vec![];
        for buffer in gltf.buffers() {
            match buffer.source() {
                // external data
                buffer::Source::Uri(uri) => {
                    let bytes = fs::read(format!("{}/{}", root, uri))?;
                    buffers.push(bytes);
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

                let mut m = self.create_mesh();
                m.vertices = vertices;
                m.uvs = uvs;
                m.indices = indices;

                if normals.is_empty() {
                    m.calculate_normals();
                } else {
                    m.normals = normals;
                    m.calculate_tangents();
                }

                meshes.insert((mesh.index(), primitive.index()), m);
            }
        }

        // load textures
        let mut texture_data = HashMap::new();
        for texture in gltf.textures() {
            match texture.source().source() {
                // internal data
                image::Source::View { view, mime_type } => {
                    let start = view.offset() as usize;
                    let end = start + view.length() as usize;
                    let buffer = &buffers[view.buffer().index()][start..end];
                    texture_data.insert(texture.index(), (mime_type, buffer.to_vec()));
                }

                // external data
                image::Source::Uri { uri, mime_type } => {
                    // check if mime type was provided
                    let mime = match mime_type {
                        Some(m) => m,
                        None => {
                            let ext = uri.split('.').last().ok_or(Error::InvalidGltf)?;
                            match ext {
                                "png" => "image/png",
                                "jpg" | "jpeg" => "image/jpeg",
                                _ => return Err(Error::UnsupportedMimeType(ext.to_string())),
                            }
                        }
                    };

                    let bytes = fs::read(format!("{}/{}", root, uri))?;
                    texture_data.insert(texture.index(), (mime, bytes));
                }
            }
        }

        // load materials
        let mut materials = HashMap::new();
        for material in gltf.materials() {
            let mat = self.load_material(&mut texture_data, &material)?;
            materials.insert(material.index().unwrap_or(0), mat);
        }

        // load scenes
        let mut nodes = vec![];
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                nodes.push(load_node(&node, &meshes, &materials)?);
            }
        }

        let mut model = self.create_model();
        model.nodes = nodes;

        Ok(model)
    }

    fn load_material(
        &mut self,
        texture_data: &mut HashMap<usize, (&str, Vec<u8>)>,
        material: &GltfMaterial<'_>,
    ) -> Result<Handle<Material>> {
        let mut textures = HashMap::new();
        let pbr = material.pbr_metallic_roughness();

        // factors
        let albedo = pbr.base_color_factor();
        let metalness = pbr.metallic_factor();
        let roughness = pbr.roughness_factor();
        let emissive = material.emissive_factor();

        // textures
        let albedo_tex = if let Some(info) = pbr.base_color_texture() {
            Some(self.load_texture(
                &mut textures,
                texture_data,
                ColorSpace::Srgb,
                info.texture().index(),
            )?)
        } else {
            None
        };
        let emissive_tex = if let Some(info) = material.emissive_texture() {
            Some(self.load_texture(
                &mut textures,
                texture_data,
                ColorSpace::Srgb,
                info.texture().index(),
            )?)
        } else {
            None
        };
        let met_rough_tex = if let Some(info) = pbr.metallic_roughness_texture() {
            Some(self.load_texture(
                &mut textures,
                texture_data,
                ColorSpace::Linear,
                info.texture().index(),
            )?)
        } else {
            None
        };
        let normal_tex = if let Some(info) = material.normal_texture() {
            Some(self.load_texture(
                &mut textures,
                texture_data,
                ColorSpace::Linear,
                info.texture().index(),
            )?)
        } else {
            None
        };
        let occ_tex = if let Some(info) = material.occlusion_texture() {
            Some(self.load_texture(
                &mut textures,
                texture_data,
                ColorSpace::Linear,
                info.texture().index(),
            )?)
        } else {
            None
        };

        // build material
        let mut mat = self.create_material()?;
        mat.albedo_color(albedo);
        mat.metalness(metalness);
        mat.roughness(roughness);
        mat.emissive(emissive);
        if let Some(tex) = albedo_tex {
            mat.albedo_texture(tex);
        }
        if let Some(tex) = emissive_tex {
            mat.emissive_texture(tex);
        }
        if let Some(tex) = met_rough_tex {
            mat.metalness_roughness_texture(tex);
        }
        if let Some(tex) = normal_tex {
            mat.normal_texture(tex);
        }
        if let Some(tex) = occ_tex {
            mat.ambient_occlusion_texture(tex);
        }
        Ok(mat)
    }

    fn load_texture(
        &mut self,
        textures: &mut HashMap<usize, Handle<Texture>>,
        texture_data: &mut HashMap<usize, (&str, Vec<u8>)>,
        color_space: ColorSpace,
        index: usize,
    ) -> Result<Handle<Texture>> {
        if let Some(tex) = textures.get(&index) {
            Ok(tex.clone())
        } else {
            let (mime_type, data) = texture_data.remove(&index).ok_or(Error::InvalidGltf)?;
            let tex = match mime_type {
                "image/png" => {
                    self.create_texture_png_bytes(&data, Some((color_space, Mips::Log2)))?
                }
                "image/jpeg" => {
                    self.create_texture_jpeg_bytes(&data, Some((color_space, Mips::Log2)))?
                }
                _ => return Err(Error::UnsupportedMimeType(mime_type.to_string())),
            };
            textures.insert(index, tex.clone());
            Ok(tex)
        }
    }
}

fn load_node(
    node: &Node<'_>,
    meshes: &HashMap<(usize, usize), Handle<Mesh>>,
    materials: &HashMap<usize, Handle<Material>>,
) -> Result<ModelNode> {
    // get transform matrix
    let matrix = {
        let (t, r, s) = node.transform().decomposed();
        Matrix4::translation([t[0], t[1], -t[2]])
            * Matrix4::from(Quaternion::new(r[0], r[1], -r[2], -r[3]))
            * Matrix4::scale(s)
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
                .get(&primitive.material().index().unwrap_or(0))
                .cloned()
                .ok_or(Error::InvalidGltf)?;
            ms.push(m);
            mats.push(mat);
        }
    }

    // get children
    let children: Vec<_> = node
        .children()
        .map(|n| load_node(&n, meshes, materials))
        .collect::<Result<_>>()?;
    Ok(ModelNode {
        meshes: ms,
        materials: mats,
        children,
        matrix,
    })
}
