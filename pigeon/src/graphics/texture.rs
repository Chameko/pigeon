use parrot::{
    Sampler,
    transform::ScreenSpace,
};
use euclid::Size2D;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
/// ID generator to generate unique IDs
fn get_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// A texture containing its own [`Sampler`] and [`BindingGroup`]
#[derive(Debug)]
pub struct Texture {
    pub id: usize,
    pub sampler: Rc<Sampler>,
    pub texture: parrot::Texture,
    pub name: String,
}

impl Texture {
    /// Returns the size of texture in pixels
    pub fn size(&self) -> Size2D<u32, ScreenSpace> {
        self.texture.size
    }

    pub fn new(texture: parrot::Texture, sampler: Rc<Sampler>, name: &str) -> Self {
        Self {
            id: get_id(),
            sampler,
            texture,
            name: name.to_string()
        }
    }
}