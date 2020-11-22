// Oliver Berzs
// https://github.com/oberzs/duku

#![cfg(feature = "window")]

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use window_dep::dpi::PhysicalPosition;
use window_dep::dpi::PhysicalSize;
use window_dep::event::DeviceEvent;
use window_dep::event::ElementState;
use window_dep::event::Event as WinitEvent;
use window_dep::event::MouseScrollDelta;
use window_dep::event::WindowEvent;
use window_dep::event_loop::ControlFlow;
use window_dep::event_loop::EventLoop;
use window_dep::window::Window as WinitWindow;
use window_dep::window::WindowBuilder as WinitWindowBuilder;

pub use window_dep::event::MouseButton;
pub use window_dep::event::VirtualKeyCode as Key;
pub use window_dep::window::CursorIcon as Cursor;

use crate::duku::Duku;
use crate::duku::DukuBuilder;
use crate::error::Result;
use crate::math::Quaternion;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::renderer::Camera;
use crate::surface::WindowHandle;

pub struct Window {
    window: WinitWindow,
    event_loop: EventLoop<()>,
}

pub struct Events {
    window: WinitWindow,
    events: Vec<Event>,

    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
    keys_typed: HashSet<Key>,
    buttons_pressed: HashSet<MouseButton>,
    buttons_released: HashSet<MouseButton>,
    buttons_clicked: HashSet<MouseButton>,
    typed_char: Option<char>,

    mouse_position: Vector2,
    mouse_delta: Vector2,
    mouse_grab: bool,
    scroll_delta: Vector2,
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Resize(Vector2),
}

#[derive(Debug, Copy, Clone)]
pub enum Controller {
    Fly { camera_angle: f32, move_speed: f32 },
    Orbit { pivot: Vector3, move_speed: f32 },
}

#[derive(Debug, Clone)]
pub struct WindowBuilder {
    duku: DukuBuilder,
    title: String,
    resizable: bool,
    width: u32,
    height: u32,
}

impl DukuBuilder {
    pub fn build_window(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            duku: self,
            title: "".to_string(),
            resizable: false,
            width,
            height,
        }
    }
}

impl Window {
    pub(crate) fn new(title: &str, width: u32, height: u32, resizable: bool) -> Self {
        let event_loop = EventLoop::new();
        let window = WinitWindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(title)
            .with_resizable(resizable)
            .build(&event_loop)
            .expect("bad window");

        Self { window, event_loop }
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use window_dep::platform::windows::WindowExtWindows;

        WindowHandle {
            hwnd: self.window.hwnd(),
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use window_dep::platform::unix::WindowExtUnix;

        WindowHandle {
            xlib_window: self.window.xlib_window().expect("Wayland not supported"),
            xlib_display: self.window.xlib_display().expect("Wayland not supported"),
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn handle(&self) -> WindowHandle {
        unimplemented!()
    }

    pub fn main_loop<F>(self, mut main_fn: F)
    where
        F: FnMut(&mut Events) + 'static,
    {
        let Self { window, event_loop } = self;
        let mut events = Events {
            events: vec![],
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            keys_typed: HashSet::new(),
            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),
            buttons_clicked: HashSet::new(),
            mouse_position: Vector2::default(),
            mouse_delta: Vector2::default(),
            mouse_grab: false,
            scroll_delta: Vector2::default(),
            typed_char: None,
            window,
        };

        let mut last_resize = None;

        event_loop.run(move |event, _, control_flow| match event {
            WinitEvent::WindowEvent { event, window_id } if window_id == events.window.id() => {
                match event {
                    // close event
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    // resize event
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            last_resize = Some(Instant::now());
                        }
                    }

                    // mouse position event
                    WindowEvent::CursorMoved { position, .. } => {
                        events.mouse_position = Vector2::new(position.x as f32, position.y as f32);
                    }

                    // keyboard key event
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => {
                                    events.keys_typed.insert(key);
                                    events.keys_pressed.insert(key);
                                    events.keys_released.remove(&key);
                                }
                                ElementState::Released => {
                                    events.keys_released.insert(key);
                                    events.keys_pressed.remove(&key);
                                    events.keys_typed.remove(&key);
                                }
                            }
                        }
                    }

                    // mouse button event
                    WindowEvent::MouseInput { state, button, .. } => match state {
                        ElementState::Pressed => {
                            events.buttons_clicked.insert(button);
                            events.buttons_pressed.insert(button);
                            events.buttons_released.remove(&button);
                        }
                        ElementState::Released => {
                            events.buttons_released.insert(button);
                            events.buttons_pressed.remove(&button);
                            events.buttons_clicked.remove(&button);
                        }
                    },

                    // text input event
                    WindowEvent::ReceivedCharacter(c) => {
                        if !c.is_ascii_control() {
                            events.typed_char = Some(c);
                        }
                    }

                    // mouse scroll event
                    WindowEvent::MouseWheel { delta, .. } => {
                        if let MouseScrollDelta::LineDelta(x, y) = delta {
                            events.scroll_delta = Vector2::new(x as f32, y as f32);
                        }
                    }

                    _ => (),
                }
            }

