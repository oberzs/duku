// Oliver Berzs
// https://github.com/oberzs/duku

use std::f32::consts::PI;

use super::Light;
use crate::color::Rgb;
use crate::color::Rgbf;
use crate::font::Font;
use crate::image::Canvas;
use crate::image::Cubemap;
use crate::image::Filter;
use crate::image::Texture;
use crate::image::Wrap;
use crate::math::Mat4;
use crate::math::Quat;
use crate::math::Vec2;
use crate::math::Vec3;
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
    // colors
    fill: Rgb,
    stroke: Rgb,
    tint: Rgb,
    pub(crate) background: Rgb,

    // shadows
    shadows: bool,
    pub(crate) shadow_depth: f32,
    pub(crate) shadow_split: f32,
    pub(crate) shadow_softness: Pcf,

    // lights
    light_index: usize,
    pub(crate) lights: [Light; 4],
    pub(crate) ambient: Vec3,
    pub(crate) exposure: f32,

    // other
    matrix: Mat4,
    stroke_weight: f32,
    font_size: u32,
    shape_mode: ShapeMode,
    border_mode: BorderMode,
    filter: Filter,
    wrap: Wrap,

    // resources
    shader: Option<Handle<Shader>>,
    material: Option<Handle<Material>>,
    font: Option<Handle<Font>>,
    pub(crate) skybox: Option<Handle<Cubemap>>,
    pub(crate) builtins: Builtins,

    // orders
    pub(crate) mesh_orders: Vec<ShaderOrder>,
    pub(crate) line_orders: Vec<LineOrder>,
    pub(crate) tri_orders: Vec<TriOrder>,
    pub(crate) char_orders: Vec<CharOrder>,

    cache: Vec<Cache>,
}

/// Shape positioning mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShapeMode {
    /// position from bottom-left corner
    BottomLeft,
    /// position from bottom-right corner
    BottomRight,
    /// position from top-left corner
    TopLeft,
    /// position from top-right corner
    TopRight,
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
    pub(crate) matrix: Mat4,
    pub(crate) color: Rgb,
    pub(crate) shadows: bool,
    pub(crate) sampler_index: u32,
}

pub(crate) struct CharOrder {
    pub(crate) points: [Vec3; 4],
    pub(crate) uvs: [Vec2; 4],
    pub(crate) color: Rgb,
    pub(crate) texture: u32,
}

pub(crate) struct LineOrder {
    pub(crate) points: [Vec3; 2],
    pub(crate) color: Rgb,
}

pub(crate) struct TriOrder {
    pub(crate) points: [Vec3; 3],
    pub(crate) uvs: [Vec2; 3],
    pub(crate) color: Rgb,
    pub(crate) texture: u32,
    pub(crate) sampler_index: u32,
    pub(crate) opaque: bool,
}

struct Cache {
    shader: Option<Handle<Shader>>,
    material: Option<Handle<Material>>,
    font: Option<Handle<Font>>,

    // colors
    background: Rgb,
    fill: Rgb,
    stroke: Rgb,
    tint: Rgb,

    // shadows
    shadows: bool,

    // other
    matrix: Mat4,
    stroke_weight: f32,
    font_size: u32,
    shape_mode: ShapeMode,
    border_mode: BorderMode,
    filter: Filter,
    wrap: Wrap,
}

impl Target {
    pub(crate) fn new(builtins: &Builtins) -> Self {
        Self {
            background: Rgb::gray(255),
            fill: Rgb::gray(255),
            stroke: Rgb::gray(0),
            tint: Rgb::gray(255),

            shadows: false,
            shadow_depth: 50.0,
            shadow_split: 0.5,
            shadow_softness: Pcf::X16,

            lights: [Light::none(); 4],
            light_index: 0,
            ambient: Vec3::uniform(0.03),
            exposure: 1.0,

            matrix: Mat4::identity(),
            stroke_weight: 2.0,
            font_size: 24,
            shape_mode: ShapeMode::Center,
            border_mode: BorderMode::Center,
            filter: Filter::Linear,
            wrap: Wrap::Repeat,

            font: None,
            shader: None,
            material: None,
            skybox: None,
            builtins: builtins.clone(),

            mesh_orders: vec![],
            char_orders: vec![],
            line_orders: vec![],
            tri_orders: vec![],

            cache: vec![],
        }
    }

