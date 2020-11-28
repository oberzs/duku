// Oliver Berzs
// https://github.com/oberzs/duku

use crate::math::Mat4;
use crate::math::Quat;
use crate::math::Vec3;

/// The view into a scene.
///
/// # Examples
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
    /// the position of the camera
    pub position: Vec3,
    /// the scale of the camera
    pub scale: Vec3,
    /// the rotation of the camera
    pub rotation: Quat,
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
            position: Vec3::default(),
            scale: Vec3::uniform(1.0),
            rotation: Quat::default(),
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
            position: Vec3::default(),
            scale: Vec3::uniform(1.0),
            rotation: Quat::default(),
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
            position: Vec3::default(),
            scale: Vec3::uniform(1.0),
            rotation: Quat::default(),
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
            position: Vec3::default(),
            scale: Vec3::uniform(1.0),
            rotation: Quat::default(),
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
            position: Vec3::default(),
            scale: Vec3::uniform(1.0),
            rotation: Quat::default(),
            height: Some(height),
            width: Some(width),
            fov,
            depth,
            projection,
        }
    }

    /// Convert perspective camera to a zoomed-in orthographic one
    ///
    /// Camera has to have a set height
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
            self.scale = Vec3::uniform(zoom);
        } else {
            self.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }

    /// Move camera by specified amount
    ///
    /// This moves using global directions
    pub fn move_by(&mut self, amount: impl Into<Vec3>) {
        self.position += amount.into();
    }

    /// Move camera up by specified amount
    ///
    /// This moves using local directions
    pub fn move_up(&mut self, amount: f32) {
        self.move_by(self.rotation.local_up() * amount);
    }

    /// Move camera down by specified amount
    ///
    /// This moves using local directions
    pub fn move_down(&mut self, amount: f32) {
        self.move_by(-self.rotation.local_up() * amount);
    }

    /// Move camera right by specified amount
    ///
    /// This moves using local directions
    pub fn move_right(&mut self, amount: f32) {
        self.move_by(self.rotation.local_up() * amount);
    }

    /// Move camera left by specified amount
    ///
    /// This moves using local directions
    pub fn move_left(&mut self, amount: f32) {
        self.move_by(-self.rotation.local_up() * amount);
    }

    /// Move camera forward by specified amount
    ///
    /// This moves using local directions
    pub fn move_forward(&mut self, amount: f32) {
        self.move_by(self.rotation.local_up() * amount);
    }

    /// Move camera back by specified amount
    ///
    /// This moves using local directions
    pub fn move_back(&mut self, amount: f32) {
        self.move_by(-self.rotation.local_up() * amount);
    }

    /// Move camera rotating it around
    /// some point around an axis
    pub fn move_around_point(&mut self, point: impl Into<Vec3>, angle: f32, axis: impl Into<Vec3>) {
        let point = point.into();
        let rotation = Quat::axis_rotation(axis, angle);
        self.position -= point;
        self.position = rotation * self.position;
        self.position += point;
    }

    /// Rotate the camera to look in specific direction
    pub fn look_dir(&mut self, dir: impl Into<Vec3>) {
        let dir = dir.into().unit();
        let up = if dir == Vec3::up() {
            Vec3::forward()
        } else {
            Vec3::up()
        };
        self.rotation = Quat::look_rotation(dir, up);
    }

    /// Rotate the camera to look at specific position
    pub fn look_at(&mut self, pos: impl Into<Vec3>) {
        self.look_dir(pos.into() - self.position);
    }

    pub(crate) fn world_to_view(&self) -> Mat4 {
        Mat4::scale(self.scale) * Mat4::from(self.rotation) * Mat4::translation(-self.position)
    }

    pub(crate) fn view_to_clip(&self) -> Mat4 {
        let width = self.width.expect("bad code");
        let height = self.height.expect("bad code");

        match self.projection {
            Projection::Orthographic => Mat4::orthographic(width, height, self.near(), self.depth),
            Projection::Perspective => {
                Mat4::perspective(self.fov as f32, width / height, self.near(), self.depth)
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
