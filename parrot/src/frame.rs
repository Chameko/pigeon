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

    pub fn pass<'a>(
        &'a mut self,
        op: PassOp,
        view: &'a impl RenderTarget,
    ) -> wgpu::RenderPass<'a> {
        let (pass_view, resolve_target) = (view.color_target(), None);

        wgpu::RenderPass::begin(
            &mut self.encoder,
            pass_view,
            resolve_target,
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