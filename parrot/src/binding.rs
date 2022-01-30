use crate::shader::ShaderStages;

/// A group of bindings
#[derive(Debug)]
pub struct BindingGroup {
    /// Wrapped wgpu component
    pub wgpu: wgpu::BindGroup,
    pub set_index: u32
}

impl BindingGroup {
    pub fn new(set_index: u32, wgpu: wgpu::BindGroup) -> Self {
        Self {
            set_index,
            wgpu,
        }
    }
}

/// The layout of [`BindingGroup`].
#[derive(Debug)]
pub struct BindingGroupLayout {
    pub wgpu: wgpu::BindGroupLayout,
    pub size: usize,
    pub set_index: u32,
}

impl BindingGroupLayout {
    pub fn new(set_index: u32, wgpu: wgpu::BindGroupLayout, size: usize) -> Self {
        Self {
            wgpu,
            size,
            set_index,
        }
    }
}

/// Represents an object that can be bound
pub trait Bind {
    /// Bind an object
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry;
}

#[derive(Debug)]
pub enum BindingType {
    UniformBuffer,
    Sampler,
    Texture,
}

impl BindingType {
    pub fn as_wgpu(&self) -> wgpu::BindingType {
        match self {
            BindingType::UniformBuffer => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            BindingType::Sampler => wgpu::BindingType::Sampler(
                wgpu::SamplerBindingType::Filtering
            ),
            BindingType::Texture => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float{ filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false // TODO: add multisampling
            }
        }
    }
}

impl From<&BindingType> for wgpu::BindingType {
    fn from(bt: &BindingType) -> Self {
        bt.as_wgpu()
    }
}

impl From<BindingType> for wgpu::BindingType {
    fn from(bt: BindingType) -> Self {
        bt.as_wgpu()
    }
}

#[derive(Debug)]
pub struct Binding {
    pub binding: BindingType,
    pub stage: ShaderStages,
}