// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// basic 1st person camera controller

#![cfg(feature = "controller")]

use super::Camera;
use crate::math::Quaternion;
use crate::math::Vector3;
use crate::window::Events;
use crate::window::Key;

pub struct Controller {
    camera_angle: f32,
    lockon_point: Vector3,
    lockon: bool,
    move_speed: f32,
}

impl Controller {
    pub fn update(&mut self, camera: &mut Camera, events: &mut Events) {
        let rotate_speed = 70.0;

        if events.is_key_typed(Key::F11) {
            events.set_fullscreen(!events.fullscreen());
        }

        if events.is_key_typed(Key::Escape) {
            events.set_mouse_grab(!events.mouse_grab());
            events.set_mouse_visible(!events.mouse_grab());
        }

        if events.is_key_typed(Key::LAlt) {
            self.lockon = !self.lockon;
        }

        let (_, scroll_y) = events.scroll_delta();
        self.move_speed += scroll_y * 0.01;
        self.move_speed = f32::max(f32::min(self.move_speed, 15.0), 1.0);

        let transform = &mut camera.transform;

        // camera movement
        let final_move_speed = self.move_speed * events.delta_time();

        if events.is_key_pressed(Key::W) {
            transform.move_forward(final_move_speed);
        }

        if events.is_key_pressed(Key::S) {
            transform.move_backward(final_move_speed);
        }

        if events.is_key_pressed(Key::A) {
            transform.move_left(final_move_speed);
        }

        if events.is_key_pressed(Key::D) {
            transform.move_right(final_move_speed);
        }

        if events.is_key_pressed(Key::Space) {
            transform.move_up(final_move_speed);
        }

        if events.is_key_pressed(Key::LShift) {
            transform.move_down(final_move_speed);
        }

        // look direction
        if events.mouse_grab() {
            let (x, y) = events.mouse_delta();

            let mouse_x = x * rotate_speed * events.delta_time();

            let change_y = y * rotate_speed * events.delta_time();
            let upper_bound = change_y + self.camera_angle <= 90.0;
            let lower_bound = change_y + self.camera_angle >= -90.0;
            let mouse_y = if upper_bound && lower_bound {
                self.camera_angle += change_y;
                change_y
            } else {
                0.0
            };

            let pitch = Quaternion::euler_rotation(0.0, mouse_x, 0.0);
            let roll = Quaternion::euler_rotation(mouse_y, 0.0, 0.0);

            transform.rotation = pitch * transform.rotation * roll;
        }

        if self.lockon {
            transform.look_at(self.lockon_point, Vector3::up());
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            camera_angle: 0.0,
            lockon_point: Vector3::default(),
            lockon: false,
            move_speed: 5.0,
        }
    }
}
