use euclid::Size2D;
use wgpu::util::DeviceExt;
use crate::{
    vertex::VertexLayout,
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
    binding::{Binding, BindingGroupLayout, Bind, BindingGroup},
    pipeline::{PipelineLayout, Pipeline, Blending, Set},
};

/// Parrot wrapper around [wgpu::Device]
#[derive(Debug)]
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

    /// Configure the surface
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

    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.wgpu.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        })
    }

    pub fn submit<I: IntoIterator<Item = wgpu::CommandBuffer>>(&mut self, cmds: I) {
        self.queue.submit(cmds);
    }

    /// Create a shader given a [`crate::shader::ShaderFile`]
    pub fn create_shader(&self, source: ShaderFile, name: Option<&str>) -> Shader {
        match source {
            ShaderFile::Spirv(bytes) => self.create_sprv_shader(bytes, name),
            ShaderFile::Wgsl(s) => self.create_wgsl_shader(s, name),
        }
    }

    /// Create a shader given the wgsl source code
    pub fn create_wgsl_shader(&self, source: &str, name: Option<&str>) -> Shader {
        Shader {
            wgpu: self.wgpu.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: name,
                source: wgpu::ShaderSource::Wgsl(source.into())
            })
        }
    }

    /// Create a shader given the bytes of a spirv bindary.
    /// # Safety
    /// Wgpu makes no attempt to check if this is a valid spirv and can hence cause a driver crash or funky behaviour. See [`wgpu::Device::create_shader_module_spirv`]
    pub fn create_sprv_shader(&self, source: &[u8], name: Option<&str>) -> Shader {
        Shader {
            wgpu: self.wgpu.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: name,
                source: wgpu::util::make_spirv(source),
            })
        }
    }

    pub fn create_vertex_buffer<T: bytemuck::Pod>(&self, verticies: &[T], name: Option<&str>) -> VertexBuffer {
        VertexBuffer {
            wgpu: self.create_buffer_from_slice(verticies, wgpu::BufferUsages::VERTEX, 
                name),
            size: (verticies.len() * std::mem::size_of::<T>()) as u32
        }
    }

    pub fn create_index_buffer(&self, indicies: &[u16], name: Option<&str>) -> IndexBuffer {
        let index_buf = self.create_buffer_from_slice(indicies, wgpu::BufferUsages::INDEX, name);
        IndexBuffer {
            wgpu: index_buf,
            elements: indicies.len() as u32,
        }
    }

    pub fn create_uniform_buffer<T>(&self, buf: &[T], name: Option<&str>) -> UniformBuffer
    where
        T: bytemuck::Pod + 'static + Copy
    {
        UniformBuffer {
            size: std::mem::size_of::<T>(),
            count: buf.len(),
            wgpu: self.wgpu.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: name,
                contents: bytemuck::cast_slice(buf),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        }
    }

    pub fn create_buffer_from_slice<T: bytemuck::Pod> (
        &self,
        slice: &[T],
        usage: wgpu::BufferUsages,
        name: Option<&str>
    ) -> wgpu::Buffer {
        self.wgpu.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: name,
            contents: bytemuck::cast_slice(slice),
            usage,
        })
    }

    /// Create a texture
    pub fn create_texture(
        &self,
        size: euclid::Size2D<u32, ScreenSpace>,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
        name: Option<&str>,
    ) -> Texture {
        let texture_extent = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 0,
        };

        let texture = self.wgpu.create_texture( &wgpu::TextureDescriptor {
            label: name,
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

    pub fn create_sampler(&self, mag_filter: wgpu::FilterMode, min_filter: wgpu::FilterMode, name: Option<&str>) -> Sampler {
        Sampler {
            wgpu: self.wgpu.create_sampler( &wgpu::SamplerDescriptor{
                label: name,
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

    pub fn create_binding_group_layout(&self, index: u32, slots: &[Binding], name: Option<&str>) -> BindingGroupLayout {
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
            label: name,
            entries: bindings.as_slice()
        });
        BindingGroupLayout::new(index, layout, bindings.len())
    }

    pub fn create_binding_group(&self, layout: &BindingGroupLayout, binds: &[&dyn Bind], name: Option<&str>) -> BindingGroup {
        assert_eq!(binds.len(), layout.size, "Layout slot doesn't match bindings");

        let mut bindings = Vec::new();

        for (i, b) in binds.iter().enumerate() {
            bindings.push(b.binding(i as u32));
        }

        BindingGroup::new(
            layout.set_index,
            self.wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout.wgpu,
                label: name,
                entries: bindings.as_slice()
            }),
        )
    }

    /// Create a pipeline layout from a set of bindings
    pub fn create_pipeline_layout(&self, sets: Option<&[Set<'_>]>) -> PipelineLayout {
        let mut b_layouts = Vec::new();
        if let Some(ss) = sets {
            for (index, bindings) in ss.iter().enumerate() {
                log::debug!("Created binding group layout");
                b_layouts.push(self.create_binding_group_layout(index as u32, bindings.0, bindings.1))
            }
        }
        
        PipelineLayout {
            b_layouts,
        }
    }

    /// Updates a uniform buffer
    pub fn update_buffer<T: bytemuck::Pod + Copy + 'static>(&self, slice: &[T], buf: &mut UniformBuffer) {
        self.queue.write_buffer(&buf.wgpu, 0, bytemuck::cast_slice(slice));
        buf.size = std::mem::size_of::<T>();
        buf.count = slice.len();
    }

    /// Updates a vertex buffer
    pub fn update_vertex_buffer<T: bytemuck::Pod + Copy + 'static>(&self, vertices: &[T], buf: &mut VertexBuffer) {
        self.queue.write_buffer(&buf.wgpu, 0, bytemuck::cast_slice(vertices));
        // Update the size of it.
        buf.size = (vertices.len() * std::mem::size_of::<T>()) as u32
    }

    /// Update a index buffer
    pub fn update_index_buffer(&self, indicies: &[u16], buf: &mut IndexBuffer) {
        self.queue.write_buffer(&buf.wgpu, 0, bytemuck::cast_slice(indicies));
        buf.elements = indicies.len() as u32
    }

    /// Create a pipeline
    pub fn create_pipeline(
        &self,
        pipeline_layout: PipelineLayout,
        vertex_layout: VertexLayout,
        blending: Blending,
        shader: &Shader,
        tex_format: wgpu::TextureFormat,
        multisample: wgpu::MultisampleState,
        name: String
    ) -> Pipeline {
        let vertex_attrs = vertex_layout.to_wgpu();
        let mut b_layouts = Vec::new();

        for s in pipeline_layout.b_layouts.iter() {
            b_layouts.push(&s.wgpu);
        }

        let layout = &self.wgpu.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{} pipeline layout", &name)),
            bind_group_layouts: b_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let (src_factor, dst_factor, operation) = blending.as_wgpu();

        // I like your funny words magic man
        let targets = [wgpu::ColorTargetState {
            format: tex_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor,
                    dst_factor,
                    operation
                },
                alpha: wgpu::BlendComponent {
                    src_factor,
                    dst_factor,
                    operation
                },
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }];

        let n = format!("{} pipeline", &name);

        let desc = wgpu::RenderPipelineDescriptor {
            label: Some(&n),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader.wgpu,
                entry_point: "vs_main",
                buffers: &[vertex_attrs],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module: &shader.wgpu,
                entry_point: "fs_main",
                targets: &targets,
            }),
            multiview: None,
        };

        let wgpu = self.wgpu.create_render_pipeline(&desc);

        Pipeline {
            layout: pipeline_layout,
            vertex_layout,
            wgpu,
        }
    }
}