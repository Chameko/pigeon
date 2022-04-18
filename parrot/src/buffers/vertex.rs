/// Represents the vertex buffer
#[derive(Debug)]
pub struct VertexBuffer {
    /// Size of the buffer in bytes
    pub size: u32,
    /// Wrapped wgpu buffer
    pub wgpu: wgpu::Buffer,
    /// Name of the vertex buffer
    pub name: Option<String>
}

impl VertexBuffer {
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.wgpu.slice(0..self.size as u64)
    }
}
