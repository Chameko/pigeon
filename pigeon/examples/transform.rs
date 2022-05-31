extern crate winit;
use winit::event_loop::ControlFlow;
use winit::event::{WindowEvent, Event};
use pigeon_2d::pigeon::{Pigeon, draw_triangle, draw};
use pigeon_2d::graphics::{Rectangle, Triangle};
use pigeon_2d::graphics::Rgba;
use euclid::{Size2D, Rotation3D, Translation3D, Angle};

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    
    // Create an event loop
    let event_loop = winit::event_loop::EventLoop::new();
    // Create a window to draw to
    let window = winit::window::WindowBuilder::new().with_title("Triangle :D").build(&event_loop).unwrap();

    // Create a wgpu instance
    let instance = wgpu::Instance::new(wgpu::Backends::GL);
    let surface = unsafe { instance.create_surface(&window) };

    // Get the size of the window
    let winsize = window.inner_size();

    let mut p = Pigeon::new(surface, &instance, Size2D::new(winsize.width as f32, winsize.height as f32), 1);

    let mut rect = Rectangle::new((0.0, -100.0, 0.0), (40.0, 40.0), Rgba::GREEN);
    let mut rect2 = Rectangle::new((-450.0, 0.0, 0.0), (40.0, 40.0), Rgba::BLUE);
    let mut rect3 = Rectangle::new((450.0, 0.0, 0.0), (40.0, 40.0), Rgba::RED);
    let mut tri = Triangle::new((0.0, 20.0, 0.0), (-10.0, 0.0, 0.0), (10.0, 0.0, 0.0), (0.0, 100.0, 0.0), Rgba::BLACK);
    let mut tri2 = tri.clone();

    rect2.rotate(Rotation3D::around_z(Angle::degrees(45.0)));

    rect.translate(Translation3D::new(0.0, 100.0, 0.0));

    rect3.rotate(Rotation3D::euler(Angle::degrees(0.0), Angle::degrees(45.0), Angle::degrees(0.0)));

    tri.rotate(Rotation3D::around_z(Angle::degrees(45.0)));

    tri2.scale(2.0, 2.0, 2.0);
    tri2.translate(Translation3D::new(0.0, -50.0, 0.0));

    // Initiate the event loop
    event_loop.run(move |event, _, control_flow| {
        // Only update the event loop if input is recieved
        *control_flow = ControlFlow::Wait;

        match event {
            // Window event
            Event::WindowEvent { event: win_event, .. } => {
                match win_event {
                    // Close if a close request is detected
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit
                    },
                    // Update the surface if resized
                    WindowEvent::Resized(size) => {
                        let size = euclid::Size2D::new(size.width, size.height);
                        p.paint.configure(size, wgpu::PresentMode::Fifo, p.paint.preferred_format());
                        let size = euclid::Size2D::new(size.width as f32, size.height as f32);
                        p.update_size(size);
                    }
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                // Time to draw our shape :D
                draw(&mut p, |cont| draw_triangle(cont, vec![&rect2, &rect, &rect3, &tri, &tri2]))
            }
            _ => ()
        }
    });
}