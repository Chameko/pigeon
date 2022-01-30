use euclid::{Rect, Size2D};
use wgpu::{TextureViewDescriptor, FilterMode, TextureFormat};

use crate::{
    device::Device,
    vertex::VertexLayout,
    error::ParrotError,
    transform::ScreenSpace,
    texture::Texture,
    pipeline::{Blending, Plumber},
    sampler::Sampler,
    binding::{BindingGroupLayout, Bind, BindingGroup},
    buffers::{
        vertex::VertexBuffer,
        uniform::UniformBuffer,
    }, 
};

/// The main interface for parrot. > Handles the rendering shenanigans so YOU don't have to TM 
pub struct Painter {
    device: Device,
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
    ) -> Texture {
        self.device.create_texture(size, format, usage)
    }

    /// Create a vertex buffer
    pub fn vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&self, verts: &[T]) -> VertexBuffer {
        self.device.create_vertex_buffer(verts)
    }

    /// Create a uniform buffer
    pub fn uniform_buffer<T: bytemuck::Pod + Copy + 'static>(&self, buf: &[T]) -> UniformBuffer {
        self.device.create_uniform_buffer(buf)
    }

    /// Createa a binding group
    pub fn binding_group(&self, layout: &BindingGroupLayout, binds: &[&dyn Bind]) -> BindingGroup {
        self.device.create_binding_group(layout, binds)
    }

    /// Create a sampler
    pub fn sampler(&self, min_filter: FilterMode, mag_filter: FilterMode) -> Sampler {
        self.device.create_sampler(min_filter, mag_filter)
    }

    /// Create a pipeline
    pub fn pipeline<T: Plumber<'static>>(&self, blending: Blending, format: TextureFormat) -> T {
        let desc = T::description();
        let pipe_layout = self.device.create_pipeline_layout(desc.pipeline_layout);
        let vertex_layout = VertexLayout::from(desc.vertex_layout);
        let shader = self.device.create_shader(desc.shader);

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
            }
        ),
        &self.device)
    }

    pub fn update_pipeline<'a, T: Plumber<'a>>(&mut self, pipe: &'a T, p: T::PrepareContext) {
        if let Some((buffer, uniforms)) = pipe.prepare(p) {
            self.device.update_uniform_buffer::<T::Uniforms>(uniforms.as_slice(), buffer);
        }
    }

    // present

    // submut
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