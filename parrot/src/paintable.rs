use crate::{buffers::vertex::VertexBuffer, painter::Painter};


/// Trait for things that are paintable.
pub trait Paintable {
    fn buffer(&self, P: &Painter) -> VertexBuffer;

    fn finish(self, p: &Painter) -> VertexBuffer
    where
        Self: std::marker::Sized,
    {
        self.buffer(p)
    }
}