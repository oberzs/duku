// Oliver Berzs
// https://github.com/oberzs/duku

use std::f32::consts::PI;

use super::Light;
use crate::color::Rgbf;
use crate::font::Font;
use crate::image::Canvas;
use crate::image::Cubemap;
use crate::image::Filter;
use crate::image::Texture;
use crate::image::Wrap;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::Model;
use crate::mesh::ModelNode;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::resources::Builtins;
use crate::resources::Handle;

/// Active render target.
///
/// Records rendering commands and settings.
/// Received from the [draw](crate::duku::Duku::draw) or
/// [draw_on_canvas](crate::duku::Duku::draw_on_canvas)
/// functions.
pub struct Target {
    // global
    /// the clear color of the canvas (background)
    pub clear_color: Rgbf,
    /// the current transform used when
    /// doing a render command
    pub transform: Transform,
    pub(crate) skybox: Option<Handle<Cubemap>>,
    pub(crate) builtins: Builtins,

    // meshes
    pub(crate) shader: Option<Handle<Shader>>,
    pub(crate) material: Option<Handle<Material>>,
    pub(crate) mesh_orders: Vec<ShaderOrder>,

    // shadows & lights
    /// coefficient used to calculate shadow map splits (0 - 1).
    /// use smaller number to achieve better shadow detail up close.
    pub shadow_split_coef: f32,
    /// maximum shadow distance in the scene.
    /// the smaller the value, the better the shadow quality.
    pub shadow_depth: f32,
    /// setting for shadow softening
    pub shadow_pcf: Pcf,
    /// controlls whether to use the PBR shader
    pub shadows: bool,
    /// the lights used in the scene
    pub lights: [Light; 4],
    /// maximum white value for HDR tone mapping
    pub max_white_point: f32,
    /// the ambient color of the scene
    pub ambient_color: Rgbf,

    // lines
    /// color used for lines
    pub line_color: Rgbf,
    /// width used for non-debug lines
    pub line_width: f32,
    pub(crate) line_orders: Vec<LineOrder>,

    // shapes
    /// color used for shapes
    pub shape_color: Rgbf,
    /// shape positioning mode
    pub shape_mode: ShapeMode,
    /// color used for shape borders
    pub border_color: Rgbf,
    /// border positioning mode
    pub border_mode: BorderMode,
    /// width used for shape borders
    pub border_width: f32,
    pub(crate) shape_orders: Vec<ShapeOrder>,

    // text
    /// font size used for text
    pub font_size: u32,
    /// color used for text
    pub text_color: Rgbf,
    pub(crate) font: Option<Handle<Font>>,
    pub(crate) text_orders: Vec<TextOrder>,

    // textures
    /// filter used for texture sampling
    pub texture_filter: Filter,
    /// wrap mode used for texture sampling
    pub texture_wrap: Wrap,
    /// whether to use mipmaps in texture sampling
    pub texture_mipmaps: bool,

    cache: Vec<Cache>,
}

/// Shape positioning mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShapeMode {
    /// position from bottom-left corner
    Corner,
    /// position from center
    Center,
}

/// Border positioning mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BorderMode {
    /// put border on the outside of shape
    Outside,
    /// put border on the inside of shape
    Inside,
    /// pub border evenly on the inside
    /// and outside of shape
    Center,
    /// disable borders for shapes
    Disabled,
}

/// Shadow softening used when sampling.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pcf {
    /// sample shadow map 16 times
    X16,
    /// sample shadow map 4 times
    X4,
    /// sample shadow map 1 time
    Disabled,
}

pub(crate) struct ShaderOrder {
    pub(crate) shader: Handle<Shader>,
    pub(crate) orders: Vec<MaterialOrder>,
}

pub(crate) struct MaterialOrder {
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
    pub(crate) color: Rgbf,
    pub(crate) font: Handle<Font>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
}

pub(crate) struct LineOrder {
    pub(crate) color: Rgbf,
    pub(crate) points: [Vector3; 2],
    pub(crate) transform: Transform,
}

