// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// 4x4 matrix struct

use std::ops::Mul;
use std::ops::MulAssign;

use super::Vector3;
use super::Vector4;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub col_x: Vector4,
    pub col_y: Vector4,
    pub col_z: Vector4,
    pub col_w: Vector4,
}

impl Matrix4 {
    pub fn from_columns(
        col_x: impl Into<Vector4>,
        col_y: impl Into<Vector4>,
        col_z: impl Into<Vector4>,
        col_w: impl Into<Vector4>,
    ) -> Self {
        Self {
            col_x: col_x.into(),
            col_y: col_y.into(),
            col_z: col_z.into(),
            col_w: col_w.into(),
        }
    }

    pub fn from_rows(
        row_x: impl Into<Vector4>,
        row_y: impl Into<Vector4>,
        row_z: impl Into<Vector4>,
        row_w: impl Into<Vector4>,
    ) -> Self {
        let rx = row_x.into();
        let ry = row_y.into();
        let rz = row_z.into();
        let rw = row_w.into();

        Self::from_columns(
            [rx.x, ry.x, rz.x, rw.x],
            [rx.y, ry.y, rz.y, rw.y],
            [rx.z, ry.z, rz.z, rw.z],
            [rx.w, ry.w, rz.w, rw.w],
        )
    }

