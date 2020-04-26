use ash::vk::Buffer;
use ash::vk::DescriptorSet;
use ash::vk::Pipeline;
use tegne_math::Transform;

use crate::builtins::BuiltinMaterial;
use crate::builtins::BuiltinMesh;
use crate::builtins::Builtins;
use crate::model::Material;
use crate::model::Mesh;
use crate::shaders::PushConstants;

pub struct Target<'a> {
    orders: Vec<Order>,
    clear: [f32; 3],
    current_pipeline: Pipeline,
    current_material: (u32, DescriptorSet),
    current_albedo: i32,
    builtins: &'a Builtins,
}

struct Order {
    pub(crate) pipeline: Pipeline,
    pub(crate) material_descriptor: (u32, DescriptorSet),
    pub(crate) push_consts: PushConstants,
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) index_count: u32,
}

impl<'a> Target<'a> {
    pub(crate) fn new(builtins: &'a Builtins) -> Self {
        let material = builtins.get_material(BuiltinMaterial::White);

        Self {
            orders: vec![],
            clear: [0.7, 0.7, 0.7],
            current_pipeline: material.pipeline(),
            current_material: material.uniforms().descriptor(),
            current_albedo: material.albedo_index(),
            builtins,
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, transform: impl Into<Transform>) {
        self.orders.push(Order {
            pipeline: self.current_pipeline,
            material_descriptor: self.current_material,
            push_consts: PushConstants {
                model: transform.into().as_matrix(),
                albedo_index: self.current_albedo,
            },
            vertex_buffer: mesh.vk_vertex_buffer(),
            index_buffer: mesh.vk_index_buffer(),
            index_count: mesh.drawn_triangles() * 3,
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.get_mesh(BuiltinMesh::Cube), transform);
    }

    pub fn set_material(&mut self, material: &Material) {
        self.current_pipeline = material.pipeline();
        self.current_material = material.uniforms().descriptor();
        self.current_albedo = material.albedo_index();
    }

    pub fn set_clear_color(&mut self, clear: [f32; 3]) {
        self.clear = clear;
    }

    pub(crate) fn clear(&self) -> [f32; 4] {
        [self.clear[0], self.clear[1], self.clear[2], 1.0]
    }
}