            // mouse delta event
            WinitEvent::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    let (x, y) = delta;
                    events.mouse_delta = Vector2::new(x as f32, y as f32);
                }
            }

            // draw event
            WinitEvent::MainEventsCleared => {
                // check resize timing
                if let Some(last) = last_resize {
                    if Instant::now().duration_since(last) >= Duration::from_millis(100) {
                        let size = events.size();
                        events.events.push(Event::Resize(size));
                        last_resize = None;

                        info!("resized window to {}x{}", size.x as u32, size.y as u32);
                    }
                }

                let size = events.size();
                if size.x as i32 != 0 && size.y as i32 != 0 && last_resize == None {
                    main_fn(&mut events);
                }

                events.events.clear();
                events.keys_typed.clear();
                events.typed_char = None;
                events.mouse_delta = Vector2::new(0.0, 0.0);
                events.scroll_delta = Vector2::new(0.0, 0.0);
            }
            _ => (),
        });
    }
}

impl Events {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_key_typed(&self, key: Key) -> bool {
        self.keys_typed.contains(&key)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }

    pub fn is_button_clicked(&self, button: MouseButton) -> bool {
        self.buttons_clicked.contains(&button)
    }

    pub const fn mouse_position(&self) -> Vector2 {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, position: Vector2) {
        self.window
            .set_cursor_position(PhysicalPosition::new(position.x as i32, position.y as i32))
            .expect("cannot set cursor position");
    }

    pub const fn mouse_delta(&self) -> Vector2 {
        self.mouse_delta
    }

    pub const fn scroll_delta(&self) -> Vector2 {
        self.scroll_delta
    }

    pub const fn mouse_grab(&self) -> bool {
        self.mouse_grab
    }

    pub fn set_mouse_grab(&mut self, grab: bool) {
        self.window
            .set_cursor_grab(grab)
            .expect("cannot set cursor grab");
        self.mouse_grab = grab;
    }

