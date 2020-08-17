// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use crate::color::Color;
use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::mesh::Mesh;
use crate::mesh::MeshUpdateData;
use crate::pipeline::Material;
use crate::pipeline::MaterialUpdateData;
use crate::pipeline::Shader;
use crate::renderer::Light;
use crate::resource::Builtins;
use crate::resource::Index;
use crate::resource::Ref;
use crate::resource::ResourceManager;

pub struct Target<'storage> {
    pub bias: f32,
    pub clear: Color,
    pub skybox: bool,
    pub wireframes: bool,
    pub font_size: u32,
    pub line_width: f32,
    pub cascade_splits: [f32; 4],
    pub cast_shadows: bool,
    pub lights: [Light; 4],
    pub texture_filter: TextureFilter,
    pub texture_wrap: TextureWrap,
    pub texture_mipmaps: bool,

    pub(crate) orders_by_shader: Vec<OrdersByShader>,
    pub(crate) text_orders: Vec<TextOrder>,
    pub(crate) has_shadow_casters: bool,
    pub(crate) builtins: &'storage Builtins,
    pub(crate) resources: &'storage mut ResourceManager,

    current_shader: Ref<Shader>,
    current_material: Index,
    current_font_material: Index,
    current_albedo: Albedo,
    current_font: Ref<Font>,
}

pub(crate) struct RenderData {
    pub(crate) line_width: f32,
    pub(crate) bias: f32,
    pub(crate) clear: Color,
    pub(crate) skybox: bool,

    pub(crate) has_shadow_casters: bool,
    pub(crate) lights: [Light; 4],
    pub(crate) cascade_splits: [f32; 4],

    pub(crate) orders_by_shader: Vec<OrdersByShader>,
    pub(crate) text_orders: Vec<TextOrder>,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Ref<Shader>,
    pub(crate) orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Index,
    pub(crate) orders: Vec<Order>,
}

#[derive(Clone)]
pub(crate) struct Order {
    pub(crate) mesh: Index,
    pub(crate) albedo: Albedo,
    pub(crate) model: Matrix4,
    pub(crate) cast_shadows: bool,
    pub(crate) sampler_index: i32,
}

pub(crate) struct TextOrder {
    pub(crate) font: Ref<Font>,
    pub(crate) size: u32,
    pub(crate) shader: Ref<Shader>,
    pub(crate) material: Index,
    pub(crate) text: String,
    pub(crate) transform: Transform,
    pub(crate) sampler_index: i32,
}

#[derive(Clone)]
pub enum Albedo {
    Texture(Ref<Texture>),
    Framebuffer(Index),
}

struct Cache {
    current_shader: Ref<Shader>,
    current_material: Index,
    current_font_material: Index,
    current_albedo: Albedo,
    current_font: Ref<Font>,
    texture_filter: TextureFilter,
    texture_wrap: TextureWrap,
    texture_mipmaps: bool,
    cast_shadows: bool,
}

