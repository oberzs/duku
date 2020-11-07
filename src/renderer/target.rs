// Oliver Berzs
// https://github.com/oberzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use std::f32::consts::PI;

use super::Pcf;
use crate::color::Color;
use crate::font::Font;
use crate::image::Cubemap;
use crate::image::Texture;
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
    pub skybox: Option<&'a Handle<Cubemap>>,
    pub transform: Transform,
    pub(crate) builtins: &'b Builtins,

    // meshes
    pub shader: Option<&'a Handle<Shader>>,
    pub material: Option<&'a Handle<Material>>,
    pub(crate) mesh_orders: Vec<OrdersByShader>,

    // shadows & lights
    pub shadow_split_coef: f32,
    pub shadow_depth: f32,
    pub shadow_pcf: Pcf,
    pub shadows: bool,
    pub lights: [Light; 4],
    pub ambient_color: Color,

    // lines
    pub line_color: Color,
    pub line_width: f32,
    pub(crate) line_orders: Vec<LineOrder>,

    // shapes
    pub shape_color: Color,
    pub shape_mode: ShapeMode,
    pub border_color: Color,
    pub border_mode: BorderMode,
    pub border_width: f32,
    pub(crate) shape_orders: Vec<ShapeOrder>,

    // text
    pub font_size: u32,
    pub font: Option<&'a Handle<Font>>,
    pub text_color: Color,
    pub(crate) text_orders: Vec<TextOrder>,

    // textures
    pub texture_filter: TextureFilter,
    pub texture_wrap: TextureWrap,
    pub texture_mipmaps: bool,

    cache: Vec<Cache<'a>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShapeMode {
    Corner,
    Center,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BorderMode {
    Outside,
    Inside,
    Center,
    Disabled,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Handle<Shader>,
    pub(crate) orders: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Handle<Material>,
    pub(crate) orders: Vec<MeshOrder>,
}

pub(crate) struct MeshOrder {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) local_to_world: Matrix4,
    pub(crate) shadows: bool,
    pub(crate) sampler_index: u32,
}

pub(crate) struct TextOrder {
    pub(crate) size: u32,
    pub(crate) color: Color,
    pub(crate) font: Handle<Font>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
}

pub(crate) struct LineOrder {
    pub(crate) color: Color,
    pub(crate) points: [Vector3; 2],
    pub(crate) transform: Transform,
}

pub(crate) struct ShapeOrder {
    pub(crate) color: Color,
    pub(crate) points: [Vector3; 3],
    pub(crate) transform: Transform,
    pub(crate) texture: Handle<Texture>,
    pub(crate) uvs: [Vector2; 3],
    pub(crate) sampler_index: u32,
}

struct Cache<'a> {
    transform: Transform,
    line_color: Color,
    shape_color: Color,
    text_color: Color,
    border_color: Color,
    font_size: u32,
    border_width: f32,
    line_width: f32,
    shape_mode: ShapeMode,
    border_mode: BorderMode,
    shader: Option<&'a Handle<Shader>>,
    material: Option<&'a Handle<Material>>,
}

