// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use crate::color::Color;
use crate::font::Font;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::renderer::Light;
use crate::storage::Builtins;
use crate::storage::Handle;

pub struct Target<'a, 'b> {
    // global
    pub clear_color: Color,
    pub skybox: bool,
    pub transform: Transform,
    pub(crate) builtins: &'b Builtins,

    // meshes
    pub shader: Option<&'a Handle<Shader>>,
    pub material: Option<&'a Handle<Material>>,
    pub texture_filter: TextureFilter,
    pub texture_wrap: TextureWrap,
    pub texture_mipmaps: bool,
    pub wireframes: bool,
    pub(crate) mesh_orders: Vec<OrdersByShader>,

    // shadows & lights
    pub shadow_bias: f32,
    pub shadow_cascades: [f32; 4],
    pub cast_shadows: bool,
    pub lights: [Light; 4],
    pub(crate) has_shadow_casters: bool,

    // lines
    pub line_color: Color,
    pub(crate) line_orders: Vec<LineOrder>,

    // text
    pub font_size: u32,
    pub font: Option<&'a Handle<Font>>,
    pub text_color: Color,
    pub(crate) text_orders: Vec<TextOrder>,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Handle<Shader>,
    pub(crate) orders: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Handle<Material>,
    pub(crate) orders: Vec<MeshOrder>,
}

#[derive(Clone)]
pub(crate) struct MeshOrder {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) local_to_world: Matrix4,
    pub(crate) cast_shadows: bool,
    pub(crate) sampler_index: u32,
}

#[derive(Clone)]
pub(crate) struct TextOrder {
    pub(crate) size: u32,
    pub(crate) color: Color,
    pub(crate) font: Handle<Font>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
}

#[derive(Clone)]
pub(crate) struct LineOrder {
    pub(crate) color: Color,
    pub(crate) point_1: Vector3,
    pub(crate) point_2: Vector3,
    pub(crate) transform: Transform,
}

impl<'b> Target<'_, 'b> {
    pub(crate) fn new(builtins: &'b Builtins) -> Self {
        Self {
            mesh_orders: vec![],
            text_orders: vec![],
            line_orders: vec![],
            clear_color: Color::WHITE,
            transform: Transform::default(),
            lights: [
                Light::directional((-1.0, -1.0, 1.0), Color::WHITE, true),
                Light::NONE,
                Light::NONE,
                Light::NONE,
            ],
            texture_filter: TextureFilter::Linear,
            texture_wrap: TextureWrap::Repeat,
            font_size: 24,
            font: None,
            shader: None,
            material: None,
            text_color: Color::BLACK,
            line_color: Color::BLACK,
            texture_mipmaps: true,
            wireframes: false,
            skybox: false,
            has_shadow_casters: false,
            shadow_cascades: [0.1, 0.25, 0.7, 1.0],
            shadow_bias: 0.002,
            cast_shadows: true,
            builtins,
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Handle<Mesh>) {
        let shader = self.shader.unwrap_or(&self.builtins.phong_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                cast_shadows: self.cast_shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_cube(&mut self) {
        let shader = self.shader.unwrap_or(&self.builtins.phong_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.cube_mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                cast_shadows: self.cast_shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_sphere(&mut self) {
        let shader = self.shader.unwrap_or(&self.builtins.phong_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.sphere_mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                cast_shadows: self.cast_shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_surface(&mut self) {
        let shader = self.shader.unwrap_or(&self.builtins.phong_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.surface_mesh.clone(),
                local_to_world: Transform::positioned(0.0, 0.0, 0.0).as_matrix(),
                cast_shadows: false,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_grid(&mut self) {
        let shader = self.builtins.line_shader.clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.grid_mesh.clone(),
                local_to_world: Transform::positioned(0.0, 0.0, 0.0).as_matrix(),
                cast_shadows: false,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_text<T, V>(&mut self, text: T, position: V)
    where
        T: AsRef<str>,
        V: Into<Vector2>,
    {
        let font = self.font.unwrap_or(&self.builtins.fira_font).clone();

        let mut transform = self.transform;
        transform.position += position.into().extend(0.0);

        self.text_orders.push(TextOrder {
            size: self.font_size,
            color: self.text_color,
            text: text.as_ref().to_string(),
            transform,
            font,
        });
    }

    pub fn draw_line<V>(&mut self, point_1: V, point_2: V)
    where
        V: Into<Vector3>,
    {
        self.line_orders.push(LineOrder {
            color: self.line_color,
            point_1: point_1.into(),
            point_2: point_2.into(),
            transform: self.transform,
        });
    }

    fn add_mesh_order(
        &mut self,
        material: Handle<Material>,
        shader: Handle<Shader>,
        order: MeshOrder,
    ) {
        if order.cast_shadows {
            self.has_shadow_casters = true;
        }

        match self.mesh_orders.iter_mut().find(|so| so.shader == shader) {
            Some(so) => match so.orders.iter_mut().find(|mo| mo.material == material) {
                Some(mo) => mo.orders.push(order.clone()),
                None => so.orders.push(OrdersByMaterial {
                    material,
                    orders: vec![order.clone()],
                }),
            },
            None => self.mesh_orders.push(OrdersByShader {
                shader,
                orders: vec![OrdersByMaterial {
                    material,
                    orders: vec![order.clone()],
                }],
            }),
        }

        if self.wireframes {
            let wireframe_shader = self.builtins.wireframe_shader.clone();
            match self
                .mesh_orders
                .iter_mut()
                .find(|so| so.shader == wireframe_shader)
            {
                Some(so) => so.orders[0].orders.push(order),
                None => self.mesh_orders.push(OrdersByShader {
                    shader: wireframe_shader,
                    orders: vec![OrdersByMaterial {
                        material: self.builtins.white_material.clone(),
                        orders: vec![order],
                    }],
                }),
            }
        }
    }

    const fn sampler_index(&self) -> u32 {
        use TextureFilter as F;
        use TextureWrap as W;

        match (self.texture_filter, self.texture_wrap, self.texture_mipmaps) {
            (F::Linear, W::Repeat, true) => 0,
            (F::Linear, W::Repeat, false) => 1,
            (F::Linear, W::ClampBorder, true) => 2,
            (F::Linear, W::ClampBorder, false) => 3,
            (F::Linear, W::ClampEdge, true) => 4,
            (F::Linear, W::ClampEdge, false) => 5,
            (F::Nearest, W::Repeat, true) => 6,
            (F::Nearest, W::Repeat, false) => 7,
            (F::Nearest, W::ClampBorder, true) => 8,
            (F::Nearest, W::ClampBorder, false) => 9,
            (F::Nearest, W::ClampEdge, true) => 10,
            (F::Nearest, W::ClampEdge, false) => 11,
        }
    }
}
