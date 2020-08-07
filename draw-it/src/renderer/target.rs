// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use crate::color::Color;
use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::mesh::Mesh;
use crate::pipeline::sampler_index;
use crate::pipeline::Material;
use crate::pipeline::SamplerAddress;
use crate::pipeline::SamplerFilter;
use crate::pipeline::SamplerMipmaps;
use crate::pipeline::Shader;
use crate::renderer::Light;
use crate::resource::Builtins;
use crate::resource::Ref;

pub struct Target {
    pub bias: f32,
    pub clear: Color,
    pub skybox: bool,
    pub wireframes: bool,
    pub font_size: u32,
    pub line_width: f32,
    pub cascade_splits: [f32; 4],
    pub cast_shadows: bool,
    pub lights: [Light; 4],

    pub(crate) orders_by_shader: Vec<OrdersByShader>,
    pub(crate) text_orders: Vec<TextOrder>,
    pub(crate) has_shadow_casters: bool,
    pub(crate) builtins: Builtins,

    current_shader: Ref<Shader>,
    current_material: Ref<Material>,
    current_font_material: Ref<Material>,
    current_albedo: Albedo,
    current_font: Ref<Font>,
    current_sampler: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SamplerOptions {
    pub filter: SamplerFilter,
    pub address: SamplerAddress,
    pub mipmaps: SamplerMipmaps,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Ref<Shader>,
    pub(crate) orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Ref<Material>,
    pub(crate) orders: Vec<Order>,
}

#[derive(Clone)]
pub(crate) struct Order {
    pub(crate) mesh: Ref<Mesh>,
    pub(crate) albedo: Albedo,
    pub(crate) model: Matrix4,
    pub(crate) cast_shadows: bool,
    pub(crate) sampler_index: i32,
}

pub(crate) struct TextOrder {
    pub(crate) font: Ref<Font>,
    pub(crate) size: u32,
    pub(crate) shader: Ref<Shader>,
    pub(crate) material: Ref<Material>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
    pub(crate) sampler_index: i32,
}

#[derive(Clone)]
pub enum Albedo {
    Texture(Ref<Texture>),
    Framebuffer(Ref<Framebuffer>),
}

struct Cache {
    current_shader: Ref<Shader>,
    current_material: Ref<Material>,
    current_font_material: Ref<Material>,
    current_albedo: Albedo,
    current_font: Ref<Font>,
    current_sampler: i32,
    cast_shadows: bool,
}

impl Target {
    pub(crate) fn new(builtins: &Builtins) -> Result<Self> {
        Ok(Self {
            orders_by_shader: vec![],
            text_orders: vec![],
            clear: Color::rgba_norm(0.7, 0.7, 0.7, 1.0),
            lights: [
                Light::directional([-1.0, -1.0, 1.0], Color::WHITE, true),
                Light::NONE,
                Light::NONE,
                Light::NONE,
            ],
            current_shader: builtins.phong_shader.clone(),
            current_material: builtins.white_material.clone(),
            current_font_material: builtins.font_material.clone(),
            current_albedo: Albedo::Texture(builtins.white_texture.clone()),
            current_font: builtins.kenney_font.clone(),
            font_size: 24,
            current_sampler: 0,
            cast_shadows: true,
            wireframes: false,
            skybox: false,
            has_shadow_casters: false,
            cascade_splits: [0.1, 0.25, 0.7, 1.0],
            line_width: 1.0,
            builtins: builtins.clone(),
            bias: 0.002,
        })
    }

