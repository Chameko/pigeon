//! # Parrot
//! A repeated middleware layer
//! 
//! ## Usage
//! Parrot is designed to be an abstraction over wgpu's types so that they are easier to work with.
//! Parrot also comes in with a basic rendering framework, however the pipeline is exposed, so users
//! can create their own.
//! 
//! To use parrot, create a pipeline class that implements the [`pipeline::Plumber`] trait.
//! Then create an instance of the [`painter::Painter`] using a wgpu compatible surface.
//! 
//! To use this library you should have a rudementary understanding of how wgpu works. A good resource to get
//! started is [learn wgpu](https://sotrh.github.io/learn-wgpu/#what-is-wgpu).

#[deny(clippy::pedantic)]
#[warn(dead_code)]

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
pub mod paintable;
pub use painter::RenderPassExtention;