pub(crate) struct ShapeOrder {
    pub(crate) color: Rgbf,
    pub(crate) points: [Vector3; 3],
    pub(crate) transform: Transform,
    pub(crate) texture: Handle<Texture>,
    pub(crate) uvs: [Vector2; 3],
    pub(crate) sampler_index: u32,
    pub(crate) opaque: bool,
}

struct Cache {
    transform: Transform,
    line_color: Rgbf,
    shape_color: Rgbf,
    text_color: Rgbf,
    border_color: Rgbf,
    font_size: u32,
    border_width: f32,
    line_width: f32,
    shape_mode: ShapeMode,
    border_mode: BorderMode,
    shader: Option<Handle<Shader>>,
    material: Option<Handle<Material>>,
}

impl Target {
    pub(crate) fn new(builtins: &Builtins) -> Self {
        Self {
            mesh_orders: vec![],
            text_orders: vec![],
            line_orders: vec![],
            shape_orders: vec![],
            cache: vec![],
            shape_mode: ShapeMode::Corner,
            clear_color: Rgbf::gray(0.0),
            text_color: Rgbf::gray(1.0),
            line_color: Rgbf::gray(1.0),
            shape_color: Rgbf::gray(1.0),
            border_color: Rgbf::gray(0.5),
            ambient_color: Rgbf::gray(1.0),
            transform: Transform::default(),
            lights: [
                Light::main([-1.0, -1.0, 1.0], Rgbf::gray(1.0), 1.0),
                Light::point([0.0, 0.0, 0.0], Rgbf::gray(1.0), 0.0),
                Light::point([0.0, 0.0, 0.0], Rgbf::gray(1.0), 0.0),
                Light::point([0.0, 0.0, 0.0], Rgbf::gray(1.0), 0.0),
            ],
            texture_filter: Filter::Linear,
            texture_wrap: Wrap::Repeat,
            max_white_point: 1.0,
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
            builtins: builtins.clone(),
        }
    }