    pub fn identity() -> Self {
        Self::from_rows(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn translation(vector: impl Into<Vector3>) -> Self {
        let v = vector.into();
        Self::from_rows(
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn scale(vector: impl Into<Vector3>) -> Self {
        let v = vector.into();
        Self::from_rows(
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

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

        Self::from_rows(row_x, row_y, row_z, row_w)
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        // L-handed and z = [0, 1]
        // Y up, Z forward, center

        let half_fov = (fov / 2.0).to_radians();
        let zoom_len = 1.0 / half_fov.tan();

        let x_scale = zoom_len / aspect;
        let y_scale = zoom_len;
        let z_scale = far / (far - near);
        let z_move = -(near * far) / (far - near);

        let copy = 1.0;

        Self::from_rows(
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, -y_scale, 0.0, 0.0],
            [0.0, 0.0, z_scale, z_move],
            [0.0, 0.0, copy, 0.0],
        )
    }

    pub fn orthographic_center(width: f32, height: f32, near: f32, far: f32) -> Self {
        // L-handed and z = [0, 1]
        // Y up, Z forward, center

        let x_scale = 2.0 / width;
        let y_scale = 2.0 / height;
        let z_scale = 1.0 / (far - near);
        let z_move = -near / (far - near);

        Self::from_rows(
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, -y_scale, 0.0, 0.0],
            [0.0, 0.0, z_scale, z_move],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
        // L-handed and z = [0, 1]
        // Y down, Z forward, top-left

        let x_scale = 2.0 / width;
        let y_scale = 2.0 / height;
        let z_scale = 1.0 / (far - near);
        let z_move = -near / (far - near);

        Self::from_rows(
            [x_scale, 0.0, 0.0, -1.0],
            [0.0, y_scale, 0.0, -1.0],
            [0.0, 0.0, z_scale, z_move],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn look_rotation(dir: impl Into<Vector3>, global_up: impl Into<Vector3>) -> Self {
        let z_axis = dir.into().unit();
        let x_axis = global_up.into().cross(z_axis).unit();
        let y_axis = z_axis.cross(x_axis);

        Self::from_rows(
            [x_axis.x, x_axis.y, x_axis.z, 0.0],
            [y_axis.x, y_axis.y, y_axis.z, 0.0],
            [z_axis.x, z_axis.y, z_axis.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn row_x(self) -> Vector4 {
        Vector4::new(self.col_x.x, self.col_y.x, self.col_z.x, self.col_w.x)
    }

    pub fn row_y(self) -> Vector4 {
        Vector4::new(self.col_x.y, self.col_y.y, self.col_z.y, self.col_w.y)
    }

    pub fn row_z(self) -> Vector4 {
        Vector4::new(self.col_x.z, self.col_y.z, self.col_z.z, self.col_w.z)
    }

    pub fn row_w(self) -> Vector4 {
        Vector4::new(self.col_x.w, self.col_y.w, self.col_z.w, self.col_w.w)
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        let x = self.row_x().dot(rhs);
        let y = self.row_y().dot(rhs);
        let z = self.row_z().dot(rhs);
        let w = self.row_w().dot(rhs);
        [x, y, z, w].into()
    }
}

impl Mul<Self> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let col_x = self * rhs.col_x;
        let col_y = self * rhs.col_y;
        let col_z = self * rhs.col_z;
        let col_w = self * rhs.col_w;
        Self::from_columns(col_x, col_y, col_z, col_w)
    }
}

impl MulAssign<Self> for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;
    use super::Vector4;

    #[test]
    fn from_columns() {
        let m = Matrix4::from_columns(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.col_x, Vector4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.col_y, Vector4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.col_z, Vector4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.col_w, Vector4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn from_rows() {
        let m = Matrix4::from_rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.col_x, Vector4::new(1.0, 5.0, 8.0, 4.0));
        assert_eq!(m.col_y, Vector4::new(2.0, 6.0, 7.0, 3.0));
        assert_eq!(m.col_z, Vector4::new(3.0, 7.0, 6.0, 2.0));
        assert_eq!(m.col_w, Vector4::new(4.0, 8.0, 5.0, 1.0));
    }

    #[test]
    fn rows() {
        let m = Matrix4::from_rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_eq!(m.row_x(), Vector4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(m.row_y(), Vector4::new(5.0, 6.0, 7.0, 8.0));
        assert_eq!(m.row_z(), Vector4::new(8.0, 7.0, 6.0, 5.0));
        assert_eq!(m.row_w(), Vector4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[test]
    fn identity() {
        let m = Matrix4::identity();
        assert_eq!(m.row_x(), Vector4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.row_y(), Vector4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(m.row_z(), Vector4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(m.row_w(), Vector4::new(0.0, 0.0, 0.0, 1.0));
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
        assert_eq!(m.row_x(), Vector4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(m.row_y(), Vector4::new(0.0, -1.0, 0.0, 0.0));
        assert_eq!(m.row_z(), Vector4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.row_w(), Vector4::new(0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn orthographic() {
        let m = Matrix4::orthographic(1.0, 1.0, 0.0, 1.0);
        assert_eq!(m.row_x(), Vector4::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(m.row_y(), Vector4::new(0.0, -2.0, 0.0, 0.0));
        assert_eq!(m.row_z(), Vector4::new(0.0, 0.0, 1.0, -0.0));
        assert_eq!(m.row_w(), Vector4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn look_rotation() {
        let m = Matrix4::look_rotation([0.0, 0.0, -1.0], [0.0, 1.0, 0.0]);
        assert_eq!(m.row_x(), Vector4::new(-1.0, 0.0, 0.0, -0.0));
        assert_eq!(m.row_y(), Vector4::new(0.0, 1.0, 0.0, -0.0));
        assert_eq!(m.row_z(), Vector4::new(0.0, 0.0, -1.0, -0.0));
        assert_eq!(m.row_w(), Vector4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mul_with_vector() {
        let m = Matrix4::from_rows(
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
        let mut ma = Matrix4::from_rows(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        let mb = Matrix4::from_rows(
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        let r = ma * mb;
        ma *= mb;
        assert_eq!(r.row_x(), Vector4::new(39.0, 43.0, 47.0, 51.0));
        assert_eq!(r.row_y(), Vector4::new(111.0, 115.0, 119.0, 123.0));
        assert_eq!(r.row_z(), Vector4::new(123.0, 119.0, 115.0, 111.0));
        assert_eq!(r.row_w(), Vector4::new(51.0, 47.0, 43.0, 39.0));
        assert_eq!(ma.row_x(), Vector4::new(39.0, 43.0, 47.0, 51.0));
        assert_eq!(ma.row_y(), Vector4::new(111.0, 115.0, 119.0, 123.0));
        assert_eq!(ma.row_z(), Vector4::new(123.0, 119.0, 115.0, 111.0));
        assert_eq!(ma.row_w(), Vector4::new(51.0, 47.0, 43.0, 39.0));
    }
}
