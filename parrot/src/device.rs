use euclid::Size2D;
use wgpu::util::DeviceExt;
use crate::{
    transform::ScreenSpace,
    shader::{
        Shader,
        ShaderFile
    },
    buffers::{
        vertex::VertexBuffer,
        index::IndexBuffer,
        uniform::UniformBuffer,
    },
    texture::Texture,
    sampler::Sampler,
    binding::{Binding, BindingGroupLayout},
    pipeline::PipelineLayout,
};

/// Parrot wrapper around [wgpu::Device]
pub struct Device {
    /// Wrapper around [`wgpu::Device`]
    pub wgpu: wgpu::Device,
    /// Wrapper around [`wgpu::Queue`]
    pub queue: wgpu::Queue,
    /// Wrapper around the surface (if there is one)
    pub surface: Option<wgpu::Surface>,
    size: euclid::Size2D<u32, ScreenSpace>,
}

impl Device {
    /// Create a device for a given surface
    pub async fn for_surface(
        surface: wgpu::Surface,
        adapter: &wgpu::Adapter,
    ) -> Result<Self, wgpu::RequestDeviceError> {
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("parrot device"),
                limits: wgpu::Limits::default(),
                features: wgpu::Features::empty(),
            },
            None
        ).await?;

        Ok(Self {
            wgpu: device,
            queue,
            surface: Some(surface),
            size: Size2D::default(),
        })
    }

    pub const fn device(&self) -> &wgpu::Device {
        &self.wgpu
    }

    pub const fn size(&self) -> Size2D<u32, ScreenSpace> {
        self.size
    }

    pub fn configure<T: Into<wgpu::PresentMode>>(
        &mut self,
        size: Size2D<u32, ScreenSpace>,
        mode: T,
        format: wgpu::TextureFormat,
    ) {
        let desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            present_mode: mode.into(),
            width: size.width,
            height: size.height,
        };
        self.surface.as_ref().expect("no surface found").configure(&self.wgpu, &desc);
        self.size = size;
    }

    /// Create a shader given a [`crate::shader::ShaderFile`]
    pub fn create_shader(&self, source: ShaderFile) -> Shader {
        match source {
            ShaderFile::Spirv(bytes) => unsafe{ self.create_sprv_shader(bytes) },
            ShaderFile::Wgsl(s) => self.create_wgsl_shader(s),
        }
    }

    /// Create a shader given the wgsl source code
    pub fn create_wgsl_shader(&self, source: &str) -> Shader {
        Shader {
            wgpu: self.wgpu.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(source.into())
            })
        }
    }

    /// Create a shader given the bytes of a spirv bindary.
    /// # Safety
    /// Wgpu makes no attempt to check if this is a valid spirv and can hence cause a driver crash or funky behaviour. See [`wgpu::Device::create_shader_module_spirv`]
    pub unsafe fn create_sprv_shader(&self, source: &[u8]) -> Shader {
        Shader {
            wgpu: self.wgpu.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                label: Some("Shader"),
                source: wgpu::util::make_spirv_raw(source)
            })
        }
    }

    pub fn create_vertex_buffer<T: bytemuck::Pod>(&self, verticies: &[T]) -> VertexBuffer {
        VertexBuffer {
            wgpu: self.create_buffer_from_slice(verticies, wgpu::BufferUsages::VERTEX),
            size: (verticies.len() * std::mem::size_of::<T>()) as u32
        }
    }

    pub fn create_index_buffer(&self, indicies: &[u16]) -> IndexBuffer {
        let index_buf = self.create_buffer_from_slice(indicies, wgpu::BufferUsages::INDEX);
        IndexBuffer {
            wgpu: index_buf,
            elements: indicies.len() as u32,
        }
    }

    pub fn create_uniform_buffer<T>(&self, buf: &[T]) -> UniformBuffer
    where
        T: bytemuck::Pod + 'static + Copy
    {
        UniformBuffer {
            size: std::mem::size_of::<T>(),
            count: buf.len(),
            wgpu: self.wgpu.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform buffer"),
                contents: bytemuck::cast_slice(buf),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        }
    }

    pub fn create_buffer_from_slice<T: bytemuck::Pod> (
        &self,
        slice: &[T],
        usage: wgpu::BufferUsages
    ) -> wgpu::Buffer {
        self.wgpu.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(slice),
            usage,
        })
    }

    pub fn create_texture(
        &self,
        size: euclid::Size2D<u32, ScreenSpace>,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages
    ) -> Texture {
        let texture_extent = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 0,
        };

        let texture = self.wgpu.create_texture( &wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture {
            wgpu: texture,
            view: texture_view,
            extent: texture_extent,
            format,
            size,
        }
    }

    pub fn create_sampler(&self, mag_filter: wgpu::FilterMode, min_filter: wgpu::FilterMode) -> Sampler {
        Sampler {
            wgpu: self.wgpu.create_sampler( &wgpu::SamplerDescriptor{
                label: None,
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter,
                min_filter,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_max_clamp: 100.0,
                lod_min_clamp: -100.0,
                compare: None,
                border_color: None,
                anisotropy_clamp: None,
            })
        }
    }

    pub fn create_binding_group_layout(&self, index: u32, slots: &[Binding]) -> BindingGroupLayout {
        let mut bindings = Vec::new();

        for s in slots {
            bindings.push(wgpu::BindGroupLayoutEntry{
                binding: bindings.len() as u32,
                visibility: s.stage,
                ty: s.binding.as_wgpu(),
                count: None,
            });
        }

        let layout = self.wgpu.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: bindings.as_slice()
        });
        BindingGroupLayout::new(index, layout, bindings.len())
    }

    pub fn create_pipeline_layout(&self, sets: &[&[Binding]]) -> PipelineLayout {
        let mut b_layouts = Vec::new();
        for (index, bindings) in sets.iter().enumerate() {
            b_layouts.push(self.create_binding_group_layout(index as u32, bindings))
        }
        
        PipelineLayout {
            b_layouts,
        }
    }
}