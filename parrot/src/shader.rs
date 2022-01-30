/// A shader
#[derive(Debug)]
pub struct Shader {
    /// Wrapped wgpu shader
    pub wgpu: wgpu::ShaderModule,
}

#[derive(Debug, Clone)]
pub enum ShaderFile {
    Wgsl(&'static str),
    Spirv(&'static [u8])
}

pub use wgpu::ShaderStages;