use pigeon_parrot as parrot;

use euclid::Size2D;
use parrot::{
    RenderPassExtention,
    Painter,
    pipeline::{
        BlendFactor,
        Blending,
        BlendOp,
    },
    transform::ScreenSpace,
};
use crate::{
    log::setup_logger,
    event_system::EventSystem,
    pipeline::triangle::Triangle,
    vertex::Vertex,
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
        let window = WindowBuilder::new().with_title(&win_name).build(&event_loop).expect("Unable to create window");

        // Create an instance and surface
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        // Create painter
        let mut painter = pollster::block_on(Painter::for_surface(surface, &instance)).expect("Unable to create painter");
        let size = Size2D::new(window.inner_size().width, window.inner_size().height);
        painter.configure(size, wgpu::PresentMode::Fifo, wgpu::TextureFormat::Rgba8UnormSrgb);

        let render_window = RenderWindow::new(window, painter, win_name);

        Self {
            instance,
            windows: vec![render_window],
            event_loop,
        }
    }

    /// Create a new window with a name
    pub fn new_window(&mut self, win_name: String) {
        let window = WindowBuilder::new().with_title(&win_name).build(&self.event_loop).expect("Unable to create window");

        let surface = unsafe { self.instance.create_surface(&window) };

        let mut painter = pollster::block_on(Painter::for_surface(surface, &self.instance)).expect("Unable to create painter");

        let size = Size2D::new(window.inner_size().width, window.inner_size().height);
        painter.configure(size, wgpu::PresentMode::Fifo, wgpu::TextureFormat::Rgba8UnormSrgb);

        let render_window = RenderWindow::new(window, painter, win_name);

        self.windows.push(render_window);
    }

    /// Run the event loop
    pub fn run_event<T: EventSystem + 'static>(mut self, event_sys: T) {
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
                // Test
                Event::RedrawRequested(window_id) => {
                    // Grab the correct window
                    for win in &mut self.windows {
                        if win.window_id == window_id {
                            // Set pipeline to triangle
                            let blend = Blending::constant();
                            let pipe = win.painter.pipeline::<Triangle>(blend, wgpu::TextureFormat::Rgba8UnormSrgb, Some("triangle shader"));

                            // verticies
                            let verticies = [Vertex::new(0.0, 0.8), Vertex::new(-0.8, -0.8), Vertex::new(0.8, -0.8)];
                            let vert_b = win.painter.vertex_buffer(&verticies, Some("Basic triangle"));

                            // Render stuff
                            let mut f = win.painter.frame();
                            let rf = win.painter.current_frame().expect("Couldn't get render frame");

                            {
                                let mut pass = f.pass(parrot::painter::PassOp::Clear(parrot::color::Rgba::new(0.0156862745 , 0.97777777777, 0.48888888888 , 1.0)), &rf);
                                pass.set_parrot_pipeline(&pipe);

                                pass.set_parrot_vertex_buffer(&vert_b);

                                pass.draw(0..3, 0..1)
                            }
                            win.painter.present(f);
                        }
                    }
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

    pub fn physical_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn logical_height(&self) -> Size2D<u32, ScreenSpace> {
        Size2D::new((self.window.inner_size().width as f64 * self.window.scale_factor()).round() as u32, (self.window.inner_size().height as f64 * self.window.scale_factor()).round() as u32)
    }
}