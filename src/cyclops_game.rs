extern crate flexi_logger;
extern crate glium;

use log::{info, warn};

pub struct Game {}

struct DebugEventHandler {}

impl DebugEventHandler {
    pub fn handle(
        &self,
        ev: &glium::glutin::event::Event<()>,
        control_flow: &mut glium::glutin::event_loop::ControlFlow,
    ) {
        use glium::glutin;
        use glium::glutin::event;

        match ev {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    info!("Close event");
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                event::WindowEvent::MouseWheel { .. } => info!("Mouse wheel event"),
                event::WindowEvent::MouseInput { button, .. } => match button {
                    event::MouseButton::Left => info!("Mouse left button pressed"),
                    event::MouseButton::Middle => info!("Mouse middle button pressed"),
                    event::MouseButton::Right => info!("Mouse right button pressed"),
                    event::MouseButton::Other { .. } => info!("Mouse other button pressed"),
                },
                _ => return,
            },
            _ => (),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn start(&self) {
        // Initializing logger
        flexi_logger::Logger::with_str("info, cyclops = debug")
            .log_to_file()
            .duplicate_to_stderr(flexi_logger::Duplicate::All)
            .start()
            .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

        info!("Initializing window");
        use glium::{glutin, Surface};

        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let wb = glium::glutin::window::WindowBuilder::new().with_title("cyclops-r");
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let handler = DebugEventHandler {};

        info!("Starting event loop");
        event_loop.run(move |ev, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

            handler.handle(&ev, control_flow);

            if *control_flow == glutin::event_loop::ControlFlow::Exit {
                return;
            }

            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.finish().unwrap();
        })
    }
}
