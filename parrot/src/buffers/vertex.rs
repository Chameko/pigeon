use crate::{
    binding::BindingGroup,
    painter::{Draw, RenderPassExtention},
};
use super::Buffer;

/// Represents the vertex buffer
#[derive(Debug)]
pub struct VertexBuffer {
    /// Size of the buffer in bytes
    pub size: u32,
    /// Wrapped wgpu buffer
    pub wgpu: wgpu::Buffer,
}

impl VertexBuffer {
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.wgpu.slice(0..self.size as u64)
    }
}

impl Draw for VertexBuffer {
    fn draw<'a, 'b>(&'a self, binding: &'a BindingGroup, pass: &'b mut wgpu::RenderPass<'a>) {
        pass.set_binding(binding, &[]);
        pass.draw_buffer(self);
    }
}

impl Buffer for VertexBuffer {
    fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.wgpu
    }
}