    /// Set background color of canvas
    pub fn background(&mut self, color: impl Into<Rgb>) {
        self.background = color.into();
    }

    /// Set fill color for shapes and text
    pub fn fill(&mut self, color: impl Into<Rgb>) {
        self.fill = color.into();
    }

    /// Set stroke color for borders and lines
    pub fn stroke(&mut self, color: impl Into<Rgb>) {
        self.stroke = color.into();
    }

    /// Set tint color for meshes and textures
    pub fn tint(&mut self, color: impl Into<Rgb>) {
        self.tint = color.into();
    }

    /// Set material for meshes
    pub fn material(&mut self, m: &Handle<Material>) {
        self.material = Some(m.clone());
    }

    /// Use default material for meshes
    pub fn no_material(&mut self) {
        self.material = None;
    }

    /// Set shader for meshes
    pub fn shader(&mut self, s: &Handle<Shader>) {
        self.shader = Some(s.clone());
    }

    /// Use default shader for meshes
    pub fn no_shader(&mut self) {
        self.shader = None;
    }

    /// Set skybox for rendering
    pub fn skybox(&mut self, s: &Handle<Cubemap>) {
        self.skybox = Some(s.clone());
    }

    /// Enable casting shadows for meshes
    ///
    /// Uses the first directional light for
    /// casting the shadows.
    pub fn shadows(&mut self) {
        self.shadows = true;
    }

    /// Disable casting shadows for meshes
    pub fn no_shadows(&mut self) {
        self.shadows = false;
    }

    /// Set max shadow depth
    pub fn shadow_depth(&mut self, d: f32) {
        self.shadow_depth = d;
    }

    /// Set shadow split coefficient
    ///
    /// 0 to 1, higher values make closer shadows
    /// look better.
    pub fn shadow_split(&mut self, s: f32) {
        self.shadow_split = s;
    }

    /// Set how soft shadows appear
    pub fn shadow_softness(&mut self, pcf: Pcf) {
        self.shadow_softness = pcf;
    }

    /// Add light to scene
    ///
    /// Max is 4. When lights are full,
    /// removes first lights.
    pub fn light(&mut self, l: Light) {
        self.lights[self.light_index] = l;
        self.light_index = (self.light_index + 1) % 4;
    }

    /// Set the ambient light that affects the entire scene
    pub fn ambient(&mut self, color: impl Into<Rgbf>, brightness: f32) {
        self.ambient = Vec3::from(color.into()) * brightness;
    }

    /// Set the exposure for tone mapping
    pub fn exposure(&mut self, e: f32) {
        self.exposure = e;
    }

    /// Set the stroke weight for lines and borders
    pub fn stroke_weight(&mut self, w: f32) {
        self.stroke_weight = w;
    }

    /// Set font size for text
    pub fn font_size(&mut self, s: u32) {
        self.font_size = s;
    }

    /// Set border mode
    pub fn border_mode(&mut self, mode: BorderMode) {
        self.border_mode = mode;
    }

    /// Set shape mode
    pub fn shape_mode(&mut self, mode: ShapeMode) {
        self.shape_mode = mode;
    }

    /// Set filter for textures
    pub fn filter(&mut self, f: Filter) {
        self.filter = f;
    }

    /// Set wrap mode for textures
    pub fn wrap(&mut self, w: Wrap) {
        self.wrap = w;
    }

    /// Transform points by matrix
    pub fn transform(&mut self, matrix: impl Into<Mat4>) {
        self.matrix = matrix.into() * self.matrix;
    }

