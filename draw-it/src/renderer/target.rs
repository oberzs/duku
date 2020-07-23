// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use crate::color::colors;
use crate::color::Color;
use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Light;
use crate::pipeline::Material;
use crate::pipeline::SamplerAddress;
use crate::pipeline::SamplerFilter;
use crate::pipeline::SamplerMipmaps;
use crate::pipeline::Shader;
use crate::resource::Builtins;
use crate::resource::Ref;

pub struct Target {
    pub(crate) orders_by_shader: Vec<OrdersByShader>,
    pub(crate) text_orders: Vec<TextOrder>,
    pub(crate) clear: Color,
    pub(crate) do_shadow_mapping: bool,
    pub(crate) cascade_splits: [f32; 4],
    pub(crate) line_width: f32,
    pub(crate) main_light: Light,
    pub(crate) builtins: Builtins,

    lights: Vec<Light>,
    current_shader: Ref<Shader>,
    current_material: Ref<Material>,
    current_font_material: Ref<Material>,
    current_albedo: Albedo,
    current_font: Ref<Font>,
    current_font_size: u32,
    current_sampler: i32,
    wireframes: bool,
    cast_shadows: bool,
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

impl Target {
    pub(crate) fn new(builtins: &Builtins) -> Result<Self> {
        Ok(Self {
            orders_by_shader: vec![],
            text_orders: vec![],
            clear: Color::rgba_norm(0.7, 0.7, 0.7, 1.0),
            main_light: Light {
                coords: Vector3::new(-0.5, -1.0, 1.0).unit().extend(0.0),
                color: colors::WHITE.to_rgba_norm_vec(),
            },
            lights: vec![],
            current_shader: builtins.phong_shader.clone(),
            current_material: builtins.white_material.clone(),
            current_font_material: builtins.font_material.clone(),
            current_albedo: Albedo::Texture(builtins.white_texture.clone()),
            current_font: builtins.kenney_font.clone(),
            current_font_size: 24,
            current_sampler: 0,
            cast_shadows: true,
            wireframes: false,
            do_shadow_mapping: false,
            cascade_splits: [0.1, 0.25, 0.7, 1.0],
            line_width: 1.0,
            builtins: builtins.clone(),
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
        let temp_albedo = self.current_albedo.clone();
        let temp_shader = self.current_shader.clone();
        let temp_shadows = self.cast_shadows;
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.cube_mesh.clone();
        self.draw(&mesh, transform);

        self.current_albedo = temp_albedo;
        self.current_shader = temp_shader;
        self.cast_shadows = temp_shadows;
    }

    pub fn draw_debug_sphere(&mut self, transform: impl Into<Transform>) {
        let temp_albedo = self.current_albedo.clone();
        let temp_shader = self.current_shader.clone();
        let temp_shadows = self.cast_shadows;
        self.current_albedo = Albedo::Texture(self.builtins.white_texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.sphere_mesh.clone();
        self.draw(&mesh, transform);

        self.current_albedo = temp_albedo;
        self.current_shader = temp_shader;
        self.cast_shadows = temp_shadows;
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
        let temp_albedo = self.current_albedo.clone();
        let temp_shader = self.current_shader.clone();
        let temp_shadows = self.cast_shadows;
        self.current_albedo = Albedo::Texture(texture.clone());
        self.current_shader = self.builtins.unshaded_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.quad_mesh.clone();
        self.draw(&mesh, transform);

        self.current_albedo = temp_albedo;
        self.current_shader = temp_shader;
        self.cast_shadows = temp_shadows;
    }

    pub fn draw_surface(&mut self) {
        let temp_shadows = self.cast_shadows;
        self.cast_shadows = false;

        let mesh = self.builtins.surface_mesh.clone();
        self.draw(&mesh, [0.0, 0.0, 0.0]);

        self.cast_shadows = temp_shadows;
    }

    pub fn blit_framebuffer(&mut self, framebuffer: &Ref<Framebuffer>) {
        let temp_shader = self.current_shader.clone();
        let temp_albedo = self.current_albedo.clone();
        self.current_shader = self.builtins.blit_shader.clone();
        self.current_albedo = Albedo::Framebuffer(framebuffer.clone());

        self.draw_surface();

        self.current_shader = temp_shader;
        self.current_albedo = temp_albedo;
    }

    pub fn draw_grid(&mut self) {
        let temp_shader = self.current_shader.clone();
        let temp_shadows = self.cast_shadows;
        self.current_shader = self.builtins.line_shader.clone();
        self.cast_shadows = false;

        let mesh = self.builtins.grid_mesh.clone();
        self.draw(&mesh, [0.0, 0.0, 0.0]);

        self.current_shader = temp_shader;
        self.cast_shadows = temp_shadows;
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let (shader, sampler_index) = if self
            .current_font
            .with(|f| f.is_bitmap(self.current_font_size))
        {
            (self.builtins.bitmap_font_shader.clone(), 7)
        } else {
            (self.builtins.sdf_font_shader.clone(), 1)
        };

        self.text_orders.push(TextOrder {
            font: self.current_font.clone(),
            size: self.current_font_size,
            text: text.as_ref().to_string(),
            transform: transform.into(),
            material: self.current_font_material.clone(),
            sampler_index,
            shader,
        });
    }

    pub fn add_directional_light(
        &mut self,
        direction: impl Into<Vector3>,
        color: impl Into<Color>,
    ) {
        self.lights.push(Light {
            coords: direction.into().unit().extend(0.0),
            color: color.into().to_rgba_norm_vec(),
        });
    }

    pub fn add_point_light(&mut self, direction: impl Into<Vector3>, color: impl Into<Color>) {
        self.lights.push(Light {
            coords: direction.into().unit().extend(1.0),
            color: color.into().to_rgba_norm_vec(),
        });
    }

    pub fn set_main_light(&mut self, direction: impl Into<Vector3>, color: impl Into<Color>) {
        self.main_light = Light {
            coords: direction.into().unit().extend(0.0),
            color: color.into().to_rgba_norm_vec(),
        };
    }

    pub fn set_clear(&mut self, clear: impl Into<Color>) {
        self.clear = clear.into();
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

    pub fn set_wireframes(&mut self, enable: bool) {
        self.wireframes = enable;
    }

    pub fn set_sampler(&mut self, options: SamplerOptions) {
        let mut index = 0;
        if options.filter == SamplerFilter::Nearest {
            index += 4;
        }
        if options.address == SamplerAddress::Clamp {
            index += 2;
        }
        if options.mipmaps == SamplerMipmaps::Disabled {
            index += 1;
        }
        self.current_sampler = index;
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub fn set_font_size(&mut self, size: u32) {
        self.current_font_size = size;
    }

    pub fn set_cascade_splits(&mut self, splits: [f32; 4]) {
        self.cascade_splits = splits;
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

    pub(crate) fn lights(&self) -> [Light; 4] {
        let mut lights: [Light; 4] = Default::default();
        lights[0] = self.main_light;
        lights[1..self.lights.len() + 1].clone_from_slice(&self.lights[..]);
        lights
    }

    fn add_order(&mut self, order: Order) {
        let material = self.current_material.clone();
        let shader = self.current_shader.clone();

        if self.cast_shadows {
            self.do_shadow_mapping = true;
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
