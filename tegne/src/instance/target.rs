use ash::vk::Buffer;
use ash::vk::Pipeline;
use tegne_math::Matrix4;
use tegne_math::Transform;
use tegne_math::Vector3;

use crate::builtins::BuiltinFont;
use crate::builtins::BuiltinMaterial;
use crate::builtins::BuiltinMesh;
use crate::builtins::BuiltinShader;
use crate::builtins::Builtins;
use crate::images::Font;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::shaders::Descriptor;
use crate::shaders::Light;
use crate::shaders::Material;

pub struct Target<'a> {
    orders_by_shader: Vec<OrdersByShader>,
    wireframe_orders: Vec<Order>,
    clear: [f32; 3],
    lights: Vec<Light>,
    current_shader: Pipeline,
    current_material: Descriptor,
    current_albedo: i32,
    current_font: &'a Font,
    draw_wireframes: bool,
    builtins: &'a Builtins,
}

pub(crate) struct OrdersByShader {
    shader: Pipeline,
    orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    material: Descriptor,
    orders: Vec<Order>,
}

#[derive(Copy, Clone)]
pub(crate) struct Order {
    pub(crate) model: Matrix4,
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) index_count: u32,
    pub(crate) albedo_index: i32,
    pub(crate) has_shadows: bool,
}

impl<'a> Target<'a> {
    pub(crate) fn new(builtins: &'a Builtins) -> Self {
        let material = builtins.get_material(BuiltinMaterial::White);
        let font = builtins.get_font(BuiltinFont::NotoSans);

        Self {
            orders_by_shader: vec![],
            wireframe_orders: vec![],
            clear: [0.7, 0.7, 0.7],
            lights: vec![],
            current_shader: material.pipeline(),
            current_material: material.uniforms().descriptor(),
            current_albedo: material.albedo_index(),
            current_font: font,
            draw_wireframes: false,
            builtins,
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, transform: impl Into<Transform>) {
        self.add_order(Order {
            model: transform.into().as_matrix(),
            vertex_buffer: mesh.vk_vertex_buffer(),
            index_buffer: mesh.vk_index_buffer(),
            index_count: mesh.drawn_triangles() * 3,
            albedo_index: self.current_albedo,
            has_shadows: true,
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.get_mesh(BuiltinMesh::Cube), transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.get_mesh(BuiltinMesh::Sphere), transform);
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let temp_shader = self.current_shader;

        let shader = self.builtins.get_shader(BuiltinShader::Font);
        self.current_shader = shader.pipeline();

        let mut current_transform = transform.into();

        for c in text.as_ref().chars() {
            if c == ' ' {
                current_transform.position.x += self.current_font.char_advance('s');
                continue;
            }

            let mesh = self.current_font.char_mesh(c);
            self.add_order(Order {
                model: current_transform.as_matrix(),
                vertex_buffer: mesh.vk_vertex_buffer(),
                index_buffer: mesh.vk_index_buffer(),
                index_count: mesh.drawn_triangles() * 3,
                albedo_index: self.current_font.image_index(),
                has_shadows: false,
            });

            current_transform.position.x += self.current_font.char_advance(c);
        }

        self.current_shader = temp_shader;
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
        self.current_shader = material.pipeline();
        self.current_material = material.uniforms().descriptor();
        self.current_albedo = material.albedo_index();
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.current_albedo = texture.image_index();
    }

    pub fn reset_material(&mut self) {
        let material = self.builtins.get_material(BuiltinMaterial::White);
        self.current_shader = material.pipeline();
        self.current_material = material.uniforms().descriptor();
        self.current_albedo = material.albedo_index();
    }

    pub fn set_clear_color(&mut self, clear: [f32; 3]) {
        self.clear = clear;
    }

    pub fn set_draw_wireframes(&mut self, draw: bool) {
        self.draw_wireframes = draw;
    }

    pub(crate) fn clear(&self) -> [f32; 4] {
        [self.clear[0], self.clear[1], self.clear[2], 1.0]
    }

    pub(crate) fn orders_by_shader(&self) -> impl Iterator<Item = &OrdersByShader> {
        self.orders_by_shader.iter()
    }

    pub(crate) fn wireframe_orders(&self) -> impl Iterator<Item = Order> + '_ {
        self.wireframe_orders.iter().cloned()
    }

    pub(crate) fn lights(&self) -> [Light; 4] {
        let mut lights: [Light; 4] = Default::default();
        lights[..self.lights.len()].clone_from_slice(&self.lights[..]);
        lights
    }

    fn add_order(&mut self, order: Order) {
        let material = self.current_material;
        let shader = self.current_shader;

        match self
            .orders_by_shader
            .iter_mut()
            .find(|so| so.shader == shader)
        {
            Some(so) => match so
                .orders_by_material
                .iter_mut()
                .find(|mo| mo.material == material)
            {
                Some(mo) => mo.orders.push(order),
                None => so.orders_by_material.push(OrdersByMaterial {
                    material,
                    orders: vec![order],
                }),
            },
            None => self.orders_by_shader.push(OrdersByShader {
                shader,
                orders_by_material: vec![OrdersByMaterial {
                    material,
                    orders: vec![order],
                }],
            }),
        }

        if self.draw_wireframes {
            self.wireframe_orders.push(order);
        }
    }
}

impl OrdersByShader {
    pub(crate) fn shader(&self) -> Pipeline {
        self.shader
    }

    pub(crate) fn orders_by_material(&self) -> impl Iterator<Item = &OrdersByMaterial> {
        self.orders_by_material.iter()
    }
}

impl OrdersByMaterial {
    pub(crate) fn material(&self) -> Descriptor {
        self.material
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = Order> + '_ {
        self.orders.iter().cloned()
    }
}
