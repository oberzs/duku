// Oliver Berzs
// https://github.com/oberzs/draw-it

// quaternion rotation struct

use super::Matrix4;
use super::Vector3;

use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn euler_rotation(roll: f32, pitch: f32, yaw: f32) -> Self {
        let cy = (yaw.to_radians() * 0.5).cos();
        let sy = (yaw.to_radians() * 0.5).sin();
        let cp = (pitch.to_radians() * 0.5).cos();
        let sp = (pitch.to_radians() * 0.5).sin();
        let cr = (roll.to_radians() * 0.5).cos();
        let sr = (roll.to_radians() * 0.5).sin();

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = sr * cp * sy + cr * sp * cy;
        let z = cr * cp * sy - sr * sp * cy;

        Self::new(x, y, z, w)
    }

    pub fn axis_rotation(axis: impl Into<Vector3>, angle: f32) -> Self {
        let an = angle.to_radians();
        let ax = axis.into();

        let w = (an / 2.0).cos();
        let x = ax.x * (an / 2.0).sin();
        let y = ax.y * (an / 2.0).sin();
        let z = ax.z * (an / 2.0).sin();

        Self::new(x, y, z, w)
    }

    pub fn look_rotation(dir: impl Into<Vector3>, global_up: impl Into<Vector3>) -> Self {
        let matrix = Matrix4::look_rotation(dir, global_up);
        let col_x = matrix.col_x;
        let col_y = matrix.col_y;
        let col_z = matrix.col_z;

        let mut result = Self::default();
        let trace = col_x.x + col_y.y + col_z.z;
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            result.w = 0.25 / s;
            result.x = (col_z.y - col_y.z) * s;
            result.y = (col_x.z - col_z.x) * s;
            result.z = (col_y.x - col_x.y) * s;
        } else if col_x.x > col_y.y && col_x.x > col_z.z {
            let s = 2.0 * (1.0 + col_x.x - col_y.y - col_z.z).sqrt();
            result.w = (col_z.y - col_y.z) / s;
            result.x = 0.25 * s;
            result.y = (col_x.y + col_y.x) / s;
            result.z = (col_x.z + col_z.x) / s;
        } else if col_y.y > col_z.z {
            let s = 2.0 * (1.0 + col_y.y - col_x.x - col_z.z).sqrt();
            result.w = (col_x.z - col_z.x) / s;
            result.x = (col_x.y + col_y.x) / s;
            result.y = 0.25 * s;
            result.z = (col_y.z + col_z.y) / s;
        } else {
            let s = 2.0 * (1.0 + col_z.z - col_x.x - col_y.y).sqrt();
            result.w = (col_y.x - col_x.y) / s;
            result.x = (col_x.z + col_z.x) / s;
            result.y = (col_y.z + col_z.y) / s;
            result.z = 0.25 * s;
        }
        result
    }

    pub fn inverse_rotation(self) -> Quaternion {
        let mut result = self;
        result.w = -result.w;
        result
    }

    pub fn as_matrix(self) -> Matrix4 {
        let angle = 2.0 * self.w.acos().to_degrees();
        let scale = (1.0 - self.w * self.w).sqrt();
        let axis = if scale < 0.001 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(self.x, self.y, self.z) / scale
        };
        Matrix4::axis_rotation(axis, angle)
    }

    pub fn rotate_vector(self, v: Vector3) -> Vector3 {
        let u = Vector3::new(self.x, self.y, self.z);
        let s = self.w;

        u * 2.0 * u.dot(v) + v * (s * s - u.dot(u)) + u.cross(v) * 2.0 * s
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 1.0);
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Mul<Self> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;

        Self::new(x, y, z, w)
    }
}

impl MulAssign<Self> for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;
    use super::Quaternion;
    use super::Vector3;

    #[test]
    fn default() {
        assert_eq!(Quaternion::default(), Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn axis_rotation() {
        let q = Quaternion::axis_rotation((1.0, 0.0, 0.0), 90.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn euler_rotation() {
        let q = Quaternion::euler_rotation(90.0, 0.0, 0.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn look_rotation() {
        let q = Quaternion::look_rotation((1.0, 0.0, 0.0), Vector3::UP);
        assert_eq!(q, Quaternion::new(0.0, 0.707_106_77, 0.0, 0.707_106_77));
    }

    #[test]
    fn rotate_vector() {
        let q = Quaternion::axis_rotation((0.0, 1.0, 0.0), 90.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(
            q.rotate_vector(v),
            Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94)
        );

        let q1 = Quaternion::look_rotation((1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        assert_eq!(
            q1.rotate_vector(v),
            Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94)
        );
    }

    #[test]
    fn as_matrix() {
        let m3 = Quaternion::axis_rotation((1.0, 0.0, 0.0), 90.0).as_matrix();
        let m4 = Matrix4::axis_rotation((1.0, 0.0, 0.0), 90.0);
        assert_eq!(m3.col_x, m4.col_x);
        assert_eq!(m3.col_y, m4.col_y);
        assert_eq!(m3.col_z, m4.col_z);
        assert_eq!(m3.col_w, m4.col_w);
    }

    #[test]
    fn operator() {
        let mut q1 = Quaternion::axis_rotation((1.0, 0.0, 0.0), 90.0);
        let r = q1 * q1;
        q1 *= q1;
        assert_eq!(r, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
        assert_eq!(q1, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
    }
}
