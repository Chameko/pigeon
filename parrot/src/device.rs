use euclid::Size2D;
use crate::{
    transform::ScreenSpace,
    shader::Shader,
};

/// Parrot wrapper around [wgpu::Device]
pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
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
            device,
            queue,
            surface: Some(surface),
            size: Size2D::default(),
        })
    }

    pub const fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub const fn size(&self) -> Size2D<u32, ScreenSpace> {
        self.size
    }

    /// Create a shader given the wgsl source code
    pub fn create_wgsl_shader(&self, source: &str) -> Shader {
        Shader {
            wgpu: self.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
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
            wgpu: self.device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                label: Some("Shader"),
                source: wgpu::util::make_spirv_raw(source)
            })
        }
    }
}