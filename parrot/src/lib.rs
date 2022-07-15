//! # Parrot
//! A repeated middleware layer
//! 
//! ## About
//! Parrot is more or less a re-implementation of [easygpu](https://github.com/khonsulabs/easygpu) for pigeon. Hence its development is primarily driven by the demands of pigeon. It deals with some of the boilerplate code that comes with [wgpu](https://crates.io/crates/wgpu) and provides various wrappers around [wgpu's](https://crates.io/crates/wgpu) types so they are easier to work with, while still giving you control. Parrot isn't magic and requires you understand wgpu well enough to be used properly.
//! 
//! ## Usage
//! Before using parrot I recommend reading [learn wgpu](https://sotrh.github.io/learn-wgpu/#what-is-wgpu) as it's an excellent resource to get you aquainted with wgpu and how it works. As a basic setup you will create a wgpu instance and a window using winit and then a Painter. From there it is up to you.
//! ```rust
//! fn main() {
//!     // Initialise the logging output at info level only from parrot
//!     env_logger::builder().filter_module("pigeon_parrot", log::LevelFilter::Info).init();
//!     // Create an event loop
//!     let event_loop = winit::event_loop::EventLoop::new();
//!     // Create a window to draw to
//!     let window = winit::window::WindowBuilder::new().with_title("Triangle :D").build(&event_loop).unwrap();
//!     // Create a wgpu instance
//!     let instance = wgpu::Instance::new(wgpu::Backends::GL);
//!     let surface = unsafe { instance.create_surface(&window) };
//!     // Create the painter
//!     let mut painter = pollster::block_on(parrot::Painter::for_surface(surface, &instance, 1)).unwrap();
//!     // Get the size of the window
//!     let winsize = window.inner_size();
//!     // Get the preferred texture format for the surface
//!    let pref_format = painter.preferred_format();
//!     // Configure the surface
//!     painter.configure(euclid::Size2D::new(winsize.width, winsize.height), wgpu::PresentMode::Fifo, pref_format);
//!     // ...
//! }
//! ```
//! I have created some examples (in the examples folder) that demonstrate parrots capabilities and will hopefully give you an idea of how to use parrot. To run them use `cargo run --example=ExampleNameHere`

pub mod painter;
pub mod pipeline;
pub mod binding;
pub mod shader;
pub mod device;
pub mod transform;
pub mod vertex;
pub mod buffers;
pub mod texture;
pub mod sampler;
pub mod color;
pub mod error;
pub mod frame;

pub use pipeline::{Plumber, PipelineCore, PipelineDescription};
pub use painter::{RenderPassExtention, Painter};
pub use texture::Texture;
pub use sampler::Sampler;
pub use buffers::*;
pub use color::*;
pub use device::Device;
