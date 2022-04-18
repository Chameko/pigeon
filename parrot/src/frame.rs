use wgpu::TextureView;

use crate::{
    painter::{RenderTarget, PassOp, RenderPassExtention}
};

#[derive(Debug)]
pub struct Frame {
    pub encoder: wgpu::CommandEncoder,
}

impl Frame {
    pub fn new(encoder: wgpu::CommandEncoder) -> Self {
        Self {
            encoder
        }
    }

    /// Start a render pass on the frame. If you are using multisampling then set the frame buffer to an appropriate texture (see multisampled lines example).
    pub fn pass<'a>(
        &'a mut self,
        op: PassOp,
        view: &'a impl RenderTarget,
        frame_buffer: Option<&'a TextureView>,
    ) -> wgpu::RenderPass<'a> {
        let (pass_view, resolve_target) = match frame_buffer {
            Some(buffer) => (buffer, Some(view.color_target())),
            None => (view.color_target(), None),
        };

        wgpu::RenderPass::begin(
            &mut self.encoder,
            pass_view,
            resolve_target,
            view.depth_target(),
            op,
        )
    }

    pub fn encoder(&self) -> &wgpu::CommandEncoder {
        &self.encoder
    }

    pub fn encoder_mut(&mut self) -> &wgpu::CommandEncoder {
        &mut self.encoder
    }
}