    /// Reset transform to default
    pub fn reset_transform(&mut self) {
        self.matrix = Mat4::identity();
    }

    /// Move transform by vector
    pub fn translate(&mut self, v: impl Into<Vec3>) {
        self.matrix = Mat4::translation(v) * self.matrix;
    }

    /// Move transform on X axis
    pub fn translate_x(&mut self, x: f32) {
        self.matrix = Mat4::translation(Vec3::right() * x) * self.matrix;
    }

    /// Move transform on Y axis
    pub fn translate_y(&mut self, y: f32) {
        self.matrix = Mat4::translation(Vec3::up() * y) * self.matrix;
    }

    /// Move transform on Z axis
    pub fn translate_z(&mut self, z: f32) {
        self.matrix = Mat4::translation(Vec3::forward() * z) * self.matrix;
    }

    /// Scale transform by vector
    pub fn scale(&mut self, v: impl Into<Vec3>) {
        self.matrix = Mat4::scale(v) * self.matrix;
    }

    /// Rotate transform by quaternion
    pub fn rotate(&mut self, q: impl Into<Quat>) {
        self.matrix = Mat4::from(q.into()) * self.matrix;
    }

    /// Rotate transform on the X axis
    ///
    /// This angle is in degrees.
    pub fn rotate_x(&mut self, d: f32) {
        self.matrix = Mat4::euler_rotation(d, 0.0, 0.0) * self.matrix;
    }

    /// Rotate transform on the Y axis
    ///
    /// This angle is in degrees.
    pub fn rotate_y(&mut self, d: f32) {
        self.matrix = Mat4::euler_rotation(0.0, d, 0.0) * self.matrix;
    }

    /// Rotate transform on the Z axis
    ///
    /// This angle is in degrees.
    pub fn rotate_z(&mut self, d: f32) {
        self.matrix = Mat4::euler_rotation(0.0, 0.0, d) * self.matrix;
    }

    /// Draw a 2D line
    pub fn line(&mut self, p1: impl Into<Vec2>, p2: impl Into<Vec2>) {
        let weight = self.stroke_weight / 2.0;
        self.path(&[p1.into(), p2.into()], false, weight, weight);
    }

    /// Draw mitered 2D lines
    pub fn lines(&mut self, points: &[Vec2]) {
        let weight = self.stroke_weight / 2.0;
        self.path(points, false, weight, weight);
    }

    /// Draw a custom 3D mesh
    pub fn mesh(&mut self, mesh: &Handle<Mesh>) {
        let unshaded = self.lights.iter().all(|l| l.is_none());

        let order = MeshOrder {
            mesh: mesh.clone(),
            matrix: self.matrix,
            color: self.tint,
            shadows: self.shadows,
            sampler_index: self.sampler_index(),
        };

        let shader = match &self.shader {
            Some(s) => s,
            None if unshaded => &self.builtins.unshaded_shader,
            None => &self.builtins.pbr_shader,
        };

        let material = match &self.material {
            Some(m) => m,
            None => &self.builtins.white_material,
        };

        match self.mesh_orders.iter_mut().find(|so| &so.shader == shader) {
            Some(so) => match so.orders.iter_mut().find(|mo| &mo.material == material) {
                Some(mo) => mo.orders.push(order),
                None => so.orders.push(MaterialOrder {
                    material: material.clone(),
                    orders: vec![order],
                }),
            },
            None => self.mesh_orders.push(ShaderOrder {
                shader: shader.clone(),
                orders: vec![MaterialOrder {
                    material: material.clone(),
                    orders: vec![order],
                }],
            }),
        }
    }

    /// Draw a wireframe for a 3D mesh
    pub fn wireframe(&mut self, mesh: &Handle<Mesh>) {
        self.push();
        self.shadows = false;
        self.shader = Some(self.builtins.wireframe_shader.clone());
        self.mesh(mesh);
        self.pop();
    }

