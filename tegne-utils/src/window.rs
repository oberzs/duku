use log::debug;
use log::error;
use log::info;
use std::process::exit;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    size: (u32, u32),
}

pub struct Events {
    pub mouse_pos: (u32, u32),
    pub mouse_delta: (f32, f32),
    pub size: (u32, u32),
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();

        debug!("create window");
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop)
            .or_error("cannot create window");
        info!("window created");

        Self {
            event_loop,
            window,
            size: (width, height),
        }
    }

    pub fn start_loop<F: Fn(&Events) + 'static>(self, draw: F) {
        let size = self.window.inner_size();
        let mut events = Events {
            mouse_pos: (0, 0),
            mouse_delta: (0.0, 0.0),
            size: (size.width, size.height),
        };

        debug!("start event loop");
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::CursorMoved { position: pos, .. } => {
                        events.mouse_pos = (pos.x as u32, pos.y as u32);
                    }
                    WindowEvent::CloseRequested => {
                        debug!("close window");
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    draw(&events);
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
        self.window.xlib_window().or_error("no xlib support")
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_display(&self) -> *mut std::ffi::c_void {
        use winit::platform::unix::WindowExtUnix;
        self.window.xlib_display().or_error("no xlib support")
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
        self.size
    }
}

trait OrError<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> OrError<T> for Result<T, E> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}

impl<T> OrError<T> for Option<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}
