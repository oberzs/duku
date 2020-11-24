// Oliver Berzs
// https://github.com/oberzs/duku

use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;

/// The view into a scene.
///
/// # Example
///
/// ```ignore
/// let camera = Camera::projection_autosized(90);
///
/// duku.draw_on_window(Some(&camera), |target| {
///     // draw commands
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Camera {
    /// the transform of the camera
    pub transform: Transform,
    /// autosized cameras change their size based
    /// on the framebuffer rendered to
    pub autosize: bool,
    /// field of view for perspective cameras
    pub fov: u32,
    /// the width of the camera
    pub width: f32,
    /// the height of the camera
    pub height: f32,
    /// the depth of the camera
    pub depth: f32,
    /// the projection type of the camera
    pub projection: Projection,
}

/// The projection type of a camera
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Projection {
    /// orthographic projection type (parallel lines stay paralel)
    Orthographic,
    /// perspective projection type (parallel lines converge to a point)
    Perspective,
}

impl Camera {
    /// Create a perspective camera
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

    /// Create a orthographic camera
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

    /// Create a perspective camera that is autosized
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

    /// Create a orthographic camera that is autosized
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

    /// Create a new camera
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

    /// Convert perspective camera to a zoomed-in orthographic one
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
