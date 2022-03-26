use euclid::Size2D;
use wgpu::{TextureViewDescriptor, FilterMode, TextureFormat};
use std::ops::Range;

use crate::{
    device::Device,
    vertex::VertexLayout,
    error::ParrotError,
    color::Rgba,
    transform::ScreenSpace,
    texture::Texture,
    frame::Frame,
    pipeline::{Blending, Plumber},
    sampler::Sampler,
    binding::{BindingGroupLayout, Bind, BindingGroup},
    buffers::{
        vertex::VertexBuffer,
        uniform::UniformBuffer,
        index::IndexBuffer,
    }, 
};

/// The main interface for parrot. *Handles the rendering shenanigans so YOU don't have to*TM
/// 
/// # Setup
/// ## General
/// Use [`Painter::for_surface`] to create the painter and the configure the surface with [`Painter::configure`]
/// 
/// ## Pipelie
/// Parrot allows you to create your own pipelines (wow).
/// See [`Plumber`] trait
/// 
/// # Usage
/// Create a frame using [`Painter::frame`].
/// To perform a render pass on the frame you'll need to grab something that implements [`RenderTarget`].
/// Currently the only type that does so is a [`RenderFrame`] which can be grabbed via [`Painter::current_frame`].
/// 
/// You can present a frame with [`Painter::present`]
#[derive(Debug)]
pub struct Painter {
    pub device: Device,
}

impl Painter {
    /// Setup painter for a surface
    pub async fn for_surface(
        surface: wgpu::Surface,
        instance: &wgpu::Instance,
    ) -> Result<Self, ParrotError> {
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or(ParrotError::NoAdaptersFound)?;

        Ok(Self {
            device: Device::for_surface(surface, &adapter).await?,
        })
    }

    /// Configure the surface
    pub fn configure<T: Into<wgpu::PresentMode>>(
        &mut self,
        size: Size2D<u32, ScreenSpace>,
        mode: T,
        format: wgpu::TextureFormat,
    ) {
        self.device.configure(size, mode, format)
    }

    /// Get the current rendereable frame. Will present when dropped.
    pub fn current_frame(&self) -> Result<RenderFrame, wgpu::SurfaceError> {
        let surface = self.device.surface.as_ref().unwrap();
        let surface_texture = surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        Ok(RenderFrame {
            wgpu: Some(surface_texture),
            view,
            size: self.device.size(),
        })
    }

    /// Create a texture
    pub fn texture(
        &self,
        size: Size2D<u32, ScreenSpace>,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
        name: Option<&str>,
    ) -> Texture {
        self.device.create_texture(size, format, usage, name)
    }

    /// Create a vertex buffer
    pub fn vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&self, verts: &[T], name: Option<&str>) -> VertexBuffer {
        self.device.create_vertex_buffer(verts, name)
    }

    /// Create a index buffer
    pub fn index_buffer(&self, indicies: &[u16], name: Option<&str>) -> IndexBuffer {
        self.device.create_index_buffer(indicies, name)
    }

    /// Create a uniform buffer
    pub fn uniform_buffer<T: bytemuck::Pod + Copy + 'static>(&self, buf: &[T], name: Option<&str>) -> UniformBuffer {
        self.device.create_uniform_buffer(buf, name)
    }

    /// Create a binding group
    pub fn binding_group(&self, layout: &BindingGroupLayout, binds: &[&dyn Bind], name: Option<&str>) -> BindingGroup {
        self.device.create_binding_group(layout, binds, name)
    }

    /// Create a sampler
    pub fn sampler(&self, min_filter: FilterMode, mag_filter: FilterMode, name: Option<&str>) -> Sampler {
        self.device.create_sampler(min_filter, mag_filter, name)
    }

    /// Create a pipeline
    pub fn pipeline<T: Plumber<'static>>(&self, blending: Blending, format: TextureFormat, shader_name: Option<&str>) -> T {
        let desc = T::description();
        let pipe_layout = self.device.create_pipeline_layout(desc.pipeline_layout);
        let vertex_layout = VertexLayout::from::<T::Vertex>(desc.vertex_layout);
        let shader = self.device.create_shader(desc.shader, shader_name);

        T::setup(self.device.create_pipeline(
            pipe_layout,
            vertex_layout,
            blending,
            &shader,
            format,
            wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            T::name()
        ),
        &self.device)
    }

    /// Update the pipeline
    pub fn update_pipeline<'a, T: Plumber<'a>>(&mut self, pipe: &'a mut T, p: T::PrepareContext) {
        for (buffer, uniforms) in pipe.prepare(p) {
            self.device.update_buffer::<T::Uniforms>(uniforms.as_slice(), buffer);
        }
    }

    /// Update a uniform buffer
    pub fn update_buffer<T: bytemuck::Pod + Copy + 'static>(&mut self, data: &[T], buffer: &mut UniformBuffer) {
        self.device.update_buffer(data, buffer);
    }

    /// Update a vertex buffer
    pub fn update_vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&mut self, vertices: &[T], buffer: &mut VertexBuffer) {
        self.device.update_vertex_buffer(vertices, buffer);
    }
    
    /// Update a index buffer
    pub fn update_index_buffer(&mut self, indicies: &[u16], buffer: &mut IndexBuffer) {
        self.device.update_index_buffer(indicies, buffer);
    }

    /// Get a frame
    pub fn frame(&mut self) -> Frame {
        let encoder = self.device.create_command_encoder();
        Frame::new(encoder)
    }

    /// Present a frame
    pub fn present(&mut self, frame: Frame) {
        self.device.submit(vec![frame.encoder.finish()]);
    }
}

