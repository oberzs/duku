// Oliver Berzs
// https://github.com/oberzs/duku

// wrapper around glfw window

#![cfg(feature = "window")]

mod controller;

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::event::ElementState;
use winit::event::Event as WinitEvent;
use winit::event::MouseScrollDelta;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;

pub use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode as Key;
pub use winit::window::CursorIcon as Cursor;

use crate::math::Vector2;
use crate::surface::WindowHandle;

pub use controller::Controller;

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

impl Window {
    pub(crate) fn new(title: &str, width: u32, height: u32, resizable: bool) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(title)
            .with_resizable(resizable)
            .build(&event_loop)
            .expect("bad window");

        Self { window, event_loop }
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use winit::platform::windows::WindowExtWindows;

        WindowHandle {
            hwnd: self.window.hwnd(),
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use winit::platform::unix::WindowExtUnix;

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
