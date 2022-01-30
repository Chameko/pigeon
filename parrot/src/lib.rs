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