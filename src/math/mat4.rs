// Oliver Berzs
// https://github.com/oberzs/duku

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;

use super::Quat;
use super::Vec3;
use super::Vec4;

/// 4x4 Matrix.
///
/// Used for transforming vectors
///
/// Is column-major
///
/// # Examples
///
/// ```ignore
/// let vector = Vec3::new(2.0, 0.0, 0.0);
/// let matrix = Mat4::scale([5.0, 1.0, 1.0]);
/// assert_eq!(matrix * vector, Vec3::new(10.0, 0.0, 0.0));
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    /// the X column
    pub x: Vec4,
    /// the Y column
    pub y: Vec4,
    /// the Z column
    pub z: Vec4,
    /// the W column
    pub w: Vec4,
}

impl Mat4 {
    /// Create matrix from column vectors
    pub fn columns(
        x: impl Into<Vec4>,
        y: impl Into<Vec4>,
        z: impl Into<Vec4>,
        w: impl Into<Vec4>,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    /// Create matrix from row vectors
    pub fn rows(
        x: impl Into<Vec4>,
        y: impl Into<Vec4>,
        z: impl Into<Vec4>,
        w: impl Into<Vec4>,
    ) -> Self {
        let rx = x.into();
        let ry = y.into();
        let rz = z.into();
        let rw = w.into();

        Self::columns(
            [rx.x, ry.x, rz.x, rw.x],
            [rx.y, ry.y, rz.y, rw.y],
            [rx.z, ry.z, rz.z, rw.z],
            [rx.w, ry.w, rz.w, rw.w],
        )
    }

    /// Create identity matrix
    pub fn identity() -> Self {
        Self::rows(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create translation matrix
    ///
    /// Translation matrix moves vectors around
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let vector = Vec3::new(2.0, 0.0, 0.0);
    /// let matrix = Mat4::translation([5.0, 1.0, 1.0]);
    /// assert_eq!(matrix * vector, Vec3::new(7.0, 1.0, 1.0));
    /// ```
    pub fn translation(vector: impl Into<Vec3>) -> Self {
        let v = vector.into();
        Self::rows(
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create scale matrix
    ///
    /// Scale matrix scales vectors
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let vector = Vec3::new(2.0, 0.0, 0.0);
    /// let matrix = Mat4::scale([5.0, 1.0, 1.0]);
    /// assert_eq!(matrix * vector, Vec3::new(10.0, 0.0, 0.0));
    /// ```
    pub fn scale(vector: impl Into<Vec3>) -> Self {
        let v = vector.into();
        Self::rows(
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create rotation matrix with euler angles
    ///
    /// This rotation's yaw, pitch and roll are z, y and x
    pub fn euler_rotation(x: f32, y: f32, z: f32) -> Self {
        let sin_a = z.to_radians().sin();
        let cos_a = z.to_radians().cos();
        let sin_b = y.to_radians().sin();
        let cos_b = y.to_radians().cos();
        let sin_g = x.to_radians().sin();
        let cos_g = x.to_radians().cos();

        let xx = cos_a * cos_b;
        let xy = cos_a * sin_b * sin_g - sin_a * cos_g;
        let xz = cos_a * sin_b * cos_g + sin_a * sin_g;

        let yx = sin_a * cos_b;
        let yy = sin_a * sin_b * sin_g + cos_a * cos_g;
        let yz = sin_a * sin_b * cos_g - cos_a * sin_g;

        let zx = -sin_b;
        let zy = cos_b * sin_g;
        let zz = cos_b * cos_g;

        Self::rows(
            [xx, xy, xz, 0.0],
            [yx, yy, yz, 0.0],
            [zx, zy, zz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create rotation matrix around axis
    ///
    /// This rotates vectors around axis by the angle
    pub fn axis_rotation(axis: impl Into<Vec3>, angle: f32) -> Self {
        let v = axis.into();
        let sin = angle.to_radians().sin();
        let cos = angle.to_radians().cos();

        let row_x = [
            v.x * v.x * (1.0 - cos) + cos,
            v.x * v.y * (1.0 - cos) - v.z * sin,
            v.x * v.z * (1.0 - cos) + v.y * sin,
            0.0,
        ];
        let row_y = [
            v.x * v.y * (1.0 - cos) + v.z * sin,
            v.y * v.y * (1.0 - cos) + cos,
            v.y * v.z * (1.0 - cos) - v.x * sin,
            0.0,
        ];
        let row_z = [
            v.x * v.z * (1.0 - cos) - v.y * sin,
            v.y * v.z * (1.0 - cos) + v.x * sin,
            v.z * v.z * (1.0 - cos) + cos,
            0.0,
        ];
        let row_w = [0.0, 0.0, 0.0, 1.0];

        Self::rows(row_x, row_y, row_z, row_w)
    }

    /// Create rotation matrix to rotate towards direction
    ///
    /// `up` is used as a guide to try aligning to
    pub fn look_rotation(forward: impl Into<Vec3>, up: impl Into<Vec3>) -> Self {
        let f = forward.into().unit();
        let r = up.into().unit().cross(f).unit();
        let u = f.cross(r).unit();

        // Self::rows(
        //     [r.x, u.x, f.x, 0.0],
        //     [r.y, u.y, f.y, 0.0],
        //     [r.z, u.z, f.z, 0.0],
        //     [0.0, 0.0, 0.0, 1.0],
        // )

        Self::rows(
            [r.x, r.y, r.z, 0.0],
            [u.x, u.y, u.z, 0.0],
            [f.x, f.y, f.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create perspective projection matrix
    ///
    /// This is a left-handed matrix
    /// with Z in range of [0; 1]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let half_fov = (fov / 2.0).to_radians();
        let zoom_len = 1.0 / half_fov.tan();

        let x_scale = zoom_len / aspect;
        let y_scale = zoom_len;
        let z_scale = far / (far - near);
        let z_move = -(near * far) / (far - near);

        let copy = 1.0;

        Self::rows(
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, y_scale, 0.0, 0.0],
            [0.0, 0.0, z_scale, z_move],
            [0.0, 0.0, copy, 0.0],
        )
    }

    /// Create orthographic projection matrix
    ///
    /// This is a left-handed matrix
    /// with Z in range of [0; 1]
    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
        let x_scale = 2.0 / width;
        let y_scale = 2.0 / height;
        let z_scale = 1.0 / (far - near);
        let z_move = -near / (far - near);

        Self::rows(
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, y_scale, 0.0, 0.0],
            [0.0, 0.0, z_scale, z_move],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create matrix from position, scale and rotation
    ///
    /// Oposite of [decompose](crate::math::Mat4::decompose).
    pub fn compose(position: Vec3, scale: Vec3, rotation: Quat) -> Self {
        Mat4::translation(position) * Mat4::from(rotation) * Mat4::scale(scale)
    }

    /// Calculate the inverse of the matrix
    pub fn inverse(&self) -> Option<Self> {
        let m: [f32; 16] = (*self).into();
        let mut inv = [0.0; 16];

        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];

        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];

        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];

        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];

        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
            - m[9] * m[3] * m[14]
            - m[13] * m[2] * m[11]
            + m[13] * m[3] * m[10];

        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
            + m[8] * m[3] * m[14]
            + m[12] * m[2] * m[11]
            - m[12] * m[3] * m[10];

        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
            - m[8] * m[3] * m[13]
            - m[12] * m[1] * m[11]
            + m[12] * m[3] * m[9];

        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
            + m[8] * m[2] * m[13]
            + m[12] * m[1] * m[10]
            - m[12] * m[2] * m[9];

        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
            + m[5] * m[3] * m[14]
            + m[13] * m[2] * m[7]
            - m[13] * m[3] * m[6];

        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
            - m[4] * m[3] * m[14]
            - m[12] * m[2] * m[7]
            + m[12] * m[3] * m[6];

        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
            + m[4] * m[3] * m[13]
            + m[12] * m[1] * m[7]
            - m[12] * m[3] * m[5];

        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
            - m[4] * m[2] * m[13]
            - m[12] * m[1] * m[6]
            + m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
            - m[5] * m[3] * m[10]
            - m[9] * m[2] * m[7]
            + m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
            + m[4] * m[3] * m[10]
            + m[8] * m[2] * m[7]
            - m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
            - m[4] * m[3] * m[9]
            - m[8] * m[1] * m[7]
            + m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
            + m[4] * m[2] * m[9]
            + m[8] * m[1] * m[6]
            - m[8] * m[2] * m[5];

        let mut det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == 0.0 {
            return None;
        }

        det = 1.0 / det;

        Some(Self::from(inv) * det)
    }

    /// Separate translation, scale and rotation parts of the matrix
    pub fn decompose(mut self) -> (Vec3, Vec3, Quat) {
        let position = Vec3::new(self.w.x, self.w.y, self.w.z);

        let determinant = self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.y.x * (self.x.y * self.z.z - self.z.y * self.x.z)
            + self.z.x * (self.x.y * self.y.z - self.y.y * self.x.z);

        let sx = self.x.xyz().length();
        let sy = self.y.xyz().length();
        let sz = self.z.xyz().length() * determinant.signum();
        let scale = Vec3::new(sx, sy, sz);

        self.x *= 1.0 / sx;
        self.y *= 1.0 / sy;
        self.z *= 1.0 / sz;

        let rotation = Quat::from(self);

        (position, scale, rotation)
    }

    /// Access the X row of the matrix
    pub const fn rx(&self) -> Vec4 {
        Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x)
    }

    /// Access the Y row of the matrix
    pub const fn ry(&self) -> Vec4 {
        Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y)
    }

    /// Access the Z row of the matrix
    pub const fn rz(&self) -> Vec4 {
        Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z)
    }

    /// Access the W row of the matrix
    pub const fn rw(&self) -> Vec4 {
        Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w)
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Vec4 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Vec4 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = self;
        m.x *= rhs;
        m.y *= rhs;
        m.z *= rhs;
        m.w *= rhs;
        m
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let x = self.rx().dot(rhs);
        let y = self.ry().dot(rhs);
        let z = self.rz().dot(rhs);
        let w = self.rw().dot(rhs);
        Vec4::new(x, y, z, w)
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        (self * Vec4::from((rhs, 1.0))).xyz()
    }
}

impl Mul<Self> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        let w = self * rhs.w;
        Self::columns(x, y, z, w)
    }
}

impl MulAssign<Self> for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Quat> for Mat4 {
    fn from(q: Quat) -> Self {
        let angle = 2.0 * q.w.acos().to_degrees();
        let scale = (1.0 - q.w * q.w).sqrt();
        let axis = if scale < 0.001 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(q.x, q.y, q.z) / scale
        };
        Mat4::axis_rotation(axis, angle)
    }
}

