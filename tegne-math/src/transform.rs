use super::Matrix4;
use super::Quaternion;
use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub rotation: Quaternion,
}

pub struct TransformBuilder {
    position: Vector3,
    scale: Vector3,
    rotation: Quaternion,
}

impl Transform {
    pub fn builder() -> TransformBuilder {
        TransformBuilder {
            position: Vector3::default(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::default(),
        }
    }

    pub fn as_matrix(self) -> Matrix4 {
        Matrix4::translation(self.position) * Matrix4::scale(self.scale) * self.rotation.as_matrix()
    }

    pub fn as_matrix_for_camera(self) -> Matrix4 {
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

impl TransformBuilder {
    pub fn with_position(mut self, position: impl Into<Vector3>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_scale(mut self, scale: impl Into<Vector3>) -> Self {
        self.scale = scale.into();
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn build(self) -> Transform {
        Transform {
            position: self.position,
            scale: self.scale,
            rotation: self.rotation,
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
    fn builder() {
        let t = Transform::builder().build();
        assert_eq!(t.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn builder_with_position() {
        let t = Transform::builder().with_position([1.0, 2.0, 3.0]).build();
        assert_eq!(t.position, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn builder_with_scale() {
        let t = Transform::builder().with_scale([2.0, 3.0, 4.0]).build();
        assert_eq!(t.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(t.scale, Vector3::new(2.0, 3.0, 4.0));
        assert_eq!(t.rotation, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn builder_with_rotation() {
        let t = Transform::builder()
            .with_rotation(Quaternion::euler_rotation(0.0, 60.0, 90.0))
            .build();
        assert_eq!(t.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(t.scale, Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(t.rotation, Quaternion::euler_rotation(0.0, 60.0, 90.0));
    }

    #[test]
    fn as_matrix() {
        let t = Transform::builder().with_position([1.0, 2.0, 3.0]).build();
        assert_eq!(t.as_matrix(), Matrix4::translation([1.0, 2.0, 3.0]));
    }

    #[test]
    fn as_matrix_for_camera() {
        let t = Transform::builder().with_position([1.0, 2.0, 3.0]).build();
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
        let t = Transform::builder().with_position([1.0, 0.0, 0.0]).build();
        assert_eq!(t.up(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(t.right(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(t.forward(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn move_by() {
        let mut t = Transform::default();
        t.move_by([1.0, 2.0, 3.0]);
        assert_eq!(
            t,
            Transform::builder().with_position([1.0, 2.0, 3.0]).build()
        );
    }
}
