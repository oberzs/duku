// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// wrapper around glfw window

#![cfg(feature = "window")]

use glfw::Action;
use glfw::Cursor as GlfwCursor;
use glfw::CursorMode;
use glfw::StandardCursor;
use glfw::Window as GlfwWindow;
use std::collections::HashSet;
use std::time::Instant;
use std::vec::Drain;

use crate::math::Vector2;

pub use glfw::Key;
pub use glfw::MouseButton;

pub struct Window {
    handle: GlfwWindow,
    events: Vec<Event>,

    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
    keys_typed: HashSet<Key>,
    buttons_pressed: HashSet<MouseButton>,
    buttons_released: HashSet<MouseButton>,
    buttons_clicked: HashSet<MouseButton>,

    mouse_position: Vector2,
    mouse_delta: Vector2,
    scroll_delta: Vector2,

    begin_time: Instant,
    last_resize: Option<Instant>,
    delta_time: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Resize(u32, u32),
}

#[derive(Debug, Copy, Clone)]
pub enum Cursor {
    Arrow,
    IBeam,
    Crosshair,
    Hand,
    HResize,
    VResize,
}

impl Window {
    pub(crate) fn new(handle: GlfwWindow) -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            keys_typed: HashSet::new(),
            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),
            buttons_clicked: HashSet::new(),
            mouse_position: Vector2::new(0.0, 0.0),
            mouse_delta: Vector2::new(0.0, 0.0),
            scroll_delta: Vector2::new(0.0, 0.0),
            begin_time: Instant::now(),
            last_resize: None,
            delta_time: 0.0,
            events: vec![],
            handle,
        }
    }

    pub fn is_open(&self) -> bool {
        !self.handle.should_close()
    }

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

    pub fn mouse_position(&self) -> Vector2 {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, position: Vector2) {
        self.handle
            .set_cursor_pos(position.x as f64, position.y as f64);
    }

    pub fn mouse_delta(&self) -> Vector2 {
        self.mouse_delta
    }

    pub fn scroll_delta(&self) -> Vector2 {
        self.scroll_delta
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn mouse_grab(&self) -> bool {
        self.handle.get_cursor_mode() == CursorMode::Disabled
    }

    pub fn set_mouse_grab(&mut self, grab: bool) {
        let cursor_mode = if grab {
            CursorMode::Disabled
        } else {
            CursorMode::Normal
        };
        self.handle.set_cursor_mode(cursor_mode);
    }

    pub fn hide_cursor(&mut self, hide: bool) {
        let cursor_mode = if hide {
            CursorMode::Hidden
        } else {
            CursorMode::Normal
        };
        self.handle.set_cursor_mode(cursor_mode);
    }

    pub fn set_cursor(&mut self, cursor: Cursor) {
        let glfw_cursor = match cursor {
            Cursor::Arrow => GlfwCursor::standard(StandardCursor::Arrow),
            Cursor::Crosshair => GlfwCursor::standard(StandardCursor::Crosshair),
            Cursor::Hand => GlfwCursor::standard(StandardCursor::Hand),
            Cursor::HResize => GlfwCursor::standard(StandardCursor::HResize),
            Cursor::IBeam => GlfwCursor::standard(StandardCursor::IBeam),
            Cursor::VResize => GlfwCursor::standard(StandardCursor::VResize),
        };
        self.handle.set_cursor(Some(glfw_cursor));
    }

    pub fn size(&self) -> Vector2 {
        let (w, h) = self.handle.get_size();
        Vector2::new(w as f32, h as f32)
    }

    pub fn events(&mut self) -> Drain<'_, Event> {
        self.events.drain(..)
    }

    pub(crate) fn handle_key(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => {
                self.keys_pressed.insert(key);
                self.keys_typed.insert(key);
                self.keys_released.remove(&key);
            }
            Action::Release => {
                self.keys_released.insert(key);
                self.keys_pressed.remove(&key);
                self.keys_typed.remove(&key);
            }
            _ => (),
        }
    }

    pub(crate) fn handle_mouse_button(&mut self, button: MouseButton, action: Action) {
        match action {
            Action::Press => {
                self.buttons_pressed.insert(button);
                self.buttons_clicked.insert(button);
                self.buttons_released.remove(&button);
            }
            Action::Release => {
                self.buttons_released.insert(button);
                self.buttons_pressed.remove(&button);
                self.buttons_clicked.remove(&button);
            }
            _ => (),
        }
    }

    pub(crate) fn handle_mouse(&mut self, x: f64, y: f64) {
        let mouse_position = Vector2::new(x as f32, y as f32);
        self.mouse_delta = mouse_position - self.mouse_position;
        self.mouse_position = mouse_position;
    }

    pub(crate) fn handle_scroll(&mut self, x: f64, y: f64) {
        self.scroll_delta = Vector2::new(x as f32, y as f32);
    }

    pub(crate) fn record_resize(&mut self) {
        self.last_resize = Some(Instant::now());
    }

    pub(crate) fn reset_resize(&mut self) {
        self.last_resize = None;
    }

    pub(crate) fn last_resize(&self) -> Option<Instant> {
        self.last_resize
    }

    pub(crate) fn handle_resize(&mut self, width: u32, height: u32) {
        self.events.push(Event::Resize(width, height));
    }

    pub(crate) fn raw_size(&self) -> (i32, i32) {
        self.handle.get_size()
    }

    pub(crate) fn clear(&mut self) {
        self.keys_typed.clear();
        self.mouse_delta = Vector2::new(0.0, 0.0);
    }

    pub(crate) fn update_delta_time(&mut self) {
        self.delta_time = self.begin_time.elapsed().as_secs_f32();
        self.begin_time = Instant::now();
    }
}
