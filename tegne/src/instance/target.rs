use tegne_math::Matrix4;
use tegne_math::Transform;
use tegne_math::Vector3;

use crate::error::Result;
use crate::images::Font;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::objects::Builtins;
use crate::objects::Id;
use crate::objects::Objects;
use crate::shaders::Light;
use crate::shaders::Material;
use crate::shaders::Shader;

pub struct Target<'a> {
    orders_by_shader: Vec<OrdersByShader>,
    wireframe_orders: Vec<Order>,
    clear: [f32; 4],
    lights: Vec<Light>,
    current_shader: Id<Shader>,
    current_material: Id<Material>,
    current_albedo: Albedo,
    current_font: Id<Font>,
    draw_wireframes: bool,
    builtins: &'a Builtins,
    objects: &'a Objects,
}

#[derive(Copy, Clone)]
pub(crate) enum Albedo {
    Texture(Id<Texture>),
    Framebuffer(Id<Framebuffer>),
}

pub(crate) struct OrdersByShader {
    shader: Id<Shader>,
    orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    material: Id<Material>,
    orders: Vec<Order>,
}

#[derive(Copy, Clone)]
pub(crate) struct Order {
    pub(crate) mesh: Id<Mesh>,
    pub(crate) albedo: Albedo,
    pub(crate) model: Matrix4,
    pub(crate) has_shadows: bool,
}

impl<'a> Target<'a> {
    pub(crate) fn new(builtins: &'a Builtins, objects: &'a Objects) -> Result<Self> {
        Ok(Self {
            orders_by_shader: vec![],
            wireframe_orders: vec![],
            clear: [0.7, 0.7, 0.7, 1.0],
            lights: vec![],
            current_shader: builtins.shaders.phong,
            current_material: builtins.materials.white,
            current_albedo: Albedo::Texture(builtins.textures.white),
            current_font: builtins.fonts.roboto_mono,
            draw_wireframes: false,
            builtins,
            objects,
        })
    }

    pub fn draw(&mut self, mesh: Id<Mesh>, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh,
            albedo: self.current_albedo,
            model: transform.into().as_matrix(),
            has_shadows: true,
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.meshes.cube, transform);
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.draw(self.builtins.meshes.sphere, transform);
    }

    pub fn draw_surface(&mut self) {
        self.draw(self.builtins.meshes.surface, [0.0, 0.0, 0.0]);
    }

    pub fn blit_framebuffer(&mut self, framebuffer: Id<Framebuffer>) {
        let temp_shader = self.current_shader;
        let temp_albedo = self.current_albedo;
        self.current_shader = self.builtins.shaders.passthru;
        self.current_albedo = Albedo::Framebuffer(framebuffer);

        self.draw(self.builtins.meshes.surface, [0.0, 0.0, 0.0]);

        self.current_shader = temp_shader;
        self.current_albedo = temp_albedo;
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let temp_shader = self.current_shader;
        self.current_shader = self.builtins.shaders.font;

        self.objects.with_font(self.current_font, |font| {
            let mut current_transform = transform.into();
            let x_scale = current_transform.scale.x;
            let albedo = font.texture();

            for c in text.as_ref().chars() {
                if c == ' ' {
                    let space_advance = font.char_advance('_');
                    current_transform.position.x += space_advance * x_scale;
                    continue;
                }

                let mesh = font.char_mesh(c);
                self.add_order(Order {
                    mesh,
                    albedo: Albedo::Texture(albedo),
                    model: current_transform.as_matrix(),
                    has_shadows: false,
                });

                current_transform.position.x += font.char_advance(c) * x_scale;
            }
        });

        self.current_shader = temp_shader;
    }

    pub fn add_directional_light(
        &mut self,
        direction: impl Into<Vector3>,
        color: impl Into<Vector3>,
    ) {
        self.lights.push(Light {
            coords: direction.into().extend(0.0),
            color: color.into().extend(1.0),
        });
    }

    pub fn set_material(&mut self, material: Id<Material>) {
        self.current_material = material;
    }

    pub fn set_material_white(&mut self) {
        self.current_material = self.builtins.materials.white;
    }

    pub fn set_albedo_texture(&mut self, texture: Id<Texture>) {
        self.current_albedo = Albedo::Texture(texture);
    }

    pub fn set_albedo_framebuffer(&mut self, framebuffer: Id<Framebuffer>) {
        self.current_albedo = Albedo::Framebuffer(framebuffer);
    }

    pub fn set_shader(&mut self, shader: Id<Shader>) {
        self.current_shader = shader;
    }

    pub fn set_shader_phong(&mut self) {
        self.current_shader = self.builtins.shaders.phong;
    }

    pub fn set_clear_color(&mut self, clear: [f32; 4]) {
        self.clear = clear;
    }

    pub fn set_wireframes(&mut self, draw: bool) {
        self.draw_wireframes = draw;
    }

    pub(crate) fn clear(&self) -> [f32; 4] {
        [self.clear[0], self.clear[1], self.clear[2], self.clear[3]]
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
    pub(crate) fn shader(&self) -> Id<Shader> {
        self.shader
    }

    pub(crate) fn orders_by_material(&self) -> impl Iterator<Item = &OrdersByMaterial> {
        self.orders_by_material.iter()
    }
}

impl OrdersByMaterial {
    pub(crate) fn material(&self) -> Id<Material> {
        self.material
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = Order> + '_ {
        self.orders.iter().cloned()
    }
}
