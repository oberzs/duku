// Oliver Berzs
// https://github.com/oberzs/duku

use super::Matrix4;
use super::Quaternion;
use super::Vector3;

/// Represents transformation in separate components.
///
/// Used as an easier way to operate with transformations
/// than matrices
///
/// # Example
///
/// ```ignore
/// let transform = Transform {
///     position: Vector3::new(1.0, 2.0, 3.0),
///     scale: Vector3::uniform(5.0),
///     rotation: Quaternion::euler_rotation(90.0, 0.0, 45.0),
/// };
/// let matrix = Matrix4::from(transform);
/// // use matrix as usual
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    /// the position component
    pub position: Vector3,
    /// the scale component
    pub scale: Vector3,
    /// the rotation component
    pub rotation: Quaternion,
}

impl Transform {
    /// local up direction for transformation
    pub fn up(self) -> Vector3 {
        self.rotation * Vector3::UP
    }

    /// local forward direction for transformation
    pub fn forward(self) -> Vector3 {
        self.rotation * Vector3::FORWARD
    }

    /// local right direction for transformation
    pub fn right(self) -> Vector3 {
        self.rotation * Vector3::RIGHT
    }

    /// move tranformation by specified amount
    ///
    /// Note: this moves using global directions
    pub fn move_by(&mut self, amount: impl Into<Vector3>) {
        self.position += amount.into();
    }

    /// move transformation up by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_up(&mut self, amount: f32) {
        self.move_by(self.up() * amount);
    }

    /// move transformation down by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_down(&mut self, amount: f32) {
        self.move_by(-self.up() * amount);
    }

    /// move transformation right by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_right(&mut self, amount: f32) {
        self.move_by(self.right() * amount);
    }

    /// move transformation left by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_left(&mut self, amount: f32) {
        self.move_by(-self.right() * amount);
    }

    /// move transformation forward by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_forward(&mut self, amount: f32) {
        self.move_by(self.forward() * amount);
    }

    /// move transformation back by specified amount
    ///
    /// Note: this moves using local directions
    pub fn move_back(&mut self, amount: f32) {
        self.move_by(-self.forward() * amount);
    }

    /// move transformation rotating it around
    /// some point around an axis
    pub fn move_around_point(
        &mut self,
        point: impl Into<Vector3>,
        angle: f32,
        axis: impl Into<Vector3>,
    ) {
        let point = point.into();
        let rotation = Quaternion::axis_rotation(axis, angle);
        self.position -= point;
        self.position = rotation * self.position;
        self.position += point;
    }

    /// rotates the transformation to look at specific direction
    pub fn look_dir(&mut self, dir: impl Into<Vector3>) {
        let dir = dir.into().unit();
        let up = if dir == Vector3::UP {
            Vector3::FORWARD
        } else {
            Vector3::UP
        };
        self.rotation = Quaternion::look_rotation(dir, up);
    }

    /// rotates the transformation to look at specific position
    pub fn look_at(&mut self, pos: impl Into<Vector3>) {
        self.look_dir(pos.into() - self.position);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::default(),
        }
    }
}

impl From<Matrix4> for Transform {
    fn from(mut m: Matrix4) -> Self {
        let position = Vector3::new(m.w.x, m.w.y, m.w.z);

        let determinant = m.x.x * (m.y.y * m.z.z - m.z.y * m.y.z)
            - m.y.x * (m.x.y * m.z.z - m.z.y * m.x.z)
            + m.z.x * (m.x.y * m.y.z - m.y.y * m.x.z);

        let sx = m.x.xyz().length();
        let sy = m.y.xyz().length();
        let sz = m.z.xyz().length() * determinant.signum();
        let scale = Vector3::new(sx, sy, sz);

        m.x *= 1.0 / sx;
        m.y *= 1.0 / sy;
        m.z *= 1.0 / sz;

        let rotation = Quaternion::from(m);

        Self {
            position,
            scale,
            rotation,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Quaternion;
    use super::Transform;
    use super::Vector3;

    #[test]
    fn default() {
        let t = Transform::default();
        assert_eq!(t.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }
}
