// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// represents a whole transform. position + scale + rotation

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
    pub fn positioned(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            ..Default::default()
        }
    }

    pub fn scaled(x: f32, y: f32, z: f32) -> Self {
        Self {
            scale: Vector3::new(x, y, z),
            ..Default::default()
        }
    }

    pub fn scaled_uniformly(s: f32) -> Self {
        Self::scaled(s, s, s)
    }

    pub fn rotated(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self {
            rotation: Quaternion::euler_rotation(roll, pitch, yaw),
            ..Default::default()
        }
    }

    pub fn as_matrix(self) -> Matrix4 {
        Matrix4::translation(self.position) * Matrix4::scale(self.scale) * self.rotation.as_matrix()
    }

    pub(crate) fn as_matrix_for_camera(self) -> Matrix4 {
        Matrix4::scale(self.scale)
            * self.rotation.inverse_rotation().as_matrix()
            * Matrix4::translation(-self.position)
    }

    pub fn up(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::up())
    }

    pub fn forward(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::forward())
    }

    pub fn right(self) -> Vector3 {
        self.rotation.rotate_vector(Vector3::right())
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
        _point: impl Into<Vector3>,
        angle: f32,
        axis: impl Into<Vector3>,
    ) {
        let rotation = Quaternion::axis_rotation(axis, angle);
        self.position = rotation.rotate_vector(self.position);
    }

    pub fn look_in_dir(&mut self, dir: impl Into<Vector3>, global_up: impl Into<Vector3>) {
        self.rotation = Quaternion::look_rotation(dir.into().unit(), global_up);
    }

    pub fn look_at(&mut self, pos: impl Into<Vector3>, up: impl Into<Vector3>) {
        self.look_in_dir(pos.into() - self.position, up);
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

impl From<[f32; 3]> for Transform {
    fn from(position: [f32; 3]) -> Self {
        Self {
            position: Vector3::new(position[0], position[1], position[2]),
            ..Default::default()
        }
    }
}

impl From<Vector3> for Transform {
    fn from(position: Vector3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;
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
        let t = Transform::from([1.0, 2.0, 3.0]);
        assert_eq!(t.position, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn as_matrix() {
        let t = Transform::from([1.0, 2.0, 3.0]);
        assert_eq!(t.as_matrix(), Matrix4::translation([1.0, 2.0, 3.0]));
    }

    #[test]
    fn as_matrix_for_camera() {
        let t = Transform::from([1.0, 2.0, 3.0]);
        assert_eq!(
            t.as_matrix_for_camera(),
            Matrix4::from_rows(
                [1.0, 0.0, 0.0, -1.0],
                [0.0, 1.0, -0.00000017484555, -1.9999995],
                [0.0, 0.00000017484555, 1.0, -3.0000002],
                [0.0, 0.0, 0.0, 1.0]
            )
        );
    }

    #[test]
    fn direction() {
        let t = Transform::from([1.0, 0.0, 0.0]);
        assert_eq!(t.up(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(t.right(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(t.forward(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn move_by() {
        let mut t = Transform::default();
        t.move_by([1.0, 2.0, 3.0]);
        assert_eq!(t, Transform::from([1.0, 2.0, 3.0]));
    }
}
