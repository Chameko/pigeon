use crate::{
    binding::BindingGroup,
    painter::{Draw, RenderPassExtention},
};

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