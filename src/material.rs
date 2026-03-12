use crate::extended_material::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::{
    mesh::MeshVertexBufferLayoutRef, prelude::*, reflect::Reflect, render::render_resource::*,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderGraph {
    pub vertex: Option<Handle<Shader>>,
    pub fragment: Option<Handle<Shader>>,
}

pub type StandardShaderGraphMaterial = ExtendedMaterial<StandardMaterial, ShaderGraphMaterial>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[bind_group_data(ShaderGraph)]
#[reflect(Default, Debug)]
pub struct ShaderGraphMaterial {
    #[uniform(100)]
    pub prop4: Vec4,
    #[reflect(ignore)]
    pub graph: ShaderGraph,
}

impl Default for ShaderGraphMaterial {
    fn default() -> Self {
        Self {
            prop4: Default::default(),
            graph: Default::default(),
        }
    }
}

impl From<&ShaderGraphMaterial> for ShaderGraph {
    fn from(material: &ShaderGraphMaterial) -> Self {
        material.graph.clone()
    }
}

impl MaterialExtension for ShaderGraphMaterial {
    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        //eprintln!("-- mesh.layout={layout:#?}");
        //eprintln!("-- pipeline.layout={:#?}", descriptor.layout);
        if let Some(new_vertex) = key.bind_group_data.vertex {
            descriptor.vertex.shader = new_vertex;
        }

        if let Some(new_fragment) = key.bind_group_data.fragment {
            if let Some(fragment) = descriptor.fragment.as_mut() {
                fragment.shader = new_fragment;
            }
        }

        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct ShaderGraphMaterialPlugin;

impl Plugin for ShaderGraphMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StandardShaderGraphMaterial>::default())
            .register_asset_reflect::<StandardShaderGraphMaterial>()
            .register_asset_reflect::<ShaderGraphMaterial>();
    }
}
