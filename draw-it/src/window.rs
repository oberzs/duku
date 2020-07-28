// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// window creation and event handling

#![cfg(feature = "window")]

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::MouseScrollDelta;
pub use winit::event::VirtualKeyCode as Key;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::desktop::EventLoopExtDesktop;
use winit::window::Fullscreen;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;

#[cfg(feature = "ui")]
use imgui::Context;
#[cfg(feature = "ui")]
use imgui::FontConfig;
#[cfg(feature = "ui")]
use imgui::FontSource;
#[cfg(feature = "ui")]
use imgui::Ui;
#[cfg(feature = "ui")]
use imgui_winit_support::HiDpiMode;
#[cfg(feature = "ui")]
use imgui_winit_support::WinitPlatform;

use crate::error::Result;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,

    #[cfg(feature = "ui")]
    imgui: Context,
}

pub struct WindowOptions<'title> {
    pub width: u32,
    pub height: u32,
    pub title: &'title str,
    pub resizable: bool,
}

pub struct Events {
    mouse_position: (u32, u32),
    mouse_delta: (f32, f32),
    scroll_delta: (f32, f32),
    mouse_grab: bool,
    keys: Keys,
    delta_time: f32,
    fps: u32,
    resized: bool,
    window: WinitWindow,
}

#[derive(Default)]
struct Keys {
    pressed: HashSet<Key>,
    released: HashSet<Key>,
    typed: HashSet<Key>,
}

const FPS_SAMPLE_COUNT: usize = 64;

impl Window {
    pub fn new(options: WindowOptions<'_>) -> Self {
        profile_scope!("new");

        let event_loop = EventLoop::new();
        let size = PhysicalSize::new(options.width, options.height);

        info!("creating window");
        let window = match WindowBuilder::new()
            .with_inner_size(size)
            .with_title(options.title)
            .with_resizable(options.resizable)
            .build(&event_loop)
        {
            Ok(value) => value,
            Err(err) => error!("{}", err),
        };

        // configure imgui
        #[cfg(feature = "ui")]
        let imgui = {
            let mut context = Context::create();
            context.set_ini_filename(None);
            context.io_mut().display_size = [options.width as f32, options.height as f32];
            {
                let mut fonts = context.fonts();
                fonts.add_font(&[FontSource::DefaultFontData {
                    config: Some(FontConfig {
                        size_pixels: 13.0,
                        ..Default::default()
                    }),
                }]);
            }
            {
                let mut style = context.style_mut();
                style.window_rounding = 3.0;
                style.use_dark_colors();
            }
            context
        };

        Self {
            event_loop,
            window,

            #[cfg(feature = "ui")]
            imgui,
        }
    }