    /// Draw a scaled cube
    pub fn cube(&mut self, scale: impl Into<Vec3>) {
        self.push();
        self.matrix *= Mat4::scale(scale);
        let mesh = self.builtins.cube_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw a scaled uv sphere
    pub fn sphere_uv(&mut self, scale: impl Into<Vec3>) {
        self.push();
        self.matrix *= Mat4::scale(scale);
        let mesh = self.builtins.uv_sphere_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw a scaled ico sphere
    pub fn sphere_ico(&mut self, scale: impl Into<Vec3>) {
        self.push();
        self.matrix *= Mat4::scale(scale);
        let mesh = self.builtins.ico_sphere_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw a scaled plane
    pub fn plane(&mut self, scale: impl Into<Vec2>) {
        self.push();
        self.matrix *= Mat4::scale(Vec3::from((scale.into(), 1.0)));
        let mesh = self.builtins.plane_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw a surface with a custom shader
    pub fn surface(&mut self, shader: &Handle<Shader>) {
        self.push();
        self.shader = Some(shader.clone());
        let mesh = self.builtins.surface_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw a fullscreen canvas
    pub fn fullscreen(&mut self, canvas: &Handle<Canvas>) {
        self.push();
        self.shader = Some(self.builtins.fullscreen_shader.clone());
        self.material = Some(canvas.material().clone());
        let mesh = self.builtins.surface_mesh.clone();
        self.mesh(&mesh);
        self.pop();
    }

    /// Draw all of the meshes of a model
    pub fn model(&mut self, model: &Handle<Model>) {
        self.push();
        for node in &model.nodes {
            self.model_node(node, self.matrix);
        }
        self.pop();
    }

    /// Draw a 3D debug line that isn't controlled by
    /// [stroke_weight](crate::renderer::Target::stroke_weight)
    pub fn debug_line<V: Into<Vec3>>(&mut self, p1: V, p2: V) {
        self.line_orders.push(LineOrder {
            points: [self.matrix * p1.into(), self.matrix * p2.into()],
            color: self.stroke,
        });
    }

    /// Draw 3D guide grid for X and Z axis
    pub fn debug_grid(&mut self) {
        let size = 100;
        let half = size / 2;
        let width = 1.0;

        self.push();
        for x in -half..half {
            let xx = x as f32 * width;
            let z_min = -half as f32 * width;
            let z_max = half as f32 * width;

            let color = match x {
                0 => "#0000fff0",
                _ if x % 10 == 0 => "#fffffff0",
                _ => "#ffffff0f",
            };

            self.stroke(color);
            self.debug_line([xx, 0.0, z_min], [xx, 0.0, z_max]);
        }
        for z in -half..half {
            let zz = z as f32 * width;
            let x_min = -half as f32 * width;
            let x_max = half as f32 * width;

            let color = match z {
                0 => "#ff0000f0",
                _ if z % 10 == 0 => "#fffffff0",
                _ => "#ffffff0f",
            };

            self.stroke(color);
            self.debug_line([x_min, 0.0, zz], [x_max, 0.0, zz]);
        }
        self.pop();
    }

    /// Draw text string
    pub fn text(&mut self, text: impl AsRef<str>, pos: impl Into<Vec2>) {
        let mut advance = pos.into();
        let t = text.as_ref();
        let font = self.font.as_ref().unwrap_or(&self.builtins.fira_font);

        let w = self.text_width(t);
        let fs = self.font_size as f32;

        advance += match self.shape_mode {
            ShapeMode::BottomLeft => Vec2::new(0.0, fs),
            ShapeMode::BottomRight => Vec2::new(-w, fs),
            ShapeMode::TopLeft => Vec2::new(0.0, 0.0),
            ShapeMode::TopRight => Vec2::new(-w, 0.0),
            ShapeMode::Center => Vec2::new(-w / 2.0, fs / 2.0),
        };

        for c in t.chars() {
            // handle whitespace
            if c == ' ' {
                advance.x += fs / 3.0;
                continue;
            }
            if c == '\n' {
                advance.x = 0.0;
                advance.y -= fs;
                continue;
            }

            // calculate positions
            let data = font.char_data(c);
            let mut pos = advance;
            pos.x += data.x_offset * fs;
            pos.y -= data.y_offset * fs;
            let width = data.width * fs;
            let height = data.height * fs;

            // calculate points
            let p1 = self.matrix * Vec3::new(pos.x, pos.y, 0.0);
            let p2 = self.matrix * Vec3::new(pos.x + width, pos.y, 0.0);
            let p3 = self.matrix * Vec3::new(pos.x + width, pos.y - height, 0.0);
            let p4 = self.matrix * Vec3::new(pos.x, pos.y - height, 0.0);

            let uv1 = Vec2::new(data.uvs.x, data.uvs.y);
            let uv2 = Vec2::new(data.uvs.z, data.uvs.y);
            let uv3 = Vec2::new(data.uvs.z, data.uvs.w);
            let uv4 = Vec2::new(data.uvs.x, data.uvs.w);

            // add order
            self.char_orders.push(CharOrder {
                points: [p1, p2, p3, p4],
                uvs: [uv1, uv2, uv3, uv4],
                color: self.stroke,
                texture: font.texture().shader_index(),
            });

            advance.x += data.advance * fs;
        }
    }

    /// Draw a custom shape from points
    ///
    /// Shape must be convex
    pub fn shape(&mut self, points: &[Vec2]) {
        // don't draw shape with less than 2 points
        if points.len() < 3 {
            return;
        }

        // check if should draw shape
        if self.fill.a > 0 {
            let texture = self.builtins.white_texture.shader_index();
            let opaque = self.fill.a == 255;

            // triangulate points
            let first = Vec3::from((points[0], 0.0));
            for i in 2..points.len() {
                self.tri_orders.push(TriOrder {
                    points: [
                        self.matrix * first,
                        self.matrix * Vec3::from((points[i - 1], 0.0)),
                        self.matrix * Vec3::from((points[i], 0.0)),
                    ],
                    uvs: [Vec2::default(); 3],
                    color: self.fill,
                    sampler_index: 0,
                    texture,
                    opaque,
                });
            }
        }

        // check if should draw borders
        if self.stroke.a > 0 {
            let outer_weight = match self.border_mode {
                BorderMode::Center => self.stroke_weight / 2.0,
                BorderMode::Outside => self.stroke_weight,
                BorderMode::Inside => 0.0,
            };
            let inner_weight = match self.border_mode {
                BorderMode::Center => self.stroke_weight / 2.0,
                BorderMode::Outside => 0.0,
                BorderMode::Inside => self.stroke_weight,
            };

            self.path(points, true, inner_weight, outer_weight);
        }
    }

    /// Draw a rectangle
    pub fn rect(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) {
        let s = size.into();
        let p = pos.into();

        let offset = match self.shape_mode {
            ShapeMode::BottomLeft => Vec2::new(0.0, 0.0),
            ShapeMode::BottomRight => Vec2::new(-s.x, 0.0),
            ShapeMode::TopLeft => Vec2::new(0.0, -s.y),
            ShapeMode::TopRight => Vec2::new(-s.x, -s.y),
            ShapeMode::Center => Vec2::new(-s.x / 2.0, -s.y / 2.0),
        };

        self.shape(&[
            Vec2::new(p.x, p.y + s.y) + offset,
            Vec2::new(p.x + s.x, p.y + s.y) + offset,
            Vec2::new(p.x + s.x, p.y) + offset,
            Vec2::new(p.x, p.y) + offset,
        ]);
    }

    /// Draw a square
    pub fn square(&mut self, pos: impl Into<Vec2>, size: f32) {
        self.rect(pos, Vec2::new(size, size));
    }

    /// Draw an ellipse
    pub fn ellipse(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) {
        let s = size.into() / 2.0;
        let side_count = (s.length() * 3.0) as u32;
        let position = pos.into();

        let offset = match self.shape_mode {
            ShapeMode::BottomLeft => s,
            ShapeMode::BottomRight => Vec2::new(-s.x, s.y),
            ShapeMode::TopLeft => Vec2::new(s.x, -s.y),
            ShapeMode::TopRight => Vec2::new(-s.x, -s.y),
            ShapeMode::Center => Vec2::new(0.0, 0.0),
        };

        let points: Vec<_> = (0..side_count)
            .map(|i| {
                let q = 2.0 * PI * (i as f32 / side_count as f32);
                let x = s.x * q.cos();
                let y = s.y * q.sin();
                position + offset + Vec2::new(x, y)
            })
            .collect();
        self.shape(&points);
    }

    /// Draw a circle
    pub fn circle(&mut self, pos: impl Into<Vec2>, size: f32) {
        self.ellipse(pos, Vec2::new(size, size));
    }

    /// Draw a textured quad
    pub fn texture(
        &mut self,
        texture: &Handle<Texture>,
        pos: impl Into<Vec2>,
        size: impl Into<Vec2>,
    ) {
        let tw = texture.width() as f32;
        let th = texture.height() as f32;
        self.texture_part(texture, pos, size, [0.0, 0.0], [tw, th]);
    }

    /// Draw a quad with part of a texture
    ///
    /// Part is defined in pixels
    pub fn texture_part(
        &mut self,
        texture: &Handle<Texture>,
        pos: impl Into<Vec2>,
        size: impl Into<Vec2>,
        part_pos: impl Into<Vec2>,
        part_size: impl Into<Vec2>,
    ) {
        let s = size.into();
        let p = pos.into();
        let pp = part_pos.into();
        let ps = part_size.into();
        let tw = texture.width() as f32;
        let th = texture.height() as f32;
        let opaque = texture.opaque() && self.tint.a == 255;

        let offset = match self.shape_mode {
            ShapeMode::BottomLeft => Vec3::new(0.0, 0.0, 0.0),
            ShapeMode::BottomRight => Vec3::new(-s.x, 0.0, 0.0),
            ShapeMode::TopLeft => Vec3::new(0.0, -s.y, 0.0),
            ShapeMode::TopRight => Vec3::new(-s.x, -s.y, 0.0),
            ShapeMode::Center => Vec3::new(-s.x / 2.0, -s.y / 2.0, 0.0),
        };

        let p1 = self.matrix * (Vec3::new(p.x, p.y + s.y, 0.0) + offset);
        let p2 = self.matrix * (Vec3::new(p.x + s.x, p.y + s.y, 0.0) + offset);
        let p3 = self.matrix * (Vec3::new(p.x + s.x, p.y, 0.0) + offset);
        let p4 = self.matrix * (Vec3::new(p.x, p.y, 0.0) + offset);

        let uv1 = Vec2::new(pp.x / tw, pp.y / th);
        let uv2 = Vec2::new((pp.x + ps.x) / tw, pp.y / th);
        let uv3 = Vec2::new((pp.x + ps.x) / tw, (pp.y + ps.y) / th);
        let uv4 = Vec2::new(pp.x / tw, (pp.y + ps.y) / th);

        self.tri_orders.push(TriOrder {
            points: [p1, p2, p3],
            color: self.tint,
            uvs: [uv1, uv2, uv3],
            texture: texture.shader_index(),
            sampler_index: self.sampler_index(),
            opaque,
        });
        self.tri_orders.push(TriOrder {
            points: [p1, p3, p4],
            color: self.tint,
            uvs: [uv1, uv3, uv4],
            texture: texture.shader_index(),
            sampler_index: self.sampler_index(),
            opaque,
        });
    }

    /// Get text width for current font
    pub fn text_width(&self, text: impl AsRef<str>) -> f32 {
        let font = self.font.as_ref().unwrap_or(&self.builtins.fira_font);
        let scale = self.font_size as f32;
        let mut width = 0.0;

        for c in text.as_ref().chars() {
            // handle whitespace
            if c == ' ' {
                width += scale / 3.0;
                continue;
            }

            let data = font.char_data(c);
            width += data.advance * scale;
        }

        width
    }

    /// Save target settings to stack
    pub fn push(&mut self) {
        self.cache.push(Cache {
            shader: self.shader.clone(),
            material: self.material.clone(),
            font: self.font.clone(),

            background: self.background,
            fill: self.fill,
            stroke: self.stroke,
            tint: self.tint,

            shadows: self.shadows,

            matrix: self.matrix,
            stroke_weight: self.stroke_weight,
            font_size: self.font_size,
            shape_mode: self.shape_mode,
            border_mode: self.border_mode,
            filter: self.filter,
            wrap: self.wrap,
        });
    }

    /// Restore target settings from stack
    pub fn pop(&mut self) {
        if let Some(cache) = self.cache.pop() {
            self.shader = cache.shader;
            self.material = cache.material;
            self.font = cache.font;

            self.background = cache.background;
            self.fill = cache.fill;
            self.stroke = cache.stroke;
            self.tint = cache.tint;

            self.shadows = cache.shadows;

            self.matrix = cache.matrix;
            self.stroke_weight = cache.stroke_weight;
            self.font_size = cache.font_size;
            self.shape_mode = cache.shape_mode;
            self.border_mode = cache.border_mode;
            self.filter = cache.filter;
            self.wrap = cache.wrap;
        }
    }

    fn model_node(&mut self, node: &ModelNode, parent: Mat4) {
        self.matrix = parent * node.matrix;

        for (mesh, material) in node.orders() {
            self.material(material);
            self.mesh(mesh);
        }

        for child in &node.children {
            self.model_node(child, self.matrix);
        }
    }

    fn path(&mut self, path: &[Vec2], closed: bool, inner_weight: f32, outer_weight: f32) {
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

        // draw tris
        for i in 0..(normals.len() - 1) {
            let curr_norm = normals[i];
            let next_norm = normals[i + 1];
            let curr_point = points[i];
            let next_point = *points.get(i + 1).unwrap_or(&points[0]);

            let p1 = self.matrix * Vec3::from((curr_point + curr_norm * outer_weight, -0.00001));
            let p2 = self.matrix * Vec3::from((next_point + next_norm * outer_weight, -0.00001));
            let p3 = self.matrix * Vec3::from((next_point - next_norm * inner_weight, -0.00001));
            let p4 = self.matrix * Vec3::from((curr_point - curr_norm * inner_weight, -0.00001));

            let texture = self.builtins.white_texture.shader_index();

            self.tri_orders.push(TriOrder {
                points: [p1, p2, p3],
                color: self.stroke,
                uvs: [Vec2::default(); 3],
                opaque: self.stroke.a == 255,
                sampler_index: 0,
                texture,
            });
            self.tri_orders.push(TriOrder {
                points: [p1, p3, p4],
                color: self.stroke,
                uvs: [Vec2::default(); 3],
                opaque: self.stroke.a == 255,
                sampler_index: 0,
                texture,
            });
        }
    }

    const fn sampler_index(&self) -> u32 {
        match (self.filter, self.wrap) {
            (Filter::Linear, Wrap::Repeat) => 0,
            (Filter::Linear, Wrap::ClampBorder) => 1,
            (Filter::Linear, Wrap::ClampEdge) => 2,
            (Filter::Nearest, Wrap::Repeat) => 3,
            (Filter::Nearest, Wrap::ClampBorder) => 4,
            (Filter::Nearest, Wrap::ClampEdge) => 5,
        }
    }
}

fn miter(line_a: Vec2, line_b: Vec2) -> Vec2 {
    let tangent = (line_a + line_b).unit();
    let miter = tangent.normal();
    let norm_a = line_a.normal();
    let miter_len = 1.0 / miter.dot(norm_a);

    miter * miter_len
}
