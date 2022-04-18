/// Represents the index buffer
#[derive(Debug)]
pub struct IndexBuffer {
    /// Wrapped wgpu type
    pub wgpu: wgpu::Buffer,
    /// Size of the buffer in indicies
    pub size: u32,
    /// Name
    pub name: Option<String>
}

impl IndexBuffer {
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.wgpu
            .slice(0..(self.size as usize * std::mem::size_of::<u16>()) as u64)
    }
}