    pub fn main_loop(
        self,
        #[cfg(feature = "ui")] mut draw_fn: impl FnMut(&mut Events, Ui<'_>) -> Result<()>,
        #[cfg(not(feature = "ui"))] mut draw_fn: impl FnMut(&mut Events) -> Result<()>,
    ) {
        let mut event_loop = self.event_loop;
        let window = self.window;

        #[cfg(feature = "ui")]
        let (mut imgui, mut platform) = {
            let mut imgui = self.imgui;
            let mut platform = WinitPlatform::init(&mut imgui);

            // configure imgui platform
            platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

            (imgui, platform)
        };

        let mut events = Events {
            mouse_position: (0, 0),
            mouse_delta: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
            mouse_grab: false,
            keys: Keys::default(),
            delta_time: 0.0,
            fps: 0,
            resized: false,
            window,
        };

        let mut frame_time = Instant::now();
        let mut frame_count = 0;
        let mut fps_samples: [u32; FPS_SAMPLE_COUNT] = [0; FPS_SAMPLE_COUNT];
        let mut last_resize = None;

        info!("staring event loop");
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            #[cfg(feature = "ui")]
            platform.handle_event(imgui.io_mut(), &events.window, &event);

            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::CursorMoved { position: pos, .. } => {
                        events.mouse_position = (pos.x as u32, pos.y as u32);
                    }
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            last_resize = Some(Instant::now());
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    } => events.keys.handle(keycode, state),
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        info!("closing window");
                    }
                    _ => (),
                },
                Event::DeviceEvent {
                    event: dev_event, ..
                } => match dev_event {
                    DeviceEvent::MouseMotion { delta } => {
                        events.mouse_delta = (delta.0 as f32, delta.1 as f32);
                    }
                    DeviceEvent::MouseWheel { delta } => {
                        if let MouseScrollDelta::LineDelta(x, y) = delta {
                            events.scroll_delta = (x, y);
                        }
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    #[cfg(feature = "ui")]
                    platform
                        .prepare_frame(imgui.io_mut(), &events.window)
                        .expect("failed to prepare frame");

                    if let Some(last) = last_resize {
                        if Instant::now().duration_since(last) >= Duration::from_millis(100) {
                            let (new_width, new_height) = events.size();
                            info!("resized window to {}x{}", new_width, new_height);
                            events.resized = true;
                            last_resize = None;

                            #[cfg(feature = "ui")]
                            {
                                imgui.io_mut().display_size = [new_width as f32, new_height as f32];
                            }
                        }
                    } else {
                        events.resized = false;
                    }

                    if events.size() != (0, 0) && last_resize == None {
                        #[cfg(feature = "ui")]
                        {
                            let ui = imgui.frame();
                            draw_fn(&mut events, ui).unwrap();
                        }
                        #[cfg(not(feature = "ui"))]
                        draw_fn(&mut events).unwrap();
                    }

                    let delta_time = frame_time.elapsed();
                    frame_time = Instant::now();
                    fps_samples[frame_count % FPS_SAMPLE_COUNT] =
                        1_000_000 / delta_time.as_micros() as u32;
                    frame_count += 1;

                    events.keys.clear_typed();
                    events.mouse_delta = (0.0, 0.0);
                    events.delta_time = delta_time.as_secs_f32();
                    events.fps = (fps_samples.iter().sum::<u32>() as f32 / FPS_SAMPLE_COUNT as f32)
                        .ceil() as u32;
                }
                _ => (),
            }
        });
    }

    #[cfg(target_os = "windows")]
    pub fn hwnd(&self) -> *mut std::ffi::c_void {
        use winit::platform::windows::WindowExtWindows;
        self.window.hwnd()
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_window(&self) -> std::os::raw::c_ulong {
        use winit::platform::unix::WindowExtUnix;
        match self.window.xlib_window() {
            Some(w) => w,
            None => {
                error!("this is not an xlib window");
                std::process::exit(1);
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_display(&self) -> *mut std::ffi::c_void {
        use winit::platform::unix::WindowExtUnix;
        match self.window.xlib_display() {
            Some(d) => d,
            None => {
                error!("this is not an xlib window");
                std::process::exit(1);
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn ns_window(&self) -> *mut std::ffi::c_void {
        use winit::platform::macos::WindowExtMacOS;
        self.window.ns_window()
    }

    #[cfg(target_os = "macos")]
    pub fn ns_view(&self) -> *mut std::ffi::c_void {
        use winit::platform::macos::WindowExtMacOS;
        self.window.ns_view()
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    #[cfg(feature = "ui")]
    pub(crate) fn build_ui_texture(&mut self) -> (Vec<u8>, u32, u32) {
        let mut fonts = self.imgui.fonts();
        let ui_texture = fonts.build_rgba32_texture();
        (
            ui_texture.data.to_vec(),
            ui_texture.width,
            ui_texture.height,
        )
    }
}

impl Default for WindowOptions<'_> {
    fn default() -> Self {
        Self {
            width: 500,
            height: 500,
            title: "Draw-it window",
            resizable: false,
        }
    }
}

impl Events {
    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn mouse_position(&self) -> (u32, u32) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }

    pub fn resized(&self) -> Option<(u32, u32)> {
        if self.resized {
            Some(self.size())
        } else {
            None
        }
    }

    pub fn mouse_grab(&self) -> bool {
        self.mouse_grab
    }

    pub fn fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    pub fn set_title(&self, title: impl AsRef<str>) {
        self.window.set_title(title.as_ref());
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.window.set_inner_size(PhysicalSize::new(width, height));
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
        if let Err(err) = self.window.set_cursor_position(PhysicalPosition::new(x, y)) {
            error!("{}", err);
        }
    }

    pub fn set_mouse_grab(&mut self, grab: bool) {
        if let Err(err) = self.window.set_cursor_grab(grab) {
            error!("{}", err);
        }
        self.mouse_grab = grab;
    }

    pub fn set_mouse_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }

    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible);
    }

    pub fn set_fullscreen(&mut self, on: bool) {
        let mode = if on {
            Some(Fullscreen::Borderless(self.window.current_monitor()))
        } else {
            None
        };

        self.window.set_fullscreen(mode);
        self.resized = true;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys.is_pressed(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys.is_released(key)
    }

    pub fn is_key_typed(&self, key: Key) -> bool {
        self.keys.is_typed(key)
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }
}

impl Keys {
    pub(crate) fn handle(&mut self, key: Key, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed.insert(key);
                self.typed.insert(key);
                self.released.remove(&key);
            }
            ElementState::Released => {
                self.released.insert(key);
                self.pressed.remove(&key);
                self.typed.remove(&key);
            }
        }
    }

    pub(crate) fn clear_typed(&mut self) {
        self.typed.clear();
    }

    pub(crate) fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    pub(crate) fn is_released(&self, key: Key) -> bool {
        self.released.contains(&key)
    }

    pub(crate) fn is_typed(&self, key: Key) -> bool {
        self.typed.contains(&key)
    }
}
