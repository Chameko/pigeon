use euclid::Size2D;
use wgpu::{TextureViewDescriptor, FilterMode, TextureFormat, RenderBundleEncoder};
use std::ops::Range;

use crate::{
    device::Device,
    vertex::VertexLayout,
    error::ParrotError,
    color::Rgba,
    transform::ScreenSpace,
    texture::Texture,
    frame::Frame,
    pipeline::{Blending, Plumber, Pipeline, PipelineLayout},
    sampler::Sampler,
    binding::{BindingGroupLayout, Bind, BindingGroup},
    buffers::{
        vertex::VertexBuffer,
        uniform::UniformBuffer,
        index::IndexBuffer, DepthBuffer, FrameBuffer,
    }, index::IndexBuffer32, 
};

/// The main interface for parrot. *Handles the rendering shenanigans so YOU don't have to*
/// 
/// # General
/// Use [`Painter::for_surface`] to create the painter and the configure the surface with [`Painter::configure`]
/// 
/// # Pipeline
/// Parrot allows you to create your own pipelines (wow).
/// See [`Plumber`] trait
/// 
/// # Usage
/// Create a frame using [`Painter::frame`].
/// To perform a render pass on the frame you'll need something that implements [`RenderTarget`].
/// Currently the only type that does is a [`RenderFrame`] which can be grabbed via [`Painter::current_frame`].
/// 
/// You can present a frame with [`Painter::present`]
#[derive(Debug)]
pub struct Painter {
    pub device: Device,
    /// Enables MSAA for values > 1
    pub(crate) sample_count: u32,
    /// The preferred texture format
    pref_format: wgpu::TextureFormat,
}

pub type PipelineFunction = fn (&Device, PipelineLayout, VertexLayout, wgpu::ShaderModule, wgpu::MultisampleState, Option<&str>) -> Pipeline;

impl Painter {
    /// Setup painter for a surface.
    pub async fn for_surface(
        surface: wgpu::Surface,
        instance: &wgpu::Instance,
        sample_count: u32,
    ) -> Result<Self, ParrotError> {
        log::info!("Creating for surface");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or(ParrotError::NoAdaptersFound)?;

        let preferred_format = surface.get_supported_formats(&adapter)[0];

        Ok(Self {
            device: Device::for_surface(surface, &adapter).await?,
            sample_count,
            pref_format: preferred_format
        })
    }

    /// Returns the preferred texture format of the surface
    pub const fn preferred_format(&self) -> wgpu::TextureFormat {
        self.pref_format
    }

    /// Get the sample count
    pub const fn sample_count(&self) -> u32 {
        self.sample_count
    }

    /// Get the size of the surface
    pub const fn size(&self) -> Size2D<u32, ScreenSpace> {
        self.device.size()
    }

    /// Updates the sample count. If you do this, you take responsibility for updating all the relevant structures such as the [`Pipeline`].
    pub fn update_sample_count(&mut self, samples: u32) {
        log::info!("Updating sample count >> Old: {} || New: {}", self.sample_count, samples);
        self.sample_count = samples;
        log::warn!("Updated sample count. The pipelines and textures must be updated")
    }

    /// Configure the surface
    pub fn configure<T: Into<wgpu::PresentMode>>(
        &mut self,
        size: Size2D<u32, ScreenSpace>,
        mode: T,
        format: wgpu::TextureFormat,
    ) {
        log::info!("Configuring for surface");
        self.device.configure(size, mode, format)
    }

    /// Get the current rendereable frame. This creates a depth buffer for itself. If you have a pipeline that doesn't support depth buffers use [`Painter::current_frame_no_depth()`]. Will present when dropped.
    pub fn current_frame(&self) -> Result<RenderFrame, wgpu::SurfaceError> {
        log::info!("Getting current frame");
        let surface = self.device.surface.as_ref().unwrap();
        let surface_texture = surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        Ok(RenderFrame {
            wgpu: Some(surface_texture),
            view,
            size: self.device.size(),
            depth: Some(self
                .device
                .create_depth_buffer(self.sample_count, Some("Current frame depth texture")))
        })
    }
    
    /// Get the current renderable frame without creating a depth buffer.
    pub fn current_frame_no_depth(&self) -> Result<RenderFrame, wgpu::SurfaceError> {
        log::info!("Getting current frame");
        let surface = self.device.surface.as_ref().unwrap();
        let surface_texture = surface.get_current_texture()?;
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        Ok(RenderFrame {
            wgpu: Some(surface_texture),
            view,
            size: self.device.size(),
            depth: None
        })
    }

