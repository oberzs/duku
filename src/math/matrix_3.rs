// Oliver Berzs
// https://github.com/oberzs/draw-it

// 3x3 matrix struct

use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3 {
    pub col_x: Vector3,
    pub col_y: Vector3,
    pub col_z: Vector3,
}

impl Matrix3 {
    pub fn from_columns(
        col_x: impl Into<Vector3>,
        col_y: impl Into<Vector3>,
        col_z: impl Into<Vector3>,
    ) -> Self {
        Self {
            col_x: col_x.into(),
            col_y: col_y.into(),
            col_z: col_z.into(),
        }
    }

    pub fn from_rows(
        row_x: impl Into<Vector3>,
        row_y: impl Into<Vector3>,
        row_z: impl Into<Vector3>,
    ) -> Self {
        let rx = row_x.into();
        let ry = row_y.into();
        let rz = row_z.into();

        Self::from_columns([rx.x, ry.x, rz.x], [rx.y, ry.y, rz.y], [rx.z, ry.z, rz.z])
    }

    pub fn identity() -> Self {
        Self::from_rows([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
    }

    pub fn determinant(&self) -> f32 {
        self.col_x.x * (self.col_y.y * self.col_z.z - self.col_z.y * self.col_y.z)
            - self.col_y.x * (self.col_x.y * self.col_z.z - self.col_z.y * self.col_x.z)
            + self.col_z.x * (self.col_x.y * self.col_y.z - self.col_y.y * self.col_x.z)
    }

    pub const fn row_x(&self) -> Vector3 {
        Vector3::new(self.col_x.x, self.col_y.x, self.col_z.x)
    }

    pub const fn row_y(&self) -> Vector3 {
        Vector3::new(self.col_x.y, self.col_y.y, self.col_z.y)
    }

    pub const fn row_z(&self) -> Vector3 {
        Vector3::new(self.col_x.z, self.col_y.z, self.col_z.z)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix3;
    use super::Vector3;

    #[test]
    fn from_columns() {
        let m = Matrix3::from_columns([1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [8.0, 7.0, 6.0]);
        assert_eq!(m.col_x, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(m.col_y, Vector3::new(5.0, 6.0, 7.0));
        assert_eq!(m.col_z, Vector3::new(8.0, 7.0, 6.0));
    }

    #[test]
    fn from_rows() {
        let m = Matrix3::from_rows([1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [8.0, 7.0, 6.0]);
        assert_eq!(m.col_x, Vector3::new(1.0, 5.0, 8.0));
        assert_eq!(m.col_y, Vector3::new(2.0, 6.0, 7.0));
        assert_eq!(m.col_z, Vector3::new(3.0, 7.0, 6.0));
    }

    #[test]
    fn rows() {
        let m = Matrix3::from_rows([1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [8.0, 7.0, 6.0]);
        assert_eq!(m.row_x(), Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(m.row_y(), Vector3::new(5.0, 6.0, 7.0));
        assert_eq!(m.row_z(), Vector3::new(8.0, 7.0, 6.0));
    }

    #[test]
    fn identity() {
        let m = Matrix3::identity();
        assert_eq!(m.row_x(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(m.row_y(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(m.row_z(), Vector3::new(0.0, 0.0, 1.0));
    }
}
