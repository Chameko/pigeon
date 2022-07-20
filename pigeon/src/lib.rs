//! # Pigeon
//!
//! A simple, flexable, cross-platform 2D rendering library... thing.
//!
//! ## Why
//!
//! Pigeon's development is primarily driven by the need for a graphical backend for AVN. However I also wanted to keep it seperate from AVN so it could be used for other projects.
//!
//! ## Design
//!
//! I wanted it to be simple, small, portable and flexable. Pigeon isn't designed to manage your application and create windows, it just draws shapes to a screen real good (or as good as I can make it).
//!
//! You can see some examples in the example folder
//! 
//! ## Getting started
//! I recommend reading the examples in the examples/ folder to get an idea of how pigeon operates. Also check out the [`crate::pigeon!`] macro

/// Contains pigeon's pipelines
pub mod pipeline;
/// Contains basic graphics and shapes
pub mod graphics;
/// Contains code to manage pigeon
pub mod pigeon;

pub use pigeon::Pigeon;
pub use parrot::transform;
extern crate pigeon_parrot as parrot;
