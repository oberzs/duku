// Oliver Berzs
// https://github.com/oberzs/duku

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;

use super::Quaternion;
use super::Transform;
use super::Vector3;
use super::Vector4;

/// 4x4 Matrix.
///
/// Used for transforming vectors
///
/// Note: column-major
///
/// # Example
///
/// ```ignore
/// let vector = Vector3::new(2.0, 0.0, 0.0);
/// let matrix = Matrix4::scale([5.0, 1.0, 1.0]);
/// assert_eq!(matrix * vector, Vector3::new(10.0, 0.0, 0.0));
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    /// the X column
    pub x: Vector4,
    /// the Y column
    pub y: Vector4,
    /// the Z column
    pub z: Vector4,
    /// the W column
    pub w: Vector4,
}

impl Matrix4 {
    /// Create matrix from column vectors
    pub fn columns(
        x: impl Into<Vector4>,
        y: impl Into<Vector4>,
        z: impl Into<Vector4>,
        w: impl Into<Vector4>,
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
        x: impl Into<Vector4>,
        y: impl Into<Vector4>,
        z: impl Into<Vector4>,
        w: impl Into<Vector4>,
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
    /// # Example
    ///
    /// ```ignore
    /// let vector = Vector3::new(2.0, 0.0, 0.0);
    /// let matrix = Matrix4::translation([5.0, 1.0, 1.0]);
    /// assert_eq!(matrix * vector, Vector3::new(7.0, 1.0, 1.0));
    /// ```
    pub fn translation(vector: impl Into<Vector3>) -> Self {
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
    /// # Example
    ///
    /// ```ignore
    /// let vector = Vector3::new(2.0, 0.0, 0.0);
    /// let matrix = Matrix4::scale([5.0, 1.0, 1.0]);
    /// assert_eq!(matrix * vector, Vector3::new(10.0, 0.0, 0.0));
    /// ```
    pub fn scale(vector: impl Into<Vector3>) -> Self {
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
    pub fn axis_rotation(axis: impl Into<Vector3>, angle: f32) -> Self {
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
    /// Note: `global_up` is used as a guide to try aligning to
    pub fn look_rotation(dir: impl Into<Vector3>, global_up: impl Into<Vector3>) -> Self {
        let z_axis = dir.into().unit();
        let x_axis = global_up.into().cross(z_axis).unit();
        let y_axis = z_axis.cross(x_axis);

        Self::rows(
            [x_axis.x, x_axis.y, x_axis.z, 0.0],
            [y_axis.x, y_axis.y, y_axis.z, 0.0],
            [z_axis.x, z_axis.y, z_axis.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create perspective projection matrix
    ///
    /// Note: this is a left-handed matrix
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
    /// Note: this is a left-handed matrix
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

    /// Access the X row of the matrix
    pub const fn rx(&self) -> Vector4 {
        Vector4::new(self.x.x, self.y.x, self.z.x, self.w.x)
    }

    /// Access the Y row of the matrix
    pub const fn ry(&self) -> Vector4 {
        Vector4::new(self.x.y, self.y.y, self.z.y, self.w.y)
    }

    /// Access the Z row of the matrix
    pub const fn rz(&self) -> Vector4 {
        Vector4::new(self.x.z, self.y.z, self.z.z, self.w.z)
    }

    /// Access the W row of the matrix
    pub const fn rw(&self) -> Vector4 {
        Vector4::new(self.x.w, self.y.w, self.z.w, self.w.w)
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4;

    fn index(&self, index: usize) -> &Vector4 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Vector4 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = self;
        m.x *= rhs;
        m.y *= rhs;
        m.z *= rhs;
        m.w *= rhs;
        m
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self.rx().dot(rhs);
        let y = self.ry().dot(rhs);
        let z = self.rz().dot(rhs);
        let w = self.rw().dot(rhs);
        Vector4::new(x, y, z, w)
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        (self * Vector4::from((rhs, 1.0))).xyz()
    }
}

impl Mul<Self> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        let w = self * rhs.w;
        Self::columns(x, y, z, w)
    }
}

impl MulAssign<Self> for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Transform> for Matrix4 {
    fn from(t: Transform) -> Self {
        Matrix4::translation(t.position) * Matrix4::from(t.rotation) * Matrix4::scale(t.scale)
    }
}

impl From<Quaternion> for Matrix4 {
    fn from(q: Quaternion) -> Self {
        let angle = 2.0 * q.w.acos().to_degrees();
        let scale = (1.0 - q.w * q.w).sqrt();
        let axis = if scale < 0.001 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(q.x, q.y, q.z) / scale
        };
        Matrix4::axis_rotation(axis, angle)
    }
}

impl From<[f32; 16]> for Matrix4 {
    fn from(m: [f32; 16]) -> Self {
        Self::columns(
            [m[0], m[1], m[2], m[3]],
            [m[4], m[5], m[6], m[7]],
            [m[8], m[9], m[10], m[11]],
            [m[12], m[13], m[14], m[15]],
        )
    }
}

impl Into<[f32; 16]> for Matrix4 {
    fn into(self) -> [f32; 16] {
        [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;
    use super::Quaternion;
    use super::Transform;
    use super::Vector4;

    #[test]
    fn columns() {
        let m = Matrix4::columns(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.x, Vector4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.y, Vector4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.z, Vector4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.w, Vector4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn rows() {
        let m = Matrix4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.x, Vector4::new(1.0, 5.0, 8.0, 4.0));
        assert_eq!(m.y, Vector4::new(2.0, 6.0, 7.0, 3.0));
        assert_eq!(m.z, Vector4::new(3.0, 7.0, 6.0, 2.0));
        assert_eq!(m.w, Vector4::new(4.0, 8.0, 5.0, 1.0));

        assert_eq!(m.rx(), Vector4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.ry(), Vector4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.rz(), Vector4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.rw(), Vector4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn identity() {
        let m = Matrix4::identity();
        assert_eq!(m.rx(), Vector4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vector4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vector4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(m.rw(), Vector4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn translation() {
        let m = Matrix4::translation([3.0, 4.0, 5.0]);
        let v = Vector4::new(6.0, 7.0, 8.0, 1.0);
        assert_eq!(m * v, Vector4::new(9.0, 11.0, 13.0, 1.0));
    }

    #[test]
    fn scale() {
        let m = Matrix4::scale([1.0, 2.0, 3.0]);
        let v = Vector4::new(3.0, 4.0, 5.0, 1.0);
        assert_eq!(m * v, Vector4::new(3.0, 8.0, 15.0, 1.0));
    }

    #[test]
    fn axis_rotation() {
        let m = Matrix4::axis_rotation([1.0, 0.0, 0.0], 180.0);
        let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(m * v, Vector4::new(1.0, -0.999_999_94, -1.000_000_1, 1.0));
    }

    #[test]
    fn perspective() {
        let m = Matrix4::perspective(90.0, 1.0, 0.0, 100.0);
        assert_eq!(m.rx(), Vector4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vector4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vector4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.rw(), Vector4::new(0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn orthographic() {
        let m = Matrix4::orthographic(1.0, 1.0, 0.0, 1.0);
        assert_eq!(m.rx(), Vector4::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vector4::new(0.0, 2.0, 0.0, 0.0));
        assert_eq!(m.rz(), Vector4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.rw(), Vector4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn look_rotation() {
        let m = Matrix4::look_rotation([0.0, 0.0, -1.0], [0.0, 1.0, 0.0]);
        assert_eq!(m.rx(), Vector4::new(-1.0, 0.0, 0.0, -0.0));
        assert_eq!(m.ry(), Vector4::new(0.0, 1.0, 0.0, -0.0));
        assert_eq!(m.rz(), Vector4::new(0.0, 0.0, -1.0, -0.0));
        assert_eq!(m.rw(), Vector4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mul_with_vector() {
        let m = Matrix4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m * v, Vector4::new(30.0, 70.0, 60.0, 20.0));
    }

    #[test]
    fn mul_with_matrix() {
        let mut ma = Matrix4::rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        let mb = Matrix4::rows(
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        let r = ma * mb;
        ma *= mb;
        assert_eq!(r.rx(), Vector4::new(39.0, 43.0, 47.0, 51.0));
        assert_eq!(r.ry(), Vector4::new(111.0, 115.0, 119.0, 123.0));
        assert_eq!(r.rz(), Vector4::new(123.0, 119.0, 115.0, 111.0));
        assert_eq!(r.rw(), Vector4::new(51.0, 47.0, 43.0, 39.0));
        assert_eq!(ma.rx(), Vector4::new(39.0, 43.0, 47.0, 51.0));
        assert_eq!(ma.ry(), Vector4::new(111.0, 115.0, 119.0, 123.0));
        assert_eq!(ma.rz(), Vector4::new(123.0, 119.0, 115.0, 111.0));
        assert_eq!(ma.rw(), Vector4::new(51.0, 47.0, 43.0, 39.0));
    }

    #[test]
    fn inverse() {
        let m = Matrix4::orthographic(20.0, 20.0, 0.1, 50.0);
        let precision = 0.99999994;
        assert_eq!(
            m * m.inverse().expect("no inverse"),
            Matrix4::identity() * precision
        );
    }

    #[test]
    fn projection() {
        let matrix = Matrix4::perspective(90.0, 16.0 / 9.0, 0.1, 10.0);
        let point = Vector4::new(0.0, 0.0, 10.0, 1.0);
        assert_eq!(matrix * point, Vector4::new(0.0, 0.0, 10.000001, 10.0));
    }

    #[test]
    fn from_transform() {
        let mut t = Transform::default();
        t.move_by([1.0, 2.0, 3.0]);
        assert_eq!(Matrix4::from(t), Matrix4::translation([1.0, 2.0, 3.0]));
    }

    #[test]
    fn from_quaternion() {
        let m3 = Matrix4::from(Quaternion::axis_rotation([1.0, 0.0, 0.0], 90.0));
        let m4 = Matrix4::axis_rotation([1.0, 0.0, 0.0], 90.0);
        assert_eq!(m3, m4);
    }
}
