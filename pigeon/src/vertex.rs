
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: [x,y],
        }
    }
}