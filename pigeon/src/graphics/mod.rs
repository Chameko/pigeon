/// Various drawable primatives
pub mod primative;
/// The texture type used by pigeon
pub mod texture;
/// A basic textured rectangle
pub mod sprite;

// Re-export colors
pub use parrot::color::{Bgra8, Rgba8, Rgba};
pub use texture::Texture;
pub use sprite::Sprite;
pub use primative::*;

use crate::pipeline::Render;
pub use crate::pipeline::{Breakdown};

/// Various primatives that can be drawn using the in built pipelines. Also contains [Drawable] to allow users to create their own renderable objects and [Texture].

/// Allows for a graphic to be broken down into a more simplistic form for use in ther renderer
pub trait Drawable {
    type Pipeline: Render;

    fn breakdown(&self) -> crate::pipeline::Breakdown<<<Self as Drawable>::Pipeline as Render>::Vertex>;
}