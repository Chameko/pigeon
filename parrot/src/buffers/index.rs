/// Represents the index buffer
#[derive(Debug)]
pub struct IndexBuffer {
    /// Wrapped wgpu type
    pub wgpu: wgpu::Buffer,
    /// Number of elements
    pub elements: u32,
}

impl IndexBuffer {
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.wgpu
            .slice(0..(self.elements as usize * std::mem::size_of::<u16>()) as u64)
    }
}