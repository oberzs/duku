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
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        debug!("create window");
        let window = match WindowBuilder::new().build(&event_loop) {
            Ok(win) => win,
            Err(_) => {
                error!("cannot create window");
                exit(1);
            }
        };
        info!("window created");

        Self { event_loop, window }
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
}
