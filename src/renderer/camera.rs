// Oliver Berzs
// https://github.com/oberzs/duku

// Camera - struct to hold matrix transforms for a camera

use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;

#[derive(Clone)]
pub struct Camera {
    pub transform: Transform,
    pub autosize: bool,
    pub fov: u32,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub projection: Projection,
}

#[derive(Debug, Copy, Clone)]
pub enum Projection {
    Orthographic,
    Perspective,
}

impl Camera {
    pub fn perspective(width: f32, height: f32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Perspective,
            autosize: false,
            depth: 100.0,
            width,
            height,
            fov,
        }
    }

    pub fn orthographic(width: f32, height: f32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Orthographic,
            autosize: false,
            depth: 100.0,
            fov: 0,
            width,
            height,
        }
    }

    pub fn perspective_autosized(fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Perspective,
            autosize: true,
            depth: 100.0,
            width: -1.0,
            height: -1.0,
            fov,
        }
    }

    pub fn orthographic_autosized() -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Orthographic,
            autosize: true,
            depth: 100.0,
            width: -1.0,
            height: -1.0,
            fov: 0,
        }
    }

    pub fn new(projection: Projection, width: f32, height: f32, depth: f32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            autosize: false,
            fov,
            depth,
            projection,
            width,
            height,
        }
    }

    pub fn fake_orthographic(&mut self, enable: bool) {
        if let Projection::Orthographic = self.projection {
            return;
        }
        if enable {
            let height = (self.fov as f32).to_radians().tan() * self.depth;
            let zoom = height / self.height as f32;
            self.transform.scale = Vector3::new(zoom, zoom, zoom);
        } else {
            self.transform.scale = Vector3::new(1.0, 1.0, 1.0);
        }
    }

    pub(crate) fn world_to_view(&self) -> Matrix4 {
        Matrix4::scale(self.transform.scale)
            * Matrix4::from(self.transform.rotation)
            * Matrix4::translation(-self.transform.position)
    }

    pub(crate) fn view_to_clip(&self) -> Matrix4 {
        match self.projection {
            Projection::Orthographic => {
                Matrix4::orthographic(self.width, self.height, self.near(), self.depth)
            }
            Projection::Perspective => Matrix4::perspective(
                self.fov as f32,
                self.width / self.height,
                self.near(),
                self.depth,
            ),
        }
    }

    pub(crate) const fn near(&self) -> f32 {
        match self.projection {
            Projection::Orthographic => -0.001,
            Projection::Perspective => 0.001,
        }
    }
}
