use log::debug;
use log::error;
use log::info;
use std::process::exit;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();

        debug!("create window");
        let window = WindowBuilder::new()
            .build(&event_loop)
            .or_error("cannot create window");
        info!("window created");

        Self {
            event_loop,
            window,
            width,
            height,
        }
    }

    pub fn start_loop<F: Fn() + 'static>(self, draw: F) {
        let window = self.window;

        debug!("start event loop");
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    debug!("close window");
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    draw();
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
        unwrap_error(self.window.xlib_window().ok_or(""), "no xlib support")
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_display(&self) -> *mut std::ffi::c_void {
        use winit::platform::unix::WindowExtUnix;
        unwrap_error(self.window.xlib_display().ok_or(""), "no xlib support")
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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
