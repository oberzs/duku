// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Camera - struct to hold matrix transforms for a camera

#[cfg(feature = "controller")]
mod controller;

use crate::math::Matrix4;
use crate::math::Sphere;
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
        let projection = self.projection();
        let view = self.transform.as_matrix_for_camera();
        projection * view
    }

    pub(crate) fn bounding_sphere_for_split(&self, near: f32, far: f32) -> Sphere {
        let mut frustum_corners = [
            Vector3::new(-1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, -1.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(-1.0, -1.0, 1.0),
        ];

        let projection = self.projection();
        let view = self.transform.as_matrix_for_camera();

        let inverse_projection = projection.inverse().expect("bad projection");

        // get projection frustum corners from NDC
        for corner in frustum_corners.iter_mut() {
            let point = inverse_projection * corner.extend(1.0);
            *corner = point.shrink() / point.w;
        }

        // cut out a section (near -> far) from the frustum
        for i in 0..4 {
            let corner_ray = frustum_corners[i + 4] - frustum_corners[i];
            let near_corner_ray = corner_ray * near;
            let far_corner_ray = corner_ray * far;
            frustum_corners[i + 4] = frustum_corners[i] + far_corner_ray;
            frustum_corners[i] += near_corner_ray;
        }

        let frustum_center = frustum_corners.iter().sum::<Vector3>() / frustum_corners.len() as f32;

        // get bounding sphere radius
        // sphere makes it axis-aligned
        let mut radius = 0.0;
        for corner in frustum_corners.iter() {
            let distance = (*corner - frustum_center).length();
            if distance > radius {
                radius = distance;
            }
        }

        // round radius to 1/16 increments
        radius = (radius * 16.0).ceil() / 16.0;

        // transform frustum center into world space
        let center = view.inverse().unwrap().transform_vector(frustum_center);

        Sphere { center, radius }
    }

    fn projection(&self) -> Matrix4 {
        match self.camera_type {
            CameraType::Orthographic => {
                Matrix4::orthographic_center(self.width, self.height, 0.0, self.depth)
            }
            CameraType::Perspective => {
                Matrix4::perspective(self.fov as f32, self.width / self.height, 0.001, self.depth)
            }
        }
    }
}
