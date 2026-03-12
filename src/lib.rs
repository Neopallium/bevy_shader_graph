//! Shader graph for Bevy engine.

extern crate alloc;

pub mod extended_material;
pub mod material;
pub use material::*;
#[cfg(feature = "egui")]
pub mod editor;
#[cfg(feature = "egui")]
pub use editor::*;
