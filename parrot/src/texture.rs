use euclid::{Size2D};

use crate::{
    binding::Bind, device::Device, transform::ScreenSpace,
};

pub struct Texture {
    pub wgpu: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub extent: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
    pub size: Size2D<u32, ScreenSpace>
}

impl Texture {
}

impl Bind for Texture {
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::TextureView(&self.view)
        }
    }
}