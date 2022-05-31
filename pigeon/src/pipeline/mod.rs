pub mod quad;
pub mod triangle;
use crate::graphics::Texture;
use std::rc::Rc;
use parrot::{transform::{ScreenSpace, WorldSpace}, Painter};
use euclid::Transform3D;

pub use quad::QuadPipe;
pub use triangle::TrianglePipe;
use wgpu::RenderPass;

/// Pigeon comes with two built in pipelines [QuadPipe] and [TrianglePipe]. Otherwise you can create
/// your own using the [Render] trait.

/// Contains the essential details needed by the pipelines to render the shape
#[derive(Debug)]
pub struct Breakdown<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy> {
    pub vertices: Vec<T>,
    pub indicies: Vec<u16>,
    pub texture: Option<Rc<Texture>>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy> Breakdown<T> {
    pub fn append(self, vec: &mut Vec<Breakdown<T>>) {
        vec.push(self);
    }
}

/// Defines how a pipeline should render itself when its called to draw
pub trait Render {
    type Vertex: bytemuck::Pod + bytemuck::Zeroable + Clone + Copy;

    fn render<'a>(&'a mut self, paint: &mut Painter, pass: &mut RenderPass<'a>);
}

/// The render information passed of to the pipelines
pub type RenderInformation<T> = (Vec<Breakdown<T>>, Transform3D<f32, WorldSpace, ScreenSpace>);

/// The size of the vertex buffer when first created
pub const VERTEX_INIT_SIZE: u32 = 10000;
/// The size of the index buffer when first created
pub const INDEX_INIT_SIZE: u32 = 10000;