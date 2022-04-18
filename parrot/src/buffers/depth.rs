use crate::texture::Texture;

/// Depth buffer
#[derive(Debug)]
pub struct DepthBuffer {
    pub texture: Texture,
}

impl DepthBuffer {
    pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
}