impl From<[f32; 16]> for Mat4 {
    fn from(m: [f32; 16]) -> Self {
        Self::columns(
            [m[0], m[1], m[2], m[3]],
            [m[4], m[5], m[6], m[7]],
            [m[8], m[9], m[10], m[11]],
            [m[12], m[13], m[14], m[15]],
        )
    }
}

impl Into<[f32; 16]> for Mat4 {
    fn into(self) -> [f32; 16] {
        [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::Mat4;
    use super::Quat;
    use super::Vec3;
    use super::Vec4;

    #[test]
    fn columns() {
        let m = Mat4::columns(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.x, Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.y, Vec4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.z, Vec4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.w, Vec4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn rows() {
        let m = Mat4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.x, Vec4::new(1.0, 5.0, 8.0, 4.0));
        assert_eq!(m.y, Vec4::new(2.0, 6.0, 7.0, 3.0));
        assert_eq!(m.z, Vec4::new(3.0, 7.0, 6.0, 2.0));
        assert_eq!(m.w, Vec4::new(4.0, 8.0, 5.0, 1.0));

        assert_eq!(m.rx(), Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.ry(), Vec4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.rz(), Vec4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.rw(), Vec4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn identity() {
        let m = Mat4::identity();
        assert_eq!(m.rx(), Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vec4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(m.rw(), Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn translation() {
        let m = Mat4::translation([3.0, 4.0, 5.0]);
        let v = Vec3::new(6.0, 7.0, 8.0);
        assert_eq!(m * v, Vec3::new(9.0, 11.0, 13.0));
    }

    #[test]
    fn scale() {
        let m = Mat4::scale([1.0, 2.0, 3.0]);
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(m * v, Vec3::new(3.0, 8.0, 15.0));
    }

    #[test]
    fn axis_rotation() {
        let m = Mat4::axis_rotation([1.0, 0.0, 0.0], 180.0);
        let v = Vec3::new(1.0, 1.0, 1.0);
        let r = m * v;
        assert_eq_delta!(r.x, 1.0);
        assert_eq_delta!(r.y, -1.0);
        assert_eq_delta!(r.z, -1.0);
    }

    #[test]
    fn look_rotation_x() {
        let m = Mat4::look_rotation(Vec3::new(1.0, 0.0, 0.0), Vec3::up());
        let r = m * Vec3::forward();
        assert_eq_delta!(r.x, -1.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn look_rotation_y() {
        let m = Mat4::look_rotation([0.0, 1.0, 0.0], Vec3::forward());
        let r = m * Vec3::forward();
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn look_rotation_z() {
        let m = Mat4::look_rotation([0.0, 0.0, -1.0], Vec3::up());
        let r = m * Vec3::forward();
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, -1.0);
    }

    #[test]
    fn euler_rotation_x() {
        let m = Mat4::euler_rotation(90.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 0.0, 1.0);
        let r = m * v;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, -1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn euler_rotation_y() {
        let m = Mat4::euler_rotation(0.0, 90.0, 0.0);
        let v = Vec3::new(0.0, 0.0, 1.0);
        let r = m * v;
        assert_eq_delta!(r.x, 1.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn euler_rotation_z() {
        let m = Mat4::euler_rotation(0.0, 0.0, 90.0);
        let v = Vec3::new(1.0, 0.0, 0.0);
        let r = m * v;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn perspective() {
        let m = Mat4::perspective(90.0, 1.0, 0.0, 100.0);
        assert_eq!(m.rx(), Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vec4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.rw(), Vec4::new(0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn orthographic() {
        let m = Mat4::orthographic(1.0, 1.0, 0.0, 1.0);
        assert_eq!(m.rx(), Vec4::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vec4::new(0.0, 2.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vec4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.rw(), Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mul_with_vector() {
        let m = Mat4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m * v, Vec4::new(30.0, 70.0, 60.0, 20.0));
    }

    #[test]
    fn mul_with_self() {
        let ma = Mat4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        let mb = Mat4::rows(
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        let r = ma * mb;
        assert_eq!(r.rx(), Vec4::new(39.0, 43.0, 47.0, 51.0));
        assert_eq!(r.ry(), Vec4::new(111.0, 115.0, 119.0, 123.0));
        assert_eq!(r.rz(), Vec4::new(123.0, 119.0, 115.0, 111.0));
        assert_eq!(r.rw(), Vec4::new(51.0, 47.0, 43.0, 39.0));
    }

    #[test]
    fn inverse() {
        let m = Mat4::orthographic(20.0, 20.0, 0.1, 50.0);
        let r = m * m.inverse().expect("no inverse");
        assert_eq_delta!(r.x.x, 1.0);
        assert_eq_delta!(r.y.y, 1.0);
        assert_eq_delta!(r.z.z, 1.0);
        assert_eq_delta!(r.w.w, 1.0);
    }

    #[test]
    fn projection() {
        let matrix = Mat4::perspective(90.0, 16.0 / 9.0, 0.1, 10.0);
        let point = Vec4::new(0.0, 0.0, 10.0, 1.0);
        let r = matrix * point;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, 10.0);
        assert_eq_delta!(r.w, 10.0);
    }

    #[test]
    fn compose() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let scale = Vec3::new(1.0, 1.0, 1.0);
        let rotation = Quat::default();
        assert_eq!(
            Mat4::compose(position, scale, rotation),
            Mat4::translation([1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn from_quaternion() {
        let v = Vec3::new(0.0, 0.0, 1.0);
        let mq = Mat4::from(Quat::euler_rotation(90.0, 0.0, 0.0));
        let m = Mat4::euler_rotation(90.0, 0.0, 0.0);

        let rq = mq * v;
        let r = m * v;

        assert_eq_delta!(rq.x, 0.0);
        assert_eq_delta!(rq.y, -1.0);
        assert_eq_delta!(rq.z, 0.0);

        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, -1.0);
        assert_eq_delta!(r.z, 0.0);
    }
}
