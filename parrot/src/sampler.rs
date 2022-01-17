use crate::binding::Bind;

/// Represents a sampler
/// 
/// Defines how a pipeline will sample a texture view
pub struct Sampler {
    pub wgpu: wgpu::Sampler,
}

impl Bind for Sampler {
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::Sampler(&self.wgpu),
        }
    }
}