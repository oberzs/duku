use ash::vk::Buffer;
use ash::vk::DescriptorSet;
use ash::vk::Pipeline;
use tegne_math::Matrix4;
use tegne_math::Transform;
use tegne_math::Vector3;

use crate::builtins::BuiltinMaterial;
use crate::builtins::BuiltinMesh;
use crate::builtins::Builtins;
use crate::mesh::Mesh;
use crate::shaders::Light;
use crate::shaders::Material;

pub struct Target<'a> {
    material_orders: Vec<MaterialOrder>,
    clear: [f32; 3],
    lights: Vec<Light>,
    current_pipeline: Pipeline,
    current_material: (u32, DescriptorSet),
    current_albedo: i32,
    builtins: &'a Builtins,
}

pub(crate) struct MaterialOrder {
    pub(crate) pipeline: Pipeline,
    pub(crate) material_descriptor: (u32, DescriptorSet),
    pub(crate) albedo_index: i32,
    pub(crate) orders: Vec<Order>,
}

pub(crate) struct Order {
    pub(crate) model: Matrix4,
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) index_count: u32,
}

impl<'a> Target<'a> {
    pub(crate) fn new(builtins: &'a Builtins) -> Self {
        let material = builtins.get_material(BuiltinMaterial::White);

        Self {
            material_orders: vec![],
            clear: [0.7, 0.7, 0.7],
            lights: vec![],
            current_pipeline: material.pipeline(),
            current_material: material.uniforms().descriptor(),
            current_albedo: material.albedo_index(),
            builtins,
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, transform: impl Into<Transform>) {
        self.add_order(Order {
            model: transform.into().as_matrix(),
            vertex_buffer: mesh.vk_vertex_buffer(),
            index_buffer: mesh.vk_index_buffer(),
            index_count: mesh.drawn_triangles() * 3,
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.get_mesh(BuiltinMesh::Cube), transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.get_mesh(BuiltinMesh::Sphere), transform);
    }

    pub fn add_directional_light(
        &mut self,
        direction: impl Into<Vector3>,
        color: impl Into<Vector3>,
    ) {
        self.lights.push(Light {
            coords: direction.into().extend(0.0),
            color: color.into(),
        });
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

    pub(crate) fn material_orders(&self) -> &[MaterialOrder] {
        &self.material_orders
    }

    pub(crate) fn lights(&self) -> [Light; 4] {
        let mut lights: [Light; 4] = Default::default();
        lights[..self.lights.len()].clone_from_slice(&self.lights[..]);
        lights
    }

    fn add_order(&mut self, order: Order) {
        let material = self.current_material;

        match self
            .material_orders
            .iter_mut()
            .find(|mo| mo.material_descriptor == material)
        {
            Some(mo) => mo.orders.push(order),
            None => self.material_orders.push(MaterialOrder {
                pipeline: self.current_pipeline,
                material_descriptor: self.current_material,
                albedo_index: self.current_albedo,
                orders: vec![order],
            }),
        }
    }
}