    pub fn hide_cursor(&mut self, hide: bool) {
        self.window.set_cursor_visible(!hide);
    }

    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.window.set_cursor_icon(cursor);
    }

    pub fn size(&self) -> Vector2 {
        let size = self.window.inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.window.set_title(title.as_ref());
    }

    pub const fn typed_char(&self) -> Option<char> {
        self.typed_char
    }
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

    pub fn update(&mut self, camera: &mut Camera, events: &mut Events, delta_time: f32) {
        match self {
            Self::Fly {
                camera_angle,
                move_speed,
            } => {
                // update move speed
                if events.is_key_typed(Key::Plus) {
                    *move_speed += 0.5;
                }
                if events.is_key_typed(Key::Minus) {
                    *move_speed -= 0.5;
                }

                // control in flying mode
                let transform = &mut camera.transform;
                let final_speed = 5.0f32.powf(*move_speed) * delta_time;
                let rotation_speed = 50.0 * delta_time;

                // movement
                if events.is_key_pressed(Key::W) {
                    transform.move_forward(final_speed);
                }
                if events.is_key_pressed(Key::S) {
                    transform.move_back(final_speed);
                }
                if events.is_key_pressed(Key::A) {
                    transform.move_left(final_speed);
                }
                if events.is_key_pressed(Key::D) {
                    transform.move_right(final_speed);
                }
                if events.is_key_pressed(Key::Space) {
                    transform.move_by(Vector3::UP * final_speed);
                }
                if events.is_key_pressed(Key::LShift) {
                    transform.move_by(Vector3::DOWN * final_speed);
                }

                // rotation
                if events.is_button_pressed(MouseButton::Middle) {
                    // toggle mouse grab if needed
                    if !events.mouse_grab() {
                        events.set_mouse_grab(true);
                    }

                    // rotate view
                    let delta = events.mouse_delta();

                    let mouse_x = delta.x * rotation_speed;
                    let mouse_y =
                        clamp_change(*camera_angle, delta.y * rotation_speed, -90.0, 90.0);
                    *camera_angle += mouse_y;

                    let pitch = Quaternion::euler_rotation(0.0, mouse_x, 0.0);
                    let roll = Quaternion::euler_rotation(mouse_y, 0.0, 0.0);

                    transform.rotation = pitch * transform.rotation * roll;
                } else {
                    // toggle mouse grab if needed
                    if events.mouse_grab() {
                        events.set_mouse_grab(false);
                    }
                }
            }
            Self::Orbit { pivot, move_speed } => {
                // update move speed
                if events.is_key_typed(Key::Plus) {
                    *move_speed += 0.5;
                }
                if events.is_key_typed(Key::Minus) {
                    *move_speed -= 0.5;
                }

                // control orbiting around pivot
                let transform = &mut camera.transform;
                let angle = 5.0f32.powf(*move_speed) * delta_time;

                // mouse rotation
                if events.is_button_pressed(MouseButton::Middle) {
                    // toggle mouse grab if needed
                    if !events.mouse_grab() {
                        events.set_mouse_grab(true);
                        events.hide_cursor(true);
                    }

                    let delta = events.mouse_delta();
                    let speed = 50.0 * delta_time;

                    transform.move_around_point(*pivot, speed * delta.x, Vector3::UP);
                    transform.move_around_point(*pivot, speed * delta.y, transform.right());
                } else {
                    // toggle mouse grab if needed
                    if events.mouse_grab() {
                        events.set_mouse_grab(false);
                        events.hide_cursor(false);
                    }
                }

                // horizontal rotation
                if events.is_key_pressed(Key::D) {
                    transform.move_around_point(*pivot, -angle, Vector3::UP);
                }
                if events.is_key_pressed(Key::A) {
                    transform.move_around_point(*pivot, angle, Vector3::UP);
                }

                // vertical rotation
                if events.is_key_pressed(Key::W) {
                    transform.move_around_point(*pivot, angle, transform.right());
                }
                if events.is_key_pressed(Key::S) {
                    transform.move_around_point(*pivot, -angle, transform.right());
                }

                // zoom
                let scroll = events.scroll_delta();
                transform.move_forward(scroll.y * (*pivot - transform.position).length() * 0.05);

                // look at pivot point
                transform.look_at(*pivot);
            }
        }
    }
}

impl WindowBuilder {
    pub const fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.title = title.as_ref().to_string();
        self
    }

    pub fn build(self) -> Result<(Duku, Window)> {
        let window = Window::new(&self.title, self.width, self.height, self.resizable);
        let duku = self.duku.attach_window(window.handle()).build()?;

        Ok((duku, window))
    }
}

fn clamp_change(current: f32, change: f32, min: f32, max: f32) -> f32 {
    if current + change > min && current + change < max {
        change
    } else {
        0.0
    }
}
