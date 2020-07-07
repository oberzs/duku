// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Camera - struct to hold matrix transforms for a camera

#[cfg(feature = "controller")]
mod controller;

use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector3;

#[cfg(feature = "controller")]
pub use controller::Controller;

#[derive(Clone)]
pub struct Camera {
    pub transform: Transform,
    pub fov: u32,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    camera_type: CameraType,
}

#[derive(Debug, Copy, Clone)]
pub enum CameraType {
    Orthographic,
    Perspective,
}

impl Camera {
    pub fn perspective(width: f32, height: f32, depth: f32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Perspective,
            depth,
            width,
            height,
            fov,
        }
    }

    pub fn orthographic(width: f32, height: f32, depth: f32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Orthographic,
            depth,
            fov: 0,
            width,
            height,
        }
    }

    pub fn new(camera_type: CameraType, width: f32, height: f32, depth: f32) -> Self {
        Self {
            transform: Transform::default(),
            fov: 90,
            depth,
            camera_type,
            width,
            height,
        }
    }

    pub fn fake_orthographic(&mut self, enable: bool) {
        if let CameraType::Orthographic = self.camera_type {
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

    pub(crate) fn matrix(&self) -> Matrix4 {
        let projection = match self.camera_type {
            CameraType::Orthographic => {
                Matrix4::orthographic_center(self.width, self.height, 0.001, self.depth)
            }
            CameraType::Perspective => {
                Matrix4::perspective(self.fov as f32, self.width / self.height, 0.001, self.depth)
            }
        };

        let view = self.transform.as_matrix_for_camera();

        projection * view
    }
}
