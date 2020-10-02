extern crate flexi_logger;
extern crate glium;
extern crate serde_yaml;
extern crate thyme;

use glium::glutin;
use glium::glutin::event;
use log::{info, warn};

pub struct Game {}

struct DebugEventHandler {}

impl DebugEventHandler {
    pub fn handle(
        &self,
        ev: &glium::glutin::event::Event<()>,
        control_flow: &mut glium::glutin::event_loop::ControlFlow,
    ) {
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

        // load assets
        let font_src = include_bytes!("../data/fonts/Roboto-Medium.ttf");
        let image_src = include_bytes!("../data/images/gui-pixel.png");
        let image = image::load_from_memory(image_src).unwrap().to_rgba();
        let theme_src = include_str!("../data/theme-base.yml");
        let theme: serde_yaml::Value = serde_yaml::from_str(theme_src).unwrap();
        let window_size = [1280.0, 720.0];

        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let wb = glium::glutin::window::WindowBuilder::new().with_title("cyclops");
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let handler = DebugEventHandler {};

        // create thyme backend
        let mut io = thyme::WinitIo::new(&event_loop, window_size.into());
        let mut renderer = thyme::GliumRenderer::new(&display).unwrap();
        let mut context_builder = thyme::ContextBuilder::new(thyme::BuildOptions {
            enable_live_reload: false,
        });

        // register resources in thyme and create the context
        let image_dims = image.dimensions();
        context_builder.register_theme(theme).unwrap();
        context_builder.register_texture("pixel", image.into_raw(), image_dims);
        context_builder.register_font("roboto", font_src.to_vec());
        let mut context = context_builder.build(&mut renderer, &mut io).unwrap();

        info!("Starting event loop");
        event_loop.run(move |ev, _, control_flow| match ev {
            event::Event::MainEventsCleared => {
                let frame_start = std::time::Instant::now();

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);

                let mut ui = context.create_frame();
                ui.window()
                ui.window("window", |ui| {
                    ui.gap(20.0);
                    ui.button("label", "Hello, World!");
                });

                renderer.draw_frame(&mut target, ui).unwrap();

                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(
                    frame_start + std::time::Duration::from_millis(16),
                );

                target.finish().unwrap();
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = glutin::event_loop::ControlFlow::Exit,
            event => {
                io.handle_event(&mut context, &event);
            }
        })
    }
}
