//! Shader graph for Bevy engine.

pub mod material;
pub use material::*;
#[cfg(feature = "egui")]
pub mod editor;
#[cfg(feature = "egui")]
pub use editor::*;