    pub fn draw(&mut self, mesh: &Ref<Mesh>, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: mesh.clone(),
            albedo: self.current_albedo.clone(),
            model: transform.into().as_matrix(),
            cast_shadows: self.cast_shadows,
            sampler_index: self.current_sampler,
        });
    }

    pub fn draw_debug_cube(&mut self, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.cube_mesh.clone();
        self.draw(&mesh, transform);

        self.restore(cache);
    }

    pub fn draw_debug_sphere(&mut self, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.sphere_mesh.clone();
        self.draw(&mesh, transform);

        self.restore(cache);
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        let mesh = self.builtins.cube_mesh.clone();
        self.draw(&mesh, transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        let mesh = self.builtins.sphere_mesh.clone();
        self.draw(&mesh, transform);
    }

    pub fn draw_texture(&mut self, texture: &Ref<Texture>, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.quad_mesh.clone();
        self.draw(&mesh, transform);

        self.restore(cache);
    }

    pub fn draw_surface(&mut self) {
        let cache = self.store();
        self.cast_shadows = false;

        let mesh = self.builtins.surface_mesh.clone();
        self.draw(&mesh, [0.0, 0.0, 0.0]);

        self.restore(cache);
    }

    pub fn blit_framebuffer(&mut self, framebuffer: &Ref<Framebuffer>) {
        let cache = self.store();
        self.current_shader = self.builtins.blit_shader.clone();
        self.current_albedo = Albedo::Framebuffer(framebuffer.clone());

        self.draw_surface();

        self.restore(cache);
    }

    pub fn draw_grid(&mut self) {
        let cache = self.store();
        self.current_shader = self.builtins.line_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.grid_mesh.clone();
        self.draw(&mesh, [0.0, 0.0, 0.0]);

        self.restore(cache);
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let (shader, sampler_index) = if self.current_font.with(|f| f.is_bitmap(self.font_size)) {
            (self.builtins.bitmap_font_shader.clone(), 7)
        } else {
            (self.builtins.sdf_font_shader.clone(), 1)
        };

        self.text_orders.push(TextOrder {
            font: self.current_font.clone(),
            size: self.font_size,
            text: text.as_ref().to_string(),
            transform: transform.into(),
            material: self.current_font_material.clone(),
            sampler_index,
            shader,
        });
    }

    pub fn set_material(&mut self, material: &Ref<Material>) {
        self.current_material = material.clone();
    }

    pub fn set_font_material(&mut self, material: &Ref<Material>) {
        self.current_font_material = material.clone();
    }

    pub fn set_albedo(&mut self, albedo: impl Into<Albedo>) {
        self.current_albedo = albedo.into();
    }

    pub fn set_shader(&mut self, shader: &Ref<Shader>) {
        self.current_shader = shader.clone();
    }

    pub fn set_sampler(&mut self, options: SamplerOptions) {
        self.current_sampler = sampler_index(options.filter, options.address, options.mipmaps);
    }

    pub fn set_shader_phong(&mut self) {
        self.current_shader = self.builtins.phong_shader.clone();
    }

    pub fn set_shader_lines(&mut self) {
        self.current_shader = self.builtins.line_shader.clone();
    }

    pub fn set_albedo_white(&mut self) {
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
    }

    pub fn set_material_white(&mut self) {
        self.current_material = self.builtins.white_material.clone();
    }

    pub fn set_font_material_black(&mut self) {
        self.current_font_material = self.builtins.font_material.clone();
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
            let wireframe_shader = self.builtins.wireframe_shader.clone();
            match self
                .orders_by_shader
                .iter_mut()
                .find(|so| so.shader == wireframe_shader)
            {
                Some(so) => so.orders_by_material[0].orders.push(order),
                None => self.orders_by_shader.push(OrdersByShader {
                    shader: wireframe_shader,
                    orders_by_material: vec![OrdersByMaterial {
                        material: self.builtins.white_material.clone(),
                        orders: vec![order],
                    }],
                }),
            }
        }
    }

    fn store(&self) -> Cache {
        Cache {
            current_shader: self.current_shader.clone(),
            current_material: self.current_material.clone(),
            current_font_material: self.current_font_material.clone(),
            current_albedo: self.current_albedo.clone(),
            current_font: self.current_font.clone(),
            current_sampler: self.current_sampler,
            cast_shadows: self.cast_shadows,
        }
    }

    fn restore(&mut self, cache: Cache) {
        self.current_shader = cache.current_shader;
        self.current_material = cache.current_material;
        self.current_font_material = cache.current_font_material;
        self.current_albedo = cache.current_albedo;
        self.current_font = cache.current_font;
        self.current_sampler = cache.current_sampler;
        self.cast_shadows = cache.cast_shadows;
    }
}

impl Default for SamplerOptions {
    fn default() -> Self {
        Self {
            filter: SamplerFilter::Linear,
            address: SamplerAddress::Repeat,
            mipmaps: SamplerMipmaps::Enabled,
        }
    }
}

impl From<&Ref<Texture>> for Albedo {
    fn from(r: &Ref<Texture>) -> Self {
        Self::Texture(r.clone())
    }
}

impl From<&Ref<Framebuffer>> for Albedo {
    fn from(r: &Ref<Framebuffer>) -> Self {
        Self::Framebuffer(r.clone())
    }
}
