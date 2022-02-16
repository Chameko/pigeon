/// Possible formats for the vertex's to be represented by
#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Floatx1,
    Floatx2,
    Floatx3,
    Floatx4,
}

// wgpu conversion
impl From<&VertexFormat> for wgpu::VertexFormat {
    fn from(vfmt: &VertexFormat) -> Self {
        vfmt.to_wgpu()
    }
}

impl VertexFormat {
    /// Return bytesize. Uses F32 which have a byte size of 4
    const fn bytesize(self) -> usize {
        match self {
            VertexFormat::Floatx1 => 4,
            VertexFormat::Floatx2 => 8,
            VertexFormat::Floatx3 => 12,
            VertexFormat::Floatx4 => 16,
        }
    }

    /// Transform into wgpu counterpart [`wgpu::VertexFormat`]
    pub fn to_wgpu(&self) -> wgpu::VertexFormat {
        match self {
            VertexFormat::Floatx1 => wgpu::VertexFormat::Float32,
            VertexFormat::Floatx2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Floatx3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::Floatx4 => wgpu::VertexFormat::Float32x4,
        }
    }
}

/// Represents a vertex layout and easily able to be converted to a [wgpu::VertexBufferLayout]
#[derive(Debug, Clone)]
pub struct VertexLayout {
    wgpu_attrs: Vec<wgpu::VertexAttribute>,
    size: usize,
}

impl VertexLayout {
    // Returns an empty VertexLayout
    fn empty () -> Self {
        Self {
            wgpu_attrs: vec![],
            size: 0,
        }
    }

    pub fn to_wgpu(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: self.wgpu_attrs.as_slice(),
        }
    }

    /// Convert from an array of VertexFormat to a VertexLayout
    pub fn from<T: bytemuck::Pod>(vformats: &[VertexFormat]) -> Self {
        let mut vl = Self::empty();

        for vfmt in vformats {
            vl.wgpu_attrs.push(wgpu::VertexAttribute {
                shader_location: vl.wgpu_attrs.len() as u32,
                offset: vl.size as wgpu::BufferAddress,
                format: vfmt.to_wgpu(),
            });
            vl.size += std::mem::size_of::<T>();
        }
        log::debug!("Vertex layout: {:?}", vl);
        vl
    }
}

// Convert parrot's vertex layout to wgpu's
impl<'a> From<&'a VertexLayout> for wgpu::VertexBufferLayout<'a> {
    fn from(vl: &'a VertexLayout) -> Self {
        vl.to_wgpu()
    }
}