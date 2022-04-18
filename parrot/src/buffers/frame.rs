use crate::{
    binding::Bind,
    buffers::DepthBuffer,
    painter::RenderTarget,
    texture::Texture,
};

#[derive(Debug)]
pub struct FrameBuffer {
    pub texture: Texture,
    pub depth: Option<DepthBuffer>,
}

impl FrameBuffer {
    /// Amount of pixels in the frame buffer
    pub fn size(&self) -> u32 {
        self.texture.size.area()
    }

    /// Framebuffer width in pixels
    pub fn width(&self) -> u32 {
        self.texture.size.width
    }

    /// Framebuffer height in pixels
    pub fn height(&self) -> u32 {
        self.texture.size.height
    }
}

impl RenderTarget for FrameBuffer {
    fn color_target(&self) -> &wgpu::TextureView {
        &self.texture.view
    }

    fn depth_target(&self) -> Option<&wgpu::TextureView> {
        if let Some(buff) = &self.depth {
            Some(&buff.texture.view)
        } else {
            None
        }
    }
}

impl Bind for FrameBuffer {
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry{
            binding: index,
            resource: wgpu::BindingResource::TextureView(&self.texture.view)
        }
    }
}