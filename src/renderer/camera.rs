// Oliver Berzs
// https://github.com/oberzs/draw-it

// Camera - struct to hold matrix transforms for a camera

use crate::math::Matrix4;
use crate::math::Sphere;
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
            * Matrix4::from(self.transform.rotation.inverse_rotation())
            * Matrix4::translation(-self.transform.position)
    }

    pub(crate) fn view_to_clip(&self) -> Matrix4 {
        match self.projection {
            Projection::Orthographic => {
                Matrix4::orthographic_center(self.width, self.height, -0.001, self.depth)
            }
            Projection::Perspective => {
                Matrix4::perspective(self.fov as f32, self.width / self.height, 0.001, self.depth)
            }
        }
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

        let view_to_clip = self.view_to_clip();
        let world_to_view = self.world_to_view();

        let inverse_projection = view_to_clip.inverse().expect("bad projection");

        // get projection frustum corners from NDC
        for corner in &mut frustum_corners {
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
        for corner in &frustum_corners {
            let distance = (*corner - frustum_center).length();
            if distance > radius {
                radius = distance;
            }
        }

        // round radius to 1/16 increments
        radius = (radius * 16.0).ceil() / 16.0;

        // transform frustum center into view space
        let center = world_to_view
            .inverse()
            .expect("no inverse")
            .transform_vector(frustum_center);

        Sphere { center, radius }
    }
}
