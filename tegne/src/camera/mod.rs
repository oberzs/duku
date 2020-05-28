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

pub struct Camera {
    transform: Transform,
    camera_type: CameraType,
    width: f32,
    height: f32,
    depth: f32,
    fov: f32,
}

enum CameraType {
    Orthographic,
    Perspective,
}

impl Camera {
    pub fn perspective(width: u32, height: u32, fov: u32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Perspective,
            width: width as f32,
            height: height as f32,
            depth: 5000.0,
            fov: fov as f32,
        }
    }

    pub fn orthographic(width: u32, height: u32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Orthographic,
            width: width as f32,
            height: height as f32,
            depth: 5000.0,
            fov: 0.0,
        }
    }

    pub fn fake_orthographic(&mut self, enable: bool) {
        if let CameraType::Orthographic = self.camera_type {
            return;
        }
        if enable {
            let height = self.fov.to_radians().tan() * self.depth;
            let zoom = height / self.height;
            self.transform.scale = Vector3::new(zoom, zoom, zoom);
        } else {
            self.transform.scale = Vector3::new(1.0, 1.0, 1.0);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
    }

    pub(crate) fn matrix(&self) -> Matrix4 {
        let projection = match self.camera_type {
            CameraType::Orthographic => {
                Matrix4::orthographic_center(self.width, self.height, -0.1, self.depth)
            }
            CameraType::Perspective => {
                Matrix4::perspective(self.fov, self.width / self.height, 0.1, self.depth)
            }
        };

        let view = self.transform.as_matrix_for_camera();

        projection * view
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
