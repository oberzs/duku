// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use super::TextTarget;
use crate::color::Color;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::renderer::Light;
use crate::storage::Builtins;
use crate::storage::Index;

pub struct Target<'b> {
    pub text: TextTarget,

    pub bias: f32,
    pub clear: Color,
    pub skybox: bool,
    pub wireframes: bool,
    pub line_width: f32,
    pub cascade_splits: [f32; 4],
    pub cast_shadows: bool,
    pub lights: [Light; 4],
    pub texture_filter: TextureFilter,
    pub texture_wrap: TextureWrap,
    pub texture_mipmaps: bool,

    pub(crate) orders_by_shader: Vec<OrdersByShader>,
    pub(crate) has_shadow_casters: bool,
    pub(crate) builtins: &'b Builtins,

    current_shader: Index,
    current_material: Index,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Index,
    pub(crate) orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Index,
    pub(crate) orders: Vec<Order>,
}

#[derive(Clone)]
pub(crate) struct Order {
    pub(crate) mesh: Index,
    pub(crate) model: Matrix4,
    pub(crate) cast_shadows: bool,
    pub(crate) sampler_index: i32,
}

struct Cache {
    current_shader: Index,
    current_material: Index,
    texture_filter: TextureFilter,
    texture_wrap: TextureWrap,
    texture_mipmaps: bool,
    cast_shadows: bool,
}

impl<'b> Target<'b> {
    pub(crate) fn new(builtins: &'b Builtins) -> Self {
        Self {
            text: TextTarget::new(builtins),
            orders_by_shader: vec![],
            clear: Color::rgba_norm(0.7, 0.7, 0.7, 1.0),
            lights: [
                Light::directional([-1.0, -1.0, 1.0], Color::WHITE, true),
                Light::NONE,
                Light::NONE,
                Light::NONE,
            ],
            current_shader: builtins.phong_shader.index.clone(),
            current_material: builtins.white_material.index.clone(),
            texture_filter: TextureFilter::Linear,
            texture_wrap: TextureWrap::Repeat,
            texture_mipmaps: true,
            cast_shadows: true,
            wireframes: false,
            skybox: false,
            has_shadow_casters: false,
            cascade_splits: [0.1, 0.25, 0.7, 1.0],
            line_width: 1.0,
            bias: 0.002,
            builtins,
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: mesh.index.clone(),
            model: transform.into().as_matrix(),
            cast_shadows: self.cast_shadows,
            sampler_index: self.sampler_index(),
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(&self.builtins.cube_mesh, transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.draw(&self.builtins.sphere_mesh, transform);
    }

    pub fn draw_surface(&mut self) {
        let cache = self.store();
        self.cast_shadows = false;

        self.draw(&self.builtins.surface_mesh, [0.0, 0.0, 0.0]);

        self.restore(cache);
    }

    pub fn draw_grid(&mut self) {
        let cache = self.store();
        self.current_shader = self.builtins.line_shader.index.clone();
        self.cast_shadows = false;

        self.draw(&self.builtins.grid_mesh, [0.0, 0.0, 0.0]);

        self.restore(cache);
    }

    pub fn set_material(&mut self, material: &Material) {
        self.current_material = material.index.clone();
    }

    pub fn set_shader(&mut self, shader: &Shader) {
        self.current_shader = shader.index.clone();
    }

    fn add_order(&mut self, order: Order) {
        let material = self.current_material.clone();
        let shader = self.current_shader.clone();

        if self.cast_shadows {
            self.has_shadow_casters = true;
        }

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
                Some(mo) => mo.orders.push(order.clone()),
                None => so.orders_by_material.push(OrdersByMaterial {
                    material,
                    orders: vec![order.clone()],
                }),
            },
            None => self.orders_by_shader.push(OrdersByShader {
                shader,
                orders_by_material: vec![OrdersByMaterial {
                    material,
                    orders: vec![order.clone()],
                }],
            }),
        }

        if self.wireframes {
            let wireframe_shader = self.builtins.wireframe_shader.index.clone();
            match self
                .orders_by_shader
                .iter_mut()
                .find(|so| so.shader == wireframe_shader)
            {
                Some(so) => so.orders_by_material[0].orders.push(order),
                None => self.orders_by_shader.push(OrdersByShader {
                    shader: wireframe_shader,
                    orders_by_material: vec![OrdersByMaterial {
                        material: self.builtins.white_material.index.clone(),
                        orders: vec![order],
                    }],
                }),
            }
        }
    }

    const fn sampler_index(&self) -> i32 {
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

    fn store(&self) -> Cache {
        Cache {
            current_shader: self.current_shader.clone(),
            current_material: self.current_material.clone(),
            texture_filter: self.texture_filter,
            texture_wrap: self.texture_wrap,
            texture_mipmaps: self.texture_mipmaps,
            cast_shadows: self.cast_shadows,
        }
    }

    fn restore(&mut self, cache: Cache) {
        self.current_shader = cache.current_shader;
        self.current_material = cache.current_material;
        self.texture_filter = cache.texture_filter;
        self.texture_wrap = cache.texture_wrap;
        self.texture_mipmaps = cache.texture_mipmaps;
        self.cast_shadows = cache.cast_shadows;
    }
}
