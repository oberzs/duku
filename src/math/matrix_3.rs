// Oliver Berzs
// https://github.com/oberzs/duku

use std::ops::Index;
use std::ops::IndexMut;

use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn columns(x: impl Into<Vector3>, y: impl Into<Vector3>, z: impl Into<Vector3>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn rows(x: impl Into<Vector3>, y: impl Into<Vector3>, z: impl Into<Vector3>) -> Self {
        let rx = x.into();
        let ry = y.into();
        let rz = z.into();

        Self::columns([rx.x, ry.x, rz.x], [rx.y, ry.y, rz.y], [rx.z, ry.z, rz.z])
    }

    pub fn identity() -> Self {
        Self::rows([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
    }

    pub fn determinant(&self) -> f32 {
        self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.y.x * (self.x.y * self.z.z - self.z.y * self.x.z)
            + self.z.x * (self.x.y * self.y.z - self.y.y * self.x.z)
    }

    pub const fn rx(&self) -> Vector3 {
        Vector3::new(self.x.x, self.y.x, self.z.x)
    }

    pub const fn ry(&self) -> Vector3 {
        Vector3::new(self.x.y, self.y.y, self.z.y)
    }

    pub const fn rz(&self) -> Vector3 {
        Vector3::new(self.x.z, self.y.z, self.z.z)
    }
}

impl Index<usize> for Matrix3 {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Vector3 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Vector3 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Matrix3;
    use super::Vector3;

    #[test]
    fn columns() {
        let m = Matrix3::columns([1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [8.0, 7.0, 6.0]);
        assert_eq!(m.x, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(m.y, Vector3::new(5.0, 6.0, 7.0));
        assert_eq!(m.z, Vector3::new(8.0, 7.0, 6.0));
    }

    #[test]
    fn rows() {
        let m = Matrix3::rows([1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [8.0, 7.0, 6.0]);
        assert_eq!(m.x, Vector3::new(1.0, 5.0, 8.0));
        assert_eq!(m.y, Vector3::new(2.0, 6.0, 7.0));
        assert_eq!(m.z, Vector3::new(3.0, 7.0, 6.0));

        assert_eq!(m.rx(), Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(m.ry(), Vector3::new(5.0, 6.0, 7.0));
        assert_eq!(m.rz(), Vector3::new(8.0, 7.0, 6.0));
    }

    #[test]
    fn identity() {
        let m = Matrix3::identity();
        assert_eq!(m.rx(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(m.ry(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(m.rz(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn determinant() {
        let m = Matrix3::rows([2.0, 3.0, 1.0], [7.0, 2.0, 3.0], [1.0, 8.0, 4.0]);
        assert_eq!(m.determinant(), -53.0);
    }
}
