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
/// let camera = Camera::projection(90);
///
/// duku.draw_on_window(Some(&camera), |target| {
///     // draw commands
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Camera {
    /// the transform of the camera
    pub transform: Transform,
    /// field of view for perspective cameras
    pub fov: u32,
    /// the width of the camera, if width is `None`
    /// the camera will be automatically sized based
    /// on the render texture or window
    pub width: Option<f32>,
    /// the height of the camera, if height is `None`
    /// the camera will be automatically sized based
    /// on the render texture or window
    pub height: Option<f32>,
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
    /// Create a perspective camera that is autosized
    pub fn perspective(fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Perspective,
            depth: 100.0,
            width: None,
            height: None,
            fov,
        }
    }

    /// Create a orthographic camera that is autosized
    pub fn orthographic() -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Orthographic,
            depth: 100.0,
            fov: 0,
            width: None,
            height: None,
        }
    }

    /// Create a perspective camera
    pub fn perspective_sized(width: f32, height: f32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Perspective,
            depth: 100.0,
            width: Some(width),
            height: Some(height),
            fov,
        }
    }

    /// Create a orthographic camera
    pub fn orthographic_sized(width: f32, height: f32) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::Orthographic,
            depth: 100.0,
            width: Some(width),
            height: Some(height),
            fov: 0,
        }
    }

    /// Create a new camera
    pub fn new(projection: Projection, width: f32, height: f32, depth: f32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            height: Some(height),
            width: Some(width),
            fov,
            depth,
            projection,
        }
    }

    /// Convert perspective camera to a zoomed-in orthographic one
    ///
    /// Note: camera has to have a set height
    pub fn fake_orthographic(&mut self, enable: bool) {
        // validate camera
        if let Projection::Orthographic = self.projection {
            return;
        }
        if self.height.is_none() {
            return;
        }

        // set fake
        if enable {
            let height = (self.fov as f32).to_radians().tan() * self.depth;
            let zoom = height / self.height.expect("bad code");
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
        let width = self.width.expect("bad code");
        let height = self.height.expect("bad code");

        match self.projection {
            Projection::Orthographic => {
                Matrix4::orthographic(width, height, self.near(), self.depth)
            }
            Projection::Perspective => {
                Matrix4::perspective(self.fov as f32, width / height, self.near(), self.depth)
            }
        }
    }

    pub(crate) const fn near(&self) -> f32 {
        match self.projection {
            Projection::Orthographic => -0.001,
            Projection::Perspective => 0.001,
        }
    }
}