impl<'b> Target<'_, 'b> {
    pub(crate) fn new(builtins: &'b Builtins) -> Self {
        Self {
            mesh_orders: vec![],
            text_orders: vec![],
            line_orders: vec![],
            shape_orders: vec![],
            cache: vec![],
            shape_mode: ShapeMode::Corner,
            clear_color: Color::BLACK,
            text_color: Color::WHITE,
            line_color: Color::WHITE,
            shape_color: Color::WHITE,
            border_color: Color::GRAY,
            ambient_color: Color::WHITE,
            transform: Transform::default(),
            lights: [
                Light::main([-1.0, -1.0, 1.0], Color::WHITE, 10.0),
                Light::NONE,
                Light::NONE,
                Light::NONE,
            ],
            texture_filter: TextureFilter::Linear,
            texture_wrap: TextureWrap::Repeat,
            border_width: 1.0,
            line_width: 1.0,
            border_mode: BorderMode::Inside,
            font_size: 24,
            font: None,
            shader: None,
            material: None,
            texture_mipmaps: true,
            skybox: None,
            shadow_depth: 50.0,
            shadow_split_coef: 0.5,
            shadows: true,
            shadow_pcf: Pcf::X16,
            builtins,
        }
    }

    pub fn push(&mut self) {
        self.cache.push(Cache {
            transform: self.transform,
            line_color: self.line_color,
            shape_color: self.shape_color,
            text_color: self.text_color,
            font_size: self.font_size,
            border_color: self.border_color,
            line_width: self.line_width,
            border_mode: self.border_mode,
            shape_mode: self.shape_mode,
            border_width: self.border_width,
            shader: self.shader,
            material: self.material,
        });
    }

    pub fn pop(&mut self) {
        if let Some(cache) = self.cache.pop() {
            self.transform = cache.transform;
            self.line_color = cache.line_color;
            self.shape_color = cache.shape_color;
            self.text_color = cache.text_color;
            self.font_size = cache.font_size;
            self.border_color = cache.border_color;
            self.line_width = cache.line_width;
            self.border_mode = cache.border_mode;
            self.shape_mode = cache.shape_mode;
            self.border_width = cache.border_width;
            self.shader = cache.shader;
            self.material = cache.material;
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Handle<Mesh>) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: mesh.clone(),
                local_to_world: Matrix4::from(self.transform),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_mesh_wireframe(&mut self, mesh: &Handle<Mesh>) {
        let shader = self.builtins.wireframe_shader.clone();
        let material = self.builtins.white_material.clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: mesh.clone(),
                local_to_world: Matrix4::from(self.transform),
                shadows: false,
                sampler_index: 0,
            },
        );
    }

    pub fn draw_cube(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.cube_mesh.clone(),
                local_to_world: Matrix4::from(self.transform),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_sphere_ico(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.ico_sphere_mesh.clone(),
                local_to_world: Matrix4::from(self.transform),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_sphere_uv(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.uv_sphere_mesh.clone(),
                local_to_world: Matrix4::from(self.transform),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_surface(&mut self) {
        let shader = self
            .shader
            .unwrap_or(&self.builtins.unshaded_shader)
            .clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.surface_mesh.clone(),
                local_to_world: Matrix4::identity(),
                shadows: false,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_fullscreen(&mut self) {
        let shader = self.builtins.fullscreen_shader.clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.surface_mesh.clone(),
                local_to_world: Matrix4::identity(),
                shadows: false,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_grid(&mut self) {
        let size = 100;
        let half = size / 2;
        let width = 1.0;

        self.push();

        for x in -half..half {
            let xx = x as f32 * width;
            let z_min = -half as f32 * width;
            let z_max = half as f32 * width;

            // set color
            self.line_color = match x {
                0 => Color::rgba(0, 0, 255, 150),
                _ if x % 10 == 0 => Color::rgba(255, 255, 255, 150),
                _ => Color::rgba(255, 255, 255, 50),
            };

            self.draw_line_debug([xx, 0.0, z_min], [xx, 0.0, z_max]);
        }

        for z in -half..half {
            let zz = z as f32 * width;
            let x_min = -half as f32 * width;
            let x_max = half as f32 * width;

            // set color
            self.line_color = match z {
                0 => Color::rgba(255, 0, 0, 150),
                _ if z % 10 == 0 => Color::rgba(255, 255, 255, 150),
                _ => Color::rgba(255, 255, 255, 50),
            };

            self.draw_line_debug([x_min, 0.0, zz], [x_max, 0.0, zz]);
        }

        self.pop();
    }

    pub fn draw_text(&mut self, text: impl AsRef<str>, position: impl Into<Vector2>) {
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

    pub fn draw_line_debug(&mut self, point_1: impl Into<Vector3>, point_2: impl Into<Vector3>) {
        self.line_orders.push(LineOrder {
            color: self.line_color,
            points: [point_1.into(), point_2.into()],
            transform: self.transform,
        });
    }

    pub fn draw_shape(&mut self, points: &[Vector2]) {
        // don't draw polygon with less than 2 points
        if points.len() < 3 {
            return;
        }

        // triangulate points
        let first = points[0].extend(0.0);
        for i in 2..points.len() {
            self.shape_orders.push(ShapeOrder {
                color: self.shape_color,
                points: [first, points[i - 1].extend(0.0), points[i].extend(0.0)],
                transform: self.transform,
                texture: self.builtins.white_texture.clone(),
                uvs: [Vector2::ZERO; 3],
                sampler_index: 0,
            });
        }

        // draw borders
        if self.border_mode != BorderMode::Disabled {
            self.push();
            self.transform.move_backward(0.00001);
            self.draw_path(
                points,
                true,
                self.border_mode,
                self.border_color,
                self.border_width,
            );
            self.pop();
        }
    }

    pub fn draw_lines(&mut self, points: &[Vector2], closed: bool) {
        self.draw_path(
            points,
            closed,
            BorderMode::Center,
            self.line_color,
            self.line_width,
        );
    }

    pub fn draw_rectangle(&mut self, position: impl Into<Vector2>, size: impl Into<Vector2>) {
        let pos = position.into();
        let s = size.into();

        self.push();

        if self.shape_mode == ShapeMode::Center {
            self.transform.position -= (s / 2.0).extend(0.0);
        }

        self.draw_shape(&[
            Vector2::new(pos.x, pos.y + s.y),
            pos + s,
            Vector2::new(pos.x + s.x, pos.y),
            pos,
        ]);

        self.pop();
    }

    pub fn draw_square(&mut self, position: impl Into<Vector2>, size: f32) {
        self.draw_rectangle(position, Vector2::new(size, size));
    }

    pub fn draw_ellipse(&mut self, position: impl Into<Vector2>, size: impl Into<Vector2>) {
        let pos = position.into();
        let s = size.into() / 2.0;
        let resolution = 50;

        self.push();

        if self.shape_mode == ShapeMode::Corner {
            self.transform.position += s.extend(0.0);
        }

        let points: Vec<_> = (0..resolution)
            .map(|i| {
                let q = 2.0 * PI * (i as f32 / resolution as f32);
                let x = s.x * q.cos();
                let y = s.y * q.sin();
                pos + Vector2::new(x, y)
            })
            .collect();
        self.draw_shape(&points);

        self.pop();
    }

    pub fn draw_circle(&mut self, position: impl Into<Vector2>, size: f32) {
        self.draw_ellipse(position, Vector2::new(size, size));
    }

    pub fn draw_texture(
        &mut self,
        texture: &Handle<Texture>,
        position: impl Into<Vector2>,
        size: impl Into<Vector2>,
    ) {
        let pos = position.into().extend(0.0);
        let s = size.into().extend(0.0);

        self.push();

        if self.shape_mode == ShapeMode::Center {
            self.transform.position -= s / 2.0;
        }

        self.shape_orders.push(ShapeOrder {
            color: self.shape_color,
            points: [
                pos,
                Vector3::new(pos.x, pos.y + s.y, 0.0),
                Vector3::new(pos.x + s.x, pos.y, 0.0),
            ],
            transform: self.transform,
            texture: texture.clone(),
            uvs: [
                Vector2::new(0.0, 1.0),
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 1.0),
            ],
            sampler_index: self.sampler_index(),
        });
        self.shape_orders.push(ShapeOrder {
            color: self.shape_color,
            points: [
                Vector3::new(pos.x, pos.y + s.y, 0.0),
                pos + s,
                Vector3::new(pos.x + s.x, pos.y, 0.0),
            ],
            transform: self.transform,
            texture: texture.clone(),
            uvs: [
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, 1.0),
            ],
            sampler_index: self.sampler_index(),
        });

        self.pop();
    }

    fn add_mesh_order(
        &mut self,
        material: Handle<Material>,
        shader: Handle<Shader>,
        order: MeshOrder,
    ) {
        match self.mesh_orders.iter_mut().find(|so| so.shader == shader) {
            Some(so) => match so.orders.iter_mut().find(|mo| mo.material == material) {
                Some(mo) => mo.orders.push(order),
                None => so.orders.push(OrdersByMaterial {
                    material,
                    orders: vec![order],
                }),
            },
            None => self.mesh_orders.push(OrdersByShader {
                shader,
                orders: vec![OrdersByMaterial {
                    material,
                    orders: vec![order],
                }],
            }),
        }
    }

    fn draw_path(
        &mut self,
        path: &[Vector2],
        closed: bool,
        border_mode: BorderMode,
        color: Color,
        width: f32,
    ) {
        // generate normals
        let mut normals = vec![];

        let mut points = path.to_vec();
        if closed {
            points.push(path[0]);
        }

        for i in 1..points.len() {
            let prev = points[i - 1];
            let curr = points[i];
            let next = points.get(i + 1);

            let line_a = (curr - prev).unit();

            if i == 1 {
                // first segment
                normals.push(line_a.normal());
            }

            if let Some(n) = next {
                let line_b = (*n - curr).unit();
                normals.push(miter(line_a, line_b));
            } else {
                // last segment
                normals.push(line_a.normal());
            }
        }

        if points.len() > 2 && closed {
            // connect first and last normals
            let prev = points[points.len() - 2];
            let curr = points[0];
            let next = points[1];

            let line_a = (curr - prev).unit();
            let line_b = (next - curr).unit();

            let m = miter(line_a, line_b);
            normals[0] = m;
            normals[points.len() - 1] = m;
        }

        // render lines
        self.push();
        self.shape_color = color;
        self.border_mode = BorderMode::Disabled;
        for i in 0..(normals.len() - 1) {
            let curr_norm = normals[i];
            let next_norm = normals[i + 1];
            let curr_point = points[i];
            let next_point = *points.get(i + 1).unwrap_or(&points[0]);

            match border_mode {
                BorderMode::Center => self.draw_shape(&[
                    curr_point + curr_norm * width * 0.5,
                    next_point + next_norm * width * 0.5,
                    next_point - next_norm * width * 0.5,
                    curr_point - curr_norm * width * 0.5,
                ]),
                BorderMode::Outside => self.draw_shape(&[
                    curr_point + curr_norm * width,
                    next_point + next_norm * width,
                    next_point,
                    curr_point,
                ]),
                BorderMode::Inside => self.draw_shape(&[
                    curr_point,
                    next_point,
                    next_point - next_norm * width,
                    curr_point - curr_norm * width,
                ]),
                BorderMode::Disabled => (),
            }
        }
        self.pop();
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

fn miter(line_a: Vector2, line_b: Vector2) -> Vector2 {
    let tangent = (line_a + line_b).unit();
    let miter = tangent.normal();
    let norm_a = line_a.normal();
    let miter_len = 1.0 / miter.dot(norm_a);

    miter * miter_len
}
