// Oliver Berzs
// https://github.com/oberzs/draw-it

// editor-like camera controller

use super::Key;
use super::MouseButton;
use super::Window;
use crate::math::Quaternion;
use crate::math::Vector3;
use crate::renderer::Camera;

#[derive(Debug, Copy, Clone)]
pub enum Controller {
    Fly { camera_angle: f32, move_speed: f32 },
    Orbit { pivot: Vector3, move_speed: f32 },
}

impl Controller {
    pub const fn fly() -> Self {
        Self::Fly {
            camera_angle: 0.0,
            move_speed: 1.0,
        }
    }

    pub fn orbit(pivot: impl Into<Vector3>) -> Self {
        Self::Orbit {
            pivot: pivot.into(),
            move_speed: 2.5,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, window: &mut Window, delta_time: f32) {
        match self {
            Self::Fly {
                camera_angle,
                move_speed,
            } => {
                // update move speed
                if window.is_key_typed(Key::Equal) {
                    *move_speed += 0.5;
                }
                if window.is_key_typed(Key::Minus) {
                    *move_speed -= 0.5;
                }

                // control in flying mode
                let transform = &mut camera.transform;
                let final_speed = 5.0f32.powf(*move_speed) * delta_time;
                let rotation_speed = 50.0 * delta_time;

                // movement
                if window.is_key_pressed(Key::W) {
                    transform.move_forward(final_speed);
                }
                if window.is_key_pressed(Key::S) {
                    transform.move_backward(final_speed);
                }
                if window.is_key_pressed(Key::A) {
                    transform.move_left(final_speed);
                }
                if window.is_key_pressed(Key::D) {
                    transform.move_right(final_speed);
                }
                if window.is_key_pressed(Key::Space) {
                    transform.move_by(Vector3::UP * final_speed);
                }
                if window.is_key_pressed(Key::LeftShift) {
                    transform.move_by(Vector3::DOWN * final_speed);
                }

                // rotation
                if window.is_button_pressed(MouseButton::Button3) {
                    // toggle mouse grab if needed
                    if !window.mouse_grab() {
                        window.set_mouse_grab(true);
                    }

                    // rotate view
                    let delta = window.mouse_delta();

                    let mouse_x = delta.x * rotation_speed;

                    let change_y = delta.y * rotation_speed;
                    let upper_bound = change_y + *camera_angle <= 90.0;
                    let lower_bound = change_y + *camera_angle >= -90.0;
                    let mouse_y = if upper_bound && lower_bound {
                        *camera_angle += change_y;
                        change_y
                    } else {
                        0.0
                    };

                    let pitch = Quaternion::euler_rotation(0.0, mouse_x, 0.0);
                    let roll = Quaternion::euler_rotation(mouse_y, 0.0, 0.0);

                    transform.rotation = pitch * transform.rotation * roll;
                } else {
                    // toggle mouse grab if needed
                    if window.mouse_grab() {
                        window.set_mouse_grab(false);
                    }
                }
            }
            Self::Orbit { pivot, move_speed } => {
                // update move speed
                if window.is_key_typed(Key::Equal) {
                    *move_speed += 0.5;
                }
                if window.is_key_typed(Key::Minus) {
                    *move_speed -= 0.5;
                }

                // control orbiting around pivot
                let transform = &mut camera.transform;
                let angle = 5.0f32.powf(*move_speed) * delta_time;

                // mouse rotation
                if window.is_button_pressed(MouseButton::Button3) {
                    // toggle mouse grab if needed
                    if !window.mouse_grab() {
                        window.set_mouse_grab(true);
                    }

                    let delta = window.mouse_delta();
                    let speed = 50.0 * delta_time;
                    transform.move_around_point(*pivot, speed * delta.x, Vector3::UP);
                    transform.move_around_point(*pivot, speed * delta.y, transform.right());
                } else {
                    // toggle mouse grab if needed
                    if window.mouse_grab() {
                        window.set_mouse_grab(false);
                    }
                }

                // horizontal rotation
                if window.is_key_pressed(Key::D) {
                    transform.move_around_point(*pivot, -angle, Vector3::UP);
                }
                if window.is_key_pressed(Key::A) {
                    transform.move_around_point(*pivot, angle, Vector3::UP);
                }

                // vertical rotation
                if window.is_key_pressed(Key::W) {
                    transform.move_around_point(*pivot, angle, transform.right());
                }
                if window.is_key_pressed(Key::S) {
                    transform.move_around_point(*pivot, -angle, transform.right());
                }

                // zoom
                let scroll = window.scroll_delta();
                transform.move_forward(scroll.y * (*pivot - transform.position).length() * 0.05);

                // look at pivot point
                transform.look_at(*pivot);
            }
        }
    }
}