    /// Save target settings to stack
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
            shader: self.shader.clone(),
            material: self.material.clone(),
        });
    }

    /// Restore target settings from stack
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

    /// Set currently used material for mesh rendering
    pub fn set_material(&mut self, material: &Handle<Material>) {
        self.material = Some(material.clone());
    }

    /// Set material to default for mesh rendering
    pub fn unset_material(&mut self) {
        self.material = None;
    }

    /// Set currently used shader for mesh rendering
    pub fn set_shader(&mut self, shader: &Handle<Shader>) {
        self.shader = Some(shader.clone());
    }

    /// Set shader to default for mesh rendering
    pub fn unset_shader(&mut self) {
        self.shader = None;
    }

    /// Set cubemap to be used as a skybox
    pub fn set_skybox(&mut self, cubemap: &Handle<Cubemap>) {
        self.skybox = Some(cubemap.clone());
    }

    /// Draw custom mesh
    pub fn draw_mesh(&mut self, mesh: &Handle<Mesh>) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.as_ref().unwrap_or(default_shader).clone();
        let material = self
            .material
            .as_ref()
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

    /// Draw wireframes for custom mesh
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

    /// Draw a cube
    pub fn draw_cube(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.as_ref().unwrap_or(default_shader).clone();
        let material = self
            .material
            .as_ref()
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

    /// Draw an ico-sphere
    pub fn draw_sphere_ico(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.as_ref().unwrap_or(default_shader).clone();
        let material = self
            .material
            .as_ref()
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

    /// Draw a uv-sphere
    pub fn draw_sphere_uv(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.pbr_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.as_ref().unwrap_or(default_shader).clone();
        let material = self
            .material
            .as_ref()
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

    /// Draw a flat surface
    pub fn draw_surface(&mut self) {
        let shader = self
            .shader
            .as_ref()
            .unwrap_or(&self.builtins.unshaded_shader)
            .clone();
        let material = self
            .material
            .as_ref()
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

    /// Draw a fullscreen quad
    pub fn draw_fullscreen(&mut self, canvas: &Handle<Canvas>) {
        let shader = self.builtins.fullscreen_shader.clone();
        let material = canvas.material().clone();

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

    /// Draw a XY line grid
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
                0 => Rgbf::blue(1.0).alpha(0.7),
                _ if x % 10 == 0 => Rgbf::gray(1.0).alpha(0.7),
                _ => Rgbf::gray(1.0).alpha(0.4),
            };

            self.draw_line_debug([xx, 0.0, z_min], [xx, 0.0, z_max]);
        }

        for z in -half..half {
            let zz = z as f32 * width;
            let x_min = -half as f32 * width;
            let x_max = half as f32 * width;

            // set color
            self.line_color = match z {
                0 => Rgbf::red(1.0).alpha(0.7),
                _ if z % 10 == 0 => Rgbf::gray(1.0).alpha(0.7),
                _ => Rgbf::gray(1.0).alpha(0.4),
            };

            self.draw_line_debug([x_min, 0.0, zz], [x_max, 0.0, zz]);
        }

        self.pop();
    }

    /// Draw a string of text
    pub fn draw_text(&mut self, text: impl AsRef<str>) {
        let font = self
            .font
            .as_ref()
            .unwrap_or(&self.builtins.fira_font)
            .clone();

        self.text_orders.push(TextOrder {
            size: self.font_size,
            color: self.text_color,
            text: text.as_ref().to_string(),
            transform: self.transform,
            font,
        });
    }

    /// Move transform down one line's heigth for the current font
    pub fn new_line(&mut self) {
        let line_height = self
            .font
            .as_ref()
            .unwrap_or(&self.builtins.fira_font)
            .line_height();
        self.transform.move_down(line_height as f32);
    }

    /// Draw a debug 1-pixel wide line
    pub fn draw_line_debug(&mut self, point_1: impl Into<Vector3>, point_2: impl Into<Vector3>) {
        self.line_orders.push(LineOrder {
            color: self.line_color,
            points: [point_1.into(), point_2.into()],
            transform: self.transform,
        });
    }

    /// Draw a custom shape from points
    pub fn draw_shape(&mut self, points: &[Vector2]) {
        // don't draw polygon with less than 2 points
        if points.len() < 3 {
            return;
        }

        let opaque = (self.shape_color.a - 1.0).abs() < f32::EPSILON;

        // triangulate points
        let first = Vector3::from((points[0], 0.0));
        for i in 2..points.len() {
            self.shape_orders.push(ShapeOrder {
                color: self.shape_color,
                points: [
                    first,
                    Vector3::from((points[i - 1], 0.0)),
                    Vector3::from((points[i], 0.0)),
                ],
                transform: self.transform,
                texture: self.builtins.white_texture.clone(),
                uvs: [Vector2::default(); 3],
                sampler_index: 0,
                opaque,
            });
        }

        // draw borders
        if self.border_mode != BorderMode::Disabled {
            self.push();
            self.transform.move_back(0.00001);
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

    /// Draw mitered line from points
    pub fn draw_lines(&mut self, points: &[Vector2], closed: bool) {
        self.draw_path(
            points,
            closed,
            BorderMode::Center,
            self.line_color,
            self.line_width,
        );
    }

    /// Draw a rectangle
    pub fn draw_rectangle(&mut self, size: impl Into<Vector2>) {
        let s = size.into();

        self.push();

        if self.shape_mode == ShapeMode::Center {
            self.transform.position -= Vector3::from((s / 2.0, 0.0));
        }

        self.draw_shape(&[
            Vector2::new(0.0, s.y),
            s,
            Vector2::new(s.x, 0.0),
            Vector2::default(),
        ]);

        self.pop();
    }

    /// Draw a square
    pub fn draw_square(&mut self, size: f32) {
        self.draw_rectangle(Vector2::new(size, size));
    }

    /// Draw an ellipse
    pub fn draw_ellipse(&mut self, size: impl Into<Vector2>) {
        let s = size.into() / 2.0;
        let side_count = (s.length() * 3.0) as u32;

        self.push();

        if self.shape_mode == ShapeMode::Corner {
            self.transform.position += Vector3::from((s, 0.0));
        }

        let points: Vec<_> = (0..side_count)
            .map(|i| {
                let q = 2.0 * PI * (i as f32 / side_count as f32);
                let x = s.x * q.cos();
                let y = s.y * q.sin();
                Vector2::new(x, y)
            })
            .collect();
        self.draw_shape(&points);

        self.pop();
    }

    /// Draw a circle
    pub fn draw_circle(&mut self, size: f32) {
        self.draw_ellipse(Vector2::new(size, size));
    }

    /// Draw a textured quad
    pub fn draw_texture(&mut self, texture: &Handle<Texture>, size: impl Into<Vector2>) {
        let s = Vector3::from((size.into(), 0.0));

        let opaque = texture.opaque() && (self.shape_color.a - 1.0).abs() < f32::EPSILON;

        self.push();

        if self.shape_mode == ShapeMode::Center {
            self.transform.position -= s / 2.0;
        }

        self.shape_orders.push(ShapeOrder {
            color: self.shape_color,
            points: [
                Vector3::default(),
                Vector3::new(0.0, s.y, 0.0),
                Vector3::new(s.x, 0.0, 0.0),
            ],
            transform: self.transform,
            texture: texture.clone(),
            uvs: [
                Vector2::new(0.0, 1.0),
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 1.0),
            ],
            sampler_index: self.sampler_index(),
            opaque,
        });
        self.shape_orders.push(ShapeOrder {
            color: self.shape_color,
            points: [Vector3::new(0.0, s.y, 0.0), s, Vector3::new(s.x, 0.0, 0.0)],
            transform: self.transform,
            texture: texture.clone(),
            uvs: [
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, 1.0),
            ],
            sampler_index: self.sampler_index(),
            opaque,
        });

        self.pop();
    }

    /// Draw all of the meshes of a model
    pub fn draw_model(&mut self, model: &Handle<Model>) {
        self.push();
        for node in &model.nodes {
            self.draw_model_node(node, Matrix4::from(self.transform));
        }
        self.pop();
    }

    fn draw_model_node(&mut self, node: &ModelNode, parent_matrix: Matrix4) {
        let matrix = parent_matrix * node.matrix;
        self.transform = Transform::from(matrix);

        for (mesh, material) in node.orders() {
            self.set_material(material);
            self.draw_mesh(mesh);
        }

        for child in &node.children {
            self.draw_model_node(child, matrix);
        }
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
                None => so.orders.push(MaterialOrder {
                    material,
                    orders: vec![order],
                }),
            },
            None => self.mesh_orders.push(ShaderOrder {
                shader,
                orders: vec![MaterialOrder {
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
        color: Rgbf,
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
        match (self.texture_filter, self.texture_wrap, self.texture_mipmaps) {
            (Filter::Linear, Wrap::Repeat, true) => 0,
            (Filter::Linear, Wrap::Repeat, false) => 1,
            (Filter::Linear, Wrap::ClampBorder, true) => 2,
            (Filter::Linear, Wrap::ClampBorder, false) => 3,
            (Filter::Linear, Wrap::ClampEdge, true) => 4,
            (Filter::Linear, Wrap::ClampEdge, false) => 5,
            (Filter::Nearest, Wrap::Repeat, true) => 6,
            (Filter::Nearest, Wrap::Repeat, false) => 7,
            (Filter::Nearest, Wrap::ClampBorder, true) => 8,
            (Filter::Nearest, Wrap::ClampBorder, false) => 9,
            (Filter::Nearest, Wrap::ClampEdge, true) => 10,
            (Filter::Nearest, Wrap::ClampEdge, false) => 11,
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
