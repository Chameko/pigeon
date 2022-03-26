use std::num::NonZeroU64;

use crate::binding::Bind;
#[derive(Debug)]
pub struct UniformBuffer {
    pub wgpu: wgpu::Buffer,
    pub size: usize,
    pub count: usize,
}

impl Bind for UniformBuffer {
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index as u32,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.wgpu,
                offset: 0,
                size: NonZeroU64::new((self.size * self.count) as u64)
            })
        }
    }
}