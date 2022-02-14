use pigeon_parrot as parrot;

use parrot::{
    Painter,
};
use crate::{
    log::setup_logger,
    event_system::EventSystem,
};
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
    event::Event,
};

/// An instance of pigeon.
pub struct Pigeon {
    /// Instance of wpgu, the rendering backend
    pub instance: wgpu::Instance,
    /// Render windows
    pub windows: Vec<RenderWindow>,
    /// The event loop
    pub event_loop: EventLoop<()>,
}

impl Pigeon {
    /// Create an instance of pigeon.
    pub fn new (log_level: log::LevelFilter, win_name: String) -> Self {
        // Initialise the logger so wgpu doesn't fail silently
        setup_logger(log_level).expect("Logger init failed.");

        // Create a window and event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).expect("Unable to create window");

        // Create an instance and surface
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        // Create painter2
        let painter = pollster::block_on(Painter::for_surface(surface, &instance)).expect("Unable to create painter");

        let render_window = RenderWindow::new(window, painter, win_name);

        Self {
            instance,
            windows: vec![render_window],
            event_loop,
        }
    }

    pub fn new_window(&mut self, win_name: String) {
        let window = WindowBuilder::new().build(&self.event_loop).expect("Unable to create window");

        let surface = unsafe { self.instance.create_surface(&window) };

        let painter = pollster::block_on(Painter::for_surface(surface, &self.instance)).expect("Unable to create painter");

        let render_window = RenderWindow::new(window, painter, win_name);

        self.windows.push(render_window);
    }

    /// Run the event loop
    pub fn run(self, event_sys: Box<dyn EventSystem>) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                     window_id,
                } => {
                    for win in &self.windows {
                        if win.window_id == window_id {
                            event_sys.event(event, &win.window, control_flow);
                        }
                    }
                },
                Event::RedrawRequested(window_id) => {
                    // Render stuff
                },
                _ => (),
            }
        });
    }
}

/// A window that pigeon can render to with [`pigeon_parrot::painter::Painter`]
pub struct RenderWindow {
    pub window: winit::window::Window,
    pub painter: Painter,
    pub window_id: winit::window::WindowId,
    pub name: String,
}

impl RenderWindow {
    pub fn new(window: winit::window::Window, painter: Painter, name: String) -> Self{
        let window_id = window.id();
        Self {
            window,
            painter,
            window_id,
            name,
        }
    }
}