// Oliver Berzs
// https://github.com/oberzs/duku

use super::Matrix3;
use super::Matrix4;
use super::Vector3;

use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
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
        let m = Matrix4::look_rotation(dir, global_up);

        let mut result = Self::default();
        let trace = m.x.x + m.y.y + m.z.z;
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            result.w = 0.25 / s;
            result.x = (m.z.y - m.y.z) * s;
            result.y = (m.x.z - m.z.x) * s;
            result.z = (m.y.x - m.x.y) * s;
        } else if m.x.x > m.y.y && m.x.x > m.z.z {
            let s = 2.0 * (1.0 + m.x.x - m.y.y - m.z.z).sqrt();
            result.w = (m.z.y - m.y.z) / s;
            result.x = 0.25 * s;
            result.y = (m.x.y + m.y.x) / s;
            result.z = (m.x.z + m.z.x) / s;
        } else if m.y.y > m.z.z {
            let s = 2.0 * (1.0 + m.y.y - m.x.x - m.z.z).sqrt();
            result.w = (m.x.z - m.z.x) / s;
            result.x = (m.x.y + m.y.x) / s;
            result.y = 0.25 * s;
            result.z = (m.y.z + m.z.y) / s;
        } else {
            let s = 2.0 * (1.0 + m.z.z - m.x.x - m.y.y).sqrt();
            result.w = (m.y.x - m.x.y) / s;
            result.x = (m.x.z + m.z.x) / s;
            result.y = (m.y.z + m.z.y) / s;
            result.z = 0.25 * s;
        }
        result
    }

    pub fn inverse_rotation(self) -> Quaternion {
        let mut result = self;
        result.w = -result.w;
        result
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

impl From<Matrix3> for Quaternion {
    fn from(m: Matrix3) -> Self {
        fn pos(value: f32) -> f32 {
            if value < 0.0 {
                0.0
            } else {
                value
            }
        }

        let qw = pos(1.0 + m.x.x + m.y.y + m.z.z).sqrt() / 2.0;
        let mut qx = pos(1.0 + m.x.x - m.y.y - m.z.z).sqrt() / 2.0;
        let mut qy = pos(1.0 - m.x.x + m.y.y - m.z.z).sqrt() / 2.0;
        let mut qz = pos(1.0 - m.x.x - m.y.y + m.z.z).sqrt() / 2.0;
        qx = qx.copysign(m.z.y - m.y.z);
        qy = -qy.copysign(m.x.z - m.z.x);
        qz = qz.copysign(m.y.x - m.x.y);

        Self::new(qx, qy, qz, qw)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Quaternion;
    use super::Vector3;

    #[test]
    fn default() {
        assert_eq!(Quaternion::default(), Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn axis_rotation() {
        let q = Quaternion::axis_rotation([1.0, 0.0, 0.0], 90.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn euler_rotation() {
        let q = Quaternion::euler_rotation(90.0, 0.0, 0.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn look_rotation() {
        let q = Quaternion::look_rotation([1.0, 0.0, 0.0], Vector3::UP);
        assert_eq!(q, Quaternion::new(0.0, 0.707_106_77, 0.0, 0.707_106_77));
    }

    #[test]
    fn rotate_vector() {
        let q = Quaternion::axis_rotation([0.0, 1.0, 0.0], 90.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(
            q.rotate_vector(v),
            Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94)
        );

        let q1 = Quaternion::look_rotation([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            q1.rotate_vector(v),
            Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94)
        );
    }

    #[test]
    fn operator() {
        let mut q1 = Quaternion::axis_rotation([1.0, 0.0, 0.0], 90.0);
        let r = q1 * q1;
        q1 *= q1;
        assert_eq!(r, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
        assert_eq!(q1, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
    }

    // #[test]
    // fn from_matrix3() {
    //     use std::f32::consts::FRAC_1_SQRT_2;

    //     let m = Matrix3::from_rows(
    //         [FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2],
    //         [0.5, FRAC_1_SQRT_2, -0.5],
    //         [-0.5, FRAC_1_SQRT_2, 0.5],
    //     );
    //     assert_eq!(
    //         Quaternion::from(m),
    //         Quaternion::new(0.3535534, 0.3535534, 0.1464466, 0.8535534)
    //     );
    // }
}