/// Can be rendered into a pass.
pub trait RenderTarget {
    /// Color component
    fn color_target(&self) -> &wgpu::TextureView;
}

/// A frame that can be rendered to. Presents when dropped.
pub struct RenderFrame {
    pub view: wgpu::TextureView,
    pub wgpu: Option<wgpu::SurfaceTexture>,
    pub size: Size2D<u32, ScreenSpace>,
}

impl RenderTarget for RenderFrame {
    fn color_target(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl Drop for RenderFrame {
    fn drop(&mut self) {
        if let Some(wgpu) = self.wgpu.take() {
            wgpu.present();
        }
    }
}

/// Wrapper around [`wgpu::LoadOp`]. Instructs wgpu to either clear the screen with a color, or load from memory
#[derive(Debug)]
pub enum PassOp {
    Clear(Rgba),
    Load(),
}

impl PassOp {
    fn to_wgpu(&self) -> wgpu::LoadOp<wgpu::Color> {
        match self {
            PassOp::Clear(color) => wgpu::LoadOp::Clear((*color).into()),
            PassOp::Load() => wgpu::LoadOp::Load
        }
    }
}

impl From<PassOp> for wgpu::LoadOp<wgpu::Color> {
    fn from(op: PassOp) -> Self {
        op.to_wgpu()
    }
}

/// An extention on [`wgpu::RenderPass`] allowing it to perform actions on parrot's types
pub trait RenderPassExtention<'a> {
    fn begin(
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        resolve_target: Option<&'a wgpu::TextureView>,
        op: PassOp
    ) -> Self;

    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T);

    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]);

    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer);
    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer);
    fn draw_buffer(&mut self, buf: &'a VertexBuffer);
    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>);
    fn draw_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>);
}

impl<'a> RenderPassExtention<'a> for wgpu::RenderPass<'a> {
    fn begin(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, resolve_target: Option<&'a wgpu::TextureView>, op: PassOp) -> Self {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target,
                ops: wgpu::Operations {
                    load: op.into(),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        })
    }

    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T) {
        self.set_pipeline(&pipeline.pipeline.wgpu);
        if let Some(bindings) = &pipeline.bindings {
            self.set_binding(bindings, &[]);
        }
    }

    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]) {
        self.set_bind_group(group.set_index, &group.wgpu, offsets);
    }

    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer) {
        self.set_index_buffer(index_buf.slice(), wgpu::IndexFormat::Uint16)
    }

    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer) {
        self.set_vertex_buffer(0, vertex_buf.slice())
    }

    fn draw_buffer(&mut self, buf: &'a VertexBuffer) {
        self.set_parrot_vertex_buffer(buf);
        self.draw(0..buf.size, 0..1);
    }

    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>) {
        self.set_parrot_vertex_buffer(buf);
        self.draw(range, 0..1);
    }

    fn draw_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>) {
        self.draw_indexed(indicies, 0, instances)
    }
}