/// A shader
#[derive(Debug)]
pub struct Shader {
    /// Wrapped wgpu shader
    pub wgpu: wgpu::ShaderModule,
}

/// Represents a shader file. I recommend using wgsl as it is first class supported and spirv is planned to be
/// depreciated. The entry points for the vertex shader is vs_main and fragment shader fs_main
#[derive(Debug, Clone)]
pub enum ShaderFile {
    Wgsl(&'static str),
    Spirv(&'static [u8])
}

pub use wgpu::ShaderStages;