impl<'storage> Target<'storage> {
    pub(crate) fn new(
        builtins: &'storage Builtins,
        resources: &'storage mut ResourceManager,
    ) -> Result<Self> {
        // update builtins
        let current_material = &builtins.white_material;
        resources
            .material_mut(&current_material.index)
            .update_if_needed(current_material.data(), current_material.index.version())?;
        let current_font_material = &builtins.font_material;
        resources
            .material_mut(&current_font_material.index)
            .update_if_needed(
                current_font_material.data(),
                current_font_material.index.version(),
            )?;

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
            current_material: current_material.index.clone(),
            current_font_material: current_font_material.index.clone(),
            current_albedo: Albedo::Texture(builtins.white_texture.clone()),
            current_font: builtins.kenney_font.clone(),
            texture_filter: TextureFilter::Linear,
            texture_wrap: TextureWrap::Repeat,
            texture_mipmaps: true,
            font_size: 24,
            cast_shadows: true,
            wireframes: false,
            skybox: false,
            has_shadow_casters: false,
            cascade_splits: [0.1, 0.25, 0.7, 1.0],
            line_width: 1.0,
            bias: 0.002,
            resources,
            builtins,
        })
    }

    pub fn draw(&mut self, mesh: &Mesh, transform: impl Into<Transform>) {
        // update mesh if needed
        self.resources
            .mesh_mut(&mesh.index)
            .update_if_needed(mesh.data(), mesh.index.version());
        // TODO: error on out of memory
        // most Vulkan errors are unrecoverable

        // add order for mesh
        self.add_order(Order {
            mesh: mesh.index.clone(),
            albedo: self.current_albedo.clone(),
            model: transform.into().as_matrix(),
            cast_shadows: self.cast_shadows,
            sampler_index: self.sampler_index(),
        });
    }

    pub fn draw_debug_cube(&mut self, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        self.draw(&self.builtins.cube_mesh, transform);

        self.restore(cache);
    }

    pub fn draw_debug_sphere(&mut self, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        self.draw(&self.builtins.sphere_mesh, transform);

        self.restore(cache);
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(&self.builtins.cube_mesh, transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.draw(&self.builtins.sphere_mesh, transform);
    }

    pub fn draw_texture(&mut self, texture: &Ref<Texture>, transform: impl Into<Transform>) {
        let cache = self.store();
        self.current_albedo = Albedo::Texture(texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        self.draw(&self.builtins.quad_mesh, transform);

        self.restore(cache);
    }

    pub fn draw_surface(&mut self) {
        let cache = self.store();
        self.cast_shadows = false;

        self.draw(&self.builtins.surface_mesh, [0.0, 0.0, 0.0]);

        self.restore(cache);
    }

    pub fn blit_framebuffer(&mut self, framebuffer: &Framebuffer) {
        let cache = self.store();
        self.current_shader = self.builtins.blit_shader.clone();
        self.current_albedo = Albedo::Framebuffer(framebuffer.index.clone());

        self.draw_surface();

        self.restore(cache);
    }

    pub fn draw_grid(&mut self) {
        let cache = self.store();
        self.current_shader = self.builtins.line_shader.clone();
        self.cast_shadows = false;

        self.draw(&self.builtins.grid_mesh, [0.0, 0.0, 0.0]);

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

    pub fn set_material(&mut self, material: &Material) {
        // update material if needed
        self.resources
            .material_mut(&material.index)
            .update_if_needed(material.data(), material.index.version());

        self.current_material = material.index.clone();
    }

    pub fn set_font_material(&mut self, material: &Material) {
        // update material if needed
        self.resources
            .material_mut(&material.index)
            .update_if_needed(material.data(), material.index.version());

        self.current_font_material = material.index.clone();
    }

    pub fn set_albedo(&mut self, albedo: impl Into<Albedo>) {
        self.current_albedo = albedo.into();
    }

    pub fn set_shader(&mut self, shader: &Ref<Shader>) {
        self.current_shader = shader.clone();
    }

    pub(crate) fn render_data(self) -> RenderData {
        RenderData {
            line_width: self.line_width,
            bias: self.bias,
            clear: self.clear,
            skybox: self.skybox,
            has_shadow_casters: self.has_shadow_casters,
            lights: self.lights,
            cascade_splits: self.cascade_splits,
            orders_by_shader: self.orders_by_shader,
            text_orders: self.text_orders,
        }
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
                        material: self.builtins.white_material.index.clone(),
                        orders: vec![order],
                    }],
                }),
            }
        }
    }

    fn sampler_index(&self) -> i32 {
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
            current_font_material: self.current_font_material.clone(),
            current_albedo: self.current_albedo.clone(),
            current_font: self.current_font.clone(),
            texture_filter: self.texture_filter,
            texture_wrap: self.texture_wrap,
            texture_mipmaps: self.texture_mipmaps,
            cast_shadows: self.cast_shadows,
        }
    }

    fn restore(&mut self, cache: Cache) {
        self.current_shader = cache.current_shader;
        self.current_material = cache.current_material;
        self.current_font_material = cache.current_font_material;
        self.current_albedo = cache.current_albedo;
        self.current_font = cache.current_font;
        self.texture_filter = cache.texture_filter;
        self.texture_wrap = cache.texture_wrap;
        self.texture_mipmaps = cache.texture_mipmaps;
        self.cast_shadows = cache.cast_shadows;
    }
}

impl From<&Ref<Texture>> for Albedo {
    fn from(r: &Ref<Texture>) -> Self {
        Self::Texture(r.clone())
    }
}

impl From<&Framebuffer> for Albedo {
    fn from(r: &Framebuffer) -> Self {
        Self::Framebuffer(r.index.clone())
    }
}