    /// Create a texture
    pub fn texture(
        &self,
        size: Size2D<u32, ScreenSpace>,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
        name: Option<&str>,
        multisampled: bool,
    ) -> Texture {
        let sample_count = if multisampled { self.sample_count } else { 1 };
        self.device.create_texture(size, format, usage, name, sample_count)
    }

    /// Create a depth buffer
    pub fn depth_buffer(&self, name: Option<&str>) -> DepthBuffer {
        self.device.create_depth_buffer(self.sample_count, name)
    }

    /// Create a vertex buffer
    pub fn vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&self, verts: &[T], name: Option<&str>) -> VertexBuffer {
        self.device.create_vertex_buffer(verts, name)
    }

    /// Create a 16 bit index buffer
    pub fn index_buffer(&self, indicies: &[u16], name: Option<&str>) -> IndexBuffer {
        self.device.create_index_buffer(indicies, name)
    }

    /// Create a 32 bit index buffer
    pub fn index_buffer_32(&self, indicies: &[u32], name: Option<&str>) -> IndexBuffer32 {
        self.device.create_index_buffer_32(indicies, name)
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

    /// Create a pipeline. Has a depth texture by default.
    pub fn pipeline<T: Plumber<'static>>(&self, blending: Blending, format: TextureFormat, shader_name: Option<&str>) -> T {
        log::info!("Creating pipeline");
        let desc = T::description();
        let pipe_layout = self.device.create_pipeline_layout(desc.pipeline_layout);
        let vertex_layout = VertexLayout::from(desc.vertex_layout);
        let shader = self.device.create_shader(desc.shader, shader_name);
        let name = desc.name;

        T::setup(self.device.create_pipeline(
            pipe_layout,
            vertex_layout,
            blending,
            shader,
            format,
            wgpu::MultisampleState {
                count: self.sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            name
        ),
        &self)
    }

    /// Create a pipeline without a depth texture
    pub fn pipeline_no_depth<T: Plumber<'static>>(&self, blending: Blending, format: TextureFormat, shader_name: Option<&str>) -> T {
        log::info!("Creating pipeline with no depth buffer");
        let desc = T::description();
        let pipe_layout = self.device.create_pipeline_layout(desc.pipeline_layout);
        let vertex_layout = VertexLayout::from(desc.vertex_layout);
        let shader = self.device.create_shader(desc.shader, shader_name);
        let name = desc.name;

        T::setup(self.device.create_pipeline_no_depth(
            pipe_layout,
            vertex_layout,
            blending,
            shader,
            format,
            wgpu::MultisampleState {
                count: self.sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            name
        ),
        &self)
    }

    /// Create a pipeline, However your have the responsibility of providing the [`Pipeline`].
    pub fn custom_pipeline<T: Plumber<'static>, F>(&self, shader_name: Option<&str>, pipe: F ) -> T
    where
        F: FnOnce(&Device, PipelineLayout, VertexLayout, wgpu::ShaderModule, wgpu::MultisampleState, Option<&str>) -> Pipeline
    {
        log::info!("Creating pipeline");
        let desc = T::description();
        let pipe_layout = self.device.create_pipeline_layout(desc.pipeline_layout);
        let vertex_layout = VertexLayout::from(desc.vertex_layout);
        let shader = self.device.create_shader(desc.shader, shader_name).wgpu;
        let name = desc.name;

        let mut b_layouts = Vec::new();
        for s in pipe_layout.b_layouts.iter() {
            b_layouts.push(&s.wgpu)
        }

        T::setup(pipe(
            &self.device,
            pipe_layout,
            vertex_layout,
            shader,
            wgpu::MultisampleState {
                count: self.sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            name
        ),
        &self)
    }

    /// Update the pipeline
    pub fn update_pipeline<'a, T: Plumber<'a>>(&mut self, pipe: &'a mut T, prep: T::PrepareContext) {
        for (buffer, uniforms) in pipe.prepare(prep, self) {
            log::info!("Updating pipeline -------");
            if let Some(b) = self.update_buffer::<T::Uniforms>(uniforms.as_slice(), buffer) {
                *buffer = b;
            }
        }
    }

    /// Update a uniform buffer
    pub fn update_buffer<T: bytemuck::Pod + Copy + 'static>(&mut self, data: &[T], buffer: &mut UniformBuffer) -> Option<UniformBuffer> {
        let bytes: &[u8] = bytemuck::cast_slice(data);
        // Check if the uniform buffer is too big
        if bytes.len() <= buffer.size * buffer.count {
            log::info!("Updating uniform buffer >> Current max: {} || Updated size: {}",buffer.size * buffer.count, bytes.len());
            self.device.update_buffer(data, buffer);
            None
        } else {
            log::info!("Creating new uniform buffer >> Current max: {} || Updated size: {}",buffer.size * buffer.count, bytes.len());
            if let Some(name) = buffer.name.clone() {
                Some(self.uniform_buffer(data, Some(name.as_str())))
            } else {
                Some(self.uniform_buffer(data, None))
            }
        }
    }

    /// Updates the vertex buffer or, if too big, creates a new one big enough to fit the data
    pub fn update_vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&mut self, vertices: &[T], buffer: &mut VertexBuffer) -> Option<VertexBuffer> {
        let bytes: &[u8] = bytemuck::cast_slice(vertices);
        // Check if the vertex buffer is big enough to fit the vertices
        if bytes.len() <= buffer.size as usize {
            log::info!("Updating vertex buffer >> Current max: {} || Updated size: {}", buffer.size, bytes.len());
            self.device.update_vertex_buffer(vertices, buffer);
            None
        } else {
            log::info!("Creating new vertex buffer >> Current max: {} || Updated size: {}", buffer.size, bytes.len());
            if let Some(name) = buffer.name.clone() {
                Some(self.vertex_buffer(vertices, Some(name.as_str())))
            } else {
                Some(self.vertex_buffer(vertices, None))
            }
        }
    }
    
    /// Updates an index buffer 32 or, if too big, creates a new one big enough to fit the new data
    pub fn update_index_buffer_32(&mut self, indicies:Vec<u32>, buffer: &mut IndexBuffer32) -> Option<IndexBuffer32> {
        // Check if the index buffer is big enough to fit the indicies
        if indicies.len() <= buffer.size as usize {
            log::info!("Updating index buffer 32 >> Current size: {} || Updated size: {}", buffer.size, indicies.len());
            self.device.update_index_buffer_32(indicies, buffer);
            None
        } else {
            log::info!("Creating new index buffer >> Current size: {} || Updated size: {}", buffer.size, indicies.len());
            if let Some(name) = buffer.name.clone() {
                Some(self.index_buffer_32(indicies.as_slice(), Some(name.as_str())))
            } else {
                Some(self.index_buffer_32(indicies.as_slice(), None))
            }
        }
    }

    /// Updates an index buffer or, if too big, creates a new one big enough to fit the new data
    pub fn update_index_buffer(&mut self, indicies: Vec<u16>, buffer: &mut IndexBuffer) -> Option<IndexBuffer> {
        // Check if the index buffer is big enough to fit the indicies
        if indicies.len() <= buffer.size as usize {
            log::info!("Updating index buffer >> Current size: {} || Updated size: {}", buffer.size, indicies.len());
            self.device.update_index_buffer(indicies, buffer);
            None
        } else {
            log::info!("Creating new index buffer >> Current size: {} || Updated size: {}", buffer.size, indicies.len());
            if let Some(name) = buffer.name.clone() {
                Some(self.index_buffer(indicies.as_slice(), Some(name.as_str())))
            } else {
                Some(self.index_buffer(indicies.as_slice(), None))
            }
        }
    }

    /// Creates a [`FrameBuffer`] with a depth texture
    pub fn create_frame_buffer(&self, size: Size2D<u32, ScreenSpace>, format: TextureFormat, name: Option<&str>) -> FrameBuffer {
        self.device.create_frame_buffer(size, format, self.sample_count, name, true)
    }

    /// Creates a [`FrameBuffer`] with **no** depth texture
    pub fn create_frame_buffer_no_depth(&self, size: Size2D<u32, ScreenSpace>, format: TextureFormat, name: Option<&str>) -> FrameBuffer {
        self.device.create_frame_buffer(size, format, self.sample_count, name, false)
    }

    /// Get a frame
    pub fn frame(&mut self) -> Frame {
        log::info!("Created frame");
        let encoder = self.device.create_command_encoder();
        Frame::new(encoder)
    }

    /// Present a frame
    pub fn present(&mut self, frame: Frame) {
        log::info!("Submitting frame commands");
        self.device.submit(vec![frame.encoder.finish()]);
    }

    /// Create a [`wgpu::RenderBundleEncoder`] for creating render bundles
    pub fn create_render_bundle(&self, name: Option<&str>, format: wgpu::TextureFormat) -> wgpu::RenderBundleEncoder {
        self.device.create_render_bundle_encoder(format, name, self.sample_count)
    }
}

