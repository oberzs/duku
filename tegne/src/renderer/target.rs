// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Target - struct that collects draw calls to be used in a renderer

use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::Mesh;
use crate::pipeline::Light;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::resource::Id;
use crate::resource::IdRef;
use crate::resource::ResourceManager;

pub struct Target<'a> {
    orders_by_shader: Vec<OrdersByShader>,
    wireframe_orders: Vec<Order>,
    clear: [f32; 4],
    lights: Vec<Light>,
    current_shader: IdRef,
    current_material: IdRef,
    current_albedo: IdRef,
    current_font: IdRef,
    draw_wireframes: bool,
    resources: &'a ResourceManager,
}

pub(crate) struct OrdersByShader {
    shader: IdRef,
    orders_by_material: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    material: IdRef,
    orders: Vec<Order>,
}

#[derive(Copy, Clone)]
pub(crate) struct Order {
    pub(crate) mesh: IdRef,
    pub(crate) albedo: IdRef,
    pub(crate) model: Matrix4,
    pub(crate) has_shadows: bool,
}

impl<'a> Target<'a> {
    pub(crate) fn new(resources: &'a ResourceManager) -> Result<Self> {
        Ok(Self {
            orders_by_shader: vec![],
            wireframe_orders: vec![],
            clear: [0.7, 0.7, 0.7, 1.0],
            lights: vec![],
            current_shader: resources.builtin("phong_sh"),
            current_material: resources.builtin("white_mat"),
            current_albedo: resources.builtin("white_tex"),
            current_font: resources.builtin("roboto_font"),
            draw_wireframes: false,
            resources,
        })
    }

    pub fn draw(&mut self, mesh: &Id<Mesh>, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: mesh.id_ref(),
            albedo: self.current_albedo,
            model: transform.into().as_matrix(),
            has_shadows: true,
        });
    }

    pub fn draw_cube(&mut self, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: self.resources.builtin("cube_mesh"),
            albedo: self.current_albedo,
            model: transform.into().as_matrix(),
            has_shadows: true,
        });
    }

    pub fn draw_sphere(&mut self, transform: impl Into<Transform>) {
        self.add_order(Order {
            mesh: self.resources.builtin("sphere_mesh"),
            albedo: self.current_albedo,
            model: transform.into().as_matrix(),
            has_shadows: true,
        });
    }

    pub fn draw_surface(&mut self) {
        self.add_order(Order {
            mesh: self.resources.builtin("surface_mesh"),
            albedo: self.current_albedo,
            model: Transform::from([0.0, 0.0, 0.0]).as_matrix(),
            has_shadows: false,
        });
    }

    pub fn blit_framebuffer(&mut self, framebuffer: &Id<Framebuffer>) {
        let temp_shader = self.current_shader;
        let temp_albedo = self.current_albedo;
        self.current_shader = self.resources.builtin("passthru_sh");
        self.current_albedo = framebuffer.id_ref();

        self.draw_surface();

        self.current_shader = temp_shader;
        self.current_albedo = temp_albedo;
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, transform: impl Into<Transform>) {
        let temp_shader = self.current_shader;
        self.current_shader = self.resources.builtin("font_sh");
        let text_str = text.as_ref();

        self.resources.with_font(self.current_font, |font| {
            let mut current_transform = transform.into();
            let x_scale = current_transform.scale.x;
            current_transform.position.x -=
                font.char_bearing(text_str.chars().next().unwrap()) * x_scale;
            let albedo = font.texture();

            for c in text_str.chars() {
                if c == ' ' {
                    let space_advance = font.char_advance('_');
                    current_transform.position.x += space_advance * x_scale;
                    continue;
                }

                let mesh = font.char_mesh(c);
                self.add_order(Order {
                    mesh,
                    albedo,
                    model: current_transform.as_matrix(),
                    has_shadows: false,
                });

                current_transform.position.x += font.char_advance(c) * x_scale;
            }
        });

        self.current_shader = temp_shader;
    }

    #[cfg(feature = "ui")]
    pub fn draw_ui(&mut self, draw_data: &imgui::DrawData) {
        // println!("display pos: {:?}", draw_data.display_pos);
        // println!("display size: {:?}", draw_data.display_size);
        // generate mesh data
        let mut triangles = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        for draw_list in draw_data.draw_lists() {
            for tri in draw_list.idx_buffer().chunks(3) {
                triangles.push([tri[0] as u32, tri[2] as u32, tri[1] as u32]);
            }
            for vert in draw_list.vtx_buffer() {
                let vertex = Vector3::new(vert.pos[0], vert.pos[1], 1.0);
                let uv = Vector2::new(vert.uv[0], vert.uv[1]);
                let color = Vector4::new(
                    vert.col[0] as f32 / 255.0,
                    vert.col[1] as f32 / 255.0,
                    vert.col[2] as f32 / 255.0,
                    vert.col[3] as f32 / 255.0,
                );
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::backward());
            }
        }

        // update mesh
        let mesh = self.resources.builtin("ui_mesh");
        self.resources.with_mesh(mesh, |m| {
            m.set_vertices(&vertices);
            m.set_normals(&normals);
            m.set_colors(&colors);
            m.set_uvs(&uvs);
            m.set_triangles(&triangles);
        });

        // draw mesh
        let temp_shader = self.current_shader;
        self.current_shader = self.resources.builtin("ui_sh");

        self.add_order(Order {
            mesh,
            albedo: self.resources.builtin("ui_tex"),
            model: Transform::from([0.0, 0.0, 0.0]).as_matrix(),
            has_shadows: false,
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

    pub fn set_material(&mut self, material: &Id<Material>) {
        self.current_material = material.id_ref();
    }

    pub fn set_material_white(&mut self) {
        self.current_material = self.resources.builtin("white_mat");
    }

    pub fn set_albedo_texture(&mut self, texture: &Id<Texture>) {
        self.current_albedo = texture.id_ref();
    }

    pub fn set_albedo_framebuffer(&mut self, framebuffer: &Id<Framebuffer>) {
        self.current_albedo = framebuffer.id_ref();
    }

    pub fn set_shader(&mut self, shader: &Id<Shader>) {
        self.current_shader = shader.id_ref();
    }

    pub fn set_shader_phong(&mut self) {
        self.current_shader = self.resources.builtin("phong_sh");
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
    pub(crate) fn shader(&self) -> IdRef {
        self.shader
    }

    pub(crate) fn orders_by_material(&self) -> impl Iterator<Item = &OrdersByMaterial> {
        self.orders_by_material.iter()
    }
}

impl OrdersByMaterial {
    pub(crate) fn material(&self) -> IdRef {
        self.material
    }

    pub(crate) fn orders(&self) -> impl Iterator<Item = Order> + '_ {
        self.orders.iter().cloned()
    }
}
