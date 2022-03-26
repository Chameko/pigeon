//! # Parrot
//! A repeated middleware layer
//! 
//! ## Usage
//! Parrot is designed to be an abstraction over wgpu's types so that they are easier to work with.
//! Since most of parrot's types are just light wrappers around wgpu components, if you want to know what they
//! do, go check out the corresponding wgpu component.
//! 
//! Parrot also contains a [`Painter`] to deal with a majority of the boilerplate code whilst still exposing
//! the pipeline to the user.
//! 
//! To use parrot, create a pipeline class that implements the [`Plumber`] trait.
//! Then create an instance of the [`Painter`] using a wgpu compatible surface.
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

pub use pipeline::Plumber;
pub use painter::RenderPassExtention;
pub use painter::Painter;
