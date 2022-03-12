pub mod vertex;
pub mod index;
pub mod uniform;

pub use {vertex::VertexBuffer, index::IndexBuffer, uniform::UniformBuffer};

/// A marker trait for buffers
pub trait Buffer {
    /// Return the contained buffer
    fn wgpu_buffer(&self) -> &wgpu::Buffer;
}