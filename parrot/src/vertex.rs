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
        match vfmt {
            VertexFormat::Floatx1 => wgpu::VertexFormat::Float32,
            VertexFormat::Floatx2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Floatx3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::Floatx4 => wgpu::VertexFormat::Float32x4,
        }
    }
}

impl VertexFormat {
    // Return bytesize. Uses F32 which have a byte size of 4
    const fn bytesize(self) -> usize {
        match self {
            VertexFormat::Floatx1 => 4,
            VertexFormat::Floatx2 => 8,
            VertexFormat::Floatx3 => 12,
            VertexFormat::Floatx4 => 16,
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
}

// Convert array of Vertex formats to a VertexLayout
impl From<&[VertexFormat]> for VertexLayout {
    fn from(vformats: &[VertexFormat]) -> Self {
        let mut vl = Self::empty();

        for vfmt in vformats {
            vl.wgpu_attrs.push(wgpu::VertexAttribute {
                shader_location: vl.wgpu_attrs.len() as u32,
                offset: vl.size as wgpu::BufferAddress,
                format: wgpu::VertexFormat::from(vfmt),
            });
            vl.size += vfmt.bytesize();
        }
        vl
    }
}

// Convert parrot's vertex layout to wgpu's
impl<'a> From<&'a VertexLayout> for wgpu::VertexBufferLayout<'a> {
    fn from(vl: &'a VertexLayout) -> Self {
        wgpu::VertexBufferLayout {
            array_stride: vl.size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vl.wgpu_attrs.as_slice(),
        }
    }
}