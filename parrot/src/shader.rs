/// A shader
#[derive(Debug)]
pub struct Shader {
    /// Wrapped wgpu shader
    pub wgpu: wgpu::ShaderModule,
}

pub use wgpu::ShaderStages;