/// Can be transformed into a redner pass via [`Frame`].
pub trait RenderTarget {
    /// Color component
    fn color_target(&self) -> &wgpu::TextureView;
    /// Depth component
    fn depth_target(&self) -> Option<&wgpu::TextureView>;
}

/// A frame that can be rendered to. Presents when dropped.
pub struct RenderFrame {
    pub view: wgpu::TextureView,
    pub wgpu: Option<wgpu::SurfaceTexture>,
    pub size: Size2D<u32, ScreenSpace>,
    pub depth: Option<DepthBuffer>,

}

impl RenderTarget for RenderFrame {
    fn color_target(&self) -> &wgpu::TextureView {
        &self.view
    }
    
    fn depth_target(&self) -> Option<&wgpu::TextureView> {
        if let Some(buff) = &self.depth {
            Some(&buff.texture.view) 
        } else {
            None
        }
    }
}

impl Drop for RenderFrame {
    fn drop(&mut self) {
        if let Some(wgpu) = self.wgpu.take() {
            log::info!("Presenting");
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
        depth: Option<&'a wgpu::TextureView>,
        op: PassOp
    ) -> Self;

    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T);

    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]);

    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer);
    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer);
    fn set_parrot_index_buffer_32(&mut self, index_buf: &'a IndexBuffer32);
    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>);
    fn draw_parrot_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>);
}

impl<'a> RenderPassExtention<'a> for wgpu::RenderPass<'a> {
    fn begin(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, resolve_target: Option<&'a wgpu::TextureView>, depth: Option<&'a wgpu::TextureView>, op: PassOp) -> Self {
        log::info!("Began render pass");
        if let Some(depth) = depth {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: op.into(),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: true,
                    })
                }),
            })
        } else {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: op.into(),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            })
        }
    }

    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T) {
        log::info!("Set pipeline");
        self.set_pipeline(&pipeline.pipeline.wgpu);
        for binding in &pipeline.bindings {
            self.set_binding(binding, &[]);
        }
    }

    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]) {
        log::info!("Set binding group >> Index: {:?}", group.set_index);
        self.set_bind_group(group.set_index, &group.wgpu, offsets);
    }

    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer) {
        log::info!("Set index buffer >> Name: {:?}", index_buf.name);
        self.set_index_buffer(index_buf.slice(), wgpu::IndexFormat::Uint16)
    }

    fn set_parrot_index_buffer_32(&mut self, index_buf: &'a IndexBuffer32) {
        log::info!("Set index buffer 32 >> Name: {:?}", index_buf.name);
        self.set_index_buffer(index_buf.slice(), wgpu::IndexFormat::Uint32)
    }

    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer) {
        log::info!("Set vertex buffer >> Name: {:?}", vertex_buf.name);
        self.set_vertex_buffer(0, vertex_buf.slice())
    }

    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>) {
        log::info!("Drawing buffer range >> Name: {:?} || Range: {:?}", buf.name, range);
        self.set_parrot_vertex_buffer(buf);
        self.draw(range, 0..1);
    }

    fn draw_parrot_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>) {
        log::info!("Drawing indexed >> Indicies: {:?} || Instances: {:?}", indicies, instances);
        self.draw_indexed(indicies, 0, instances)
    }
}

/// Extention trait for the render bundle
pub trait RenderBundleExtention<'a> {
    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T);
    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]);
    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer);
    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer);
    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>);
    fn draw_parrot_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>);
}

impl<'a> RenderBundleExtention<'a> for RenderBundleEncoder<'a> {
    fn set_parrot_vertex_buffer(&mut self, vertex_buf: &'a VertexBuffer) {
        log::info!("Set render bundle vertex buffer >> Name: {:?}", vertex_buf.name);
        self.set_vertex_buffer(0, vertex_buf.slice());
    }

    fn set_parrot_index_buffer(&mut self, index_buf: &'a IndexBuffer) {
        log::info!("Set render bundle index buffer >> Name: {:?}", index_buf.name);
        self.set_index_buffer(index_buf.slice(), wgpu::IndexFormat::Uint16)
    }

    fn set_binding(&mut self, group: &'a BindingGroup, offsets: &[u32]) {
        log::info!("Set render bundle binding");
        self.set_bind_group(group.set_index, &group.wgpu, offsets);
    }

    fn set_parrot_pipeline<'b, T: Plumber<'b>>(&mut self, pipeline: &'a T) {
        log::info!("Set render bundle pipeline");
        self.set_pipeline(&pipeline.pipeline.wgpu);
        for binding in &pipeline.bindings {
            self.set_binding(binding, &[]);
        }
    }

    fn draw_buffer_range(&mut self, buf: &'a VertexBuffer, range: Range<u32>) {
        log::info!("Render bundle drawing buffer range >> Name: {:?} || Range: {:?}", buf.name, range);
        self.set_parrot_vertex_buffer(buf);
        self.draw(range, 0..1);
    }

    fn draw_parrot_indexed(&mut self, indicies: Range<u32>, instances: Range<u32>) {
        log::info!("Render bundle drawing indexed >> Indicies: {:?} || Instances: {:?}", indicies, instances);
        self.draw_indexed(indicies, 0, instances)
    }
}