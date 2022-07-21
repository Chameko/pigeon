extern crate winit;
extern crate pigeon_parrot as parrot;
extern crate image;
use winit::event_loop::ControlFlow;
use winit::event::{WindowEvent, Event};
use pigeon_2d::pigeon::{Pigeon, add_quad, draw};
use pigeon_2d::graphics::{Sprite, Texture};
use euclid::Size2D;
use std::rc::Rc;

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

    // Load our image
    let img_bytes = include_bytes!("./logo.png");
    let img = image::load_from_memory(img_bytes).unwrap();
    // Convert to our colour format
    let img_rgb = img.to_rgba8().to_vec();
    let img_rgb = parrot::color::Rgba8::align(img_rgb.as_slice());

    use image::GenericImageView;
    let dimensions = img.dimensions();

    // Create an empty texture
    let texture = p.paint.texture(Size2D::from(dimensions), wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, Some("logo"), false);
    // Fill the texture with the image bytes
    parrot::Texture::fill(&texture, img_rgb, &p.paint.device);
    // Create a sampler for our texture
    let sampler = Rc::new(p.paint.sampler(wgpu::FilterMode::Nearest, wgpu::FilterMode::Linear, Some("Image sampler")));

    let sprite_texture = Texture::new(texture, sampler, "logo");

    let sprite = Sprite::new((0.0, 0.0, 0.0), (364.0, 467.0), std::rc::Rc::new(sprite_texture));

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
                draw(&mut p, |cont| add_quad(cont, vec![&sprite]))
            }
            _ => ()
        }
    });
}