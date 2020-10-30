// Oliver Berzs
// https://github.com/oberzs/draw-it

// represents a whole transform. position + scale + rotation

use super::Matrix3;
use super::Matrix4;
use super::Quaternion;
use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub const fn positioned(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            scale: Vector3::uniform(1.0),
            rotation: Quaternion::ZERO,
        }
    }

    pub const fn scaled(x: f32, y: f32, z: f32) -> Self {
        Self {
            scale: Vector3::new(x, y, z),
            position: Vector3::ZERO,
            rotation: Quaternion::ZERO,
        }
    }

    pub const fn scaled_uniformly(s: f32) -> Self {
        Self::scaled(s, s, s)
    }

    pub fn rotated(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self {
            rotation: Quaternion::euler_rotation(roll, pitch, yaw),
            ..Default::default()
        }
    }

    pub fn up(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::UP)
    }

    pub fn forward(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::FORWARD)
    }

    pub fn right(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::RIGHT)
    }

    pub fn move_by(&mut self, amount: impl Into<Vector3>) {
        self.position += amount.into();
    }

    pub fn move_up(&mut self, amount: f32) {
        self.move_by(self.up() * amount);
    }

    pub fn move_down(&mut self, amount: f32) {
        self.move_by(-self.up() * amount);
    }

    pub fn move_right(&mut self, amount: f32) {
        self.move_by(self.right() * amount);
    }

    pub fn move_left(&mut self, amount: f32) {
        self.move_by(-self.right() * amount);
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.move_by(self.forward() * amount);
    }

    pub fn move_backward(&mut self, amount: f32) {
        self.move_by(-self.forward() * amount);
    }

    pub fn move_around_point(
        &mut self,
        point: impl Into<Vector3>,
        angle: f32,
        axis: impl Into<Vector3>,
    ) {
        let point = point.into();
        let rotation = Quaternion::axis_rotation(axis, angle);
        self.position -= point;
        self.position = rotation.rotate_vector(self.position);
        self.position += point;
    }

    pub fn look_dir(&mut self, dir: impl Into<Vector3>) {
        let dir = dir.into().unit();
        let up = if dir == Vector3::UP {
            Vector3::FORWARD
        } else {
            Vector3::UP
        };
        self.rotation = Quaternion::look_rotation(dir, up);
    }

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
    fn from(m: Matrix4) -> Self {
        let position = Vector3::new(m.col_w.x, m.col_w.y, m.col_w.z);

        let mut i = Matrix3::from_columns(m.col_x.shrink(), m.col_y.shrink(), m.col_z.shrink());

        let sx = i.col_x.length();
        let sy = i.col_y.length();
        let sz = i.col_z.length() * i.determinant().signum();
        let scale = Vector3::new(sx, sy, sz);

        i.col_x *= 1.0 / sx;
        i.col_y *= 1.0 / sy;
        i.col_z *= 1.0 / sz;

        let rotation = Quaternion::from(i);

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

    #[test]
    fn from_position() {
        let t = Transform::positioned(1.0, 2.0, 3.0);
        assert_eq!(t.position, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn direction() {
        let t = Transform::positioned(1.0, 0.0, 0.0);
        assert_eq!(t.up(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(t.right(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(t.forward(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn move_by() {
        let mut t = Transform::default();
        t.move_by([1.0, 2.0, 3.0]);
        assert_eq!(t, Transform::positioned(1.0, 2.0, 3.0));
    }
}
