//! Shader graph for Bevy engine.

use bevy::{pbr::*, prelude::*};

pub mod material;
pub use material::*;
#[cfg(feature = "egui")]
pub mod editor;
#[cfg(feature = "egui")]
pub use editor::*;

#[derive(Default, Clone, Debug)]
pub struct ShaderGraphMaterialPlugin;

impl Plugin for ShaderGraphMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StandardShaderGraphMaterial>::default())
            .register_asset_reflect::<StandardShaderGraphMaterial>()
            .register_asset_reflect::<ShaderGraphMaterial>();
    }
}
