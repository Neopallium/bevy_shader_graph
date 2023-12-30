use bevy::{
  pbr::*,
  prelude::*,
  reflect::{std_traits::ReflectDefault, Reflect, TypeUuid},
  render::{
    render_resource::*,
    mesh::*,
  },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Reflect)]
#[reflect(Default, Debug)]
pub struct ShaderGraph {
  pub vertex: Option<Handle<Shader>>,
  pub fragment: Option<Handle<Shader>>,
}

pub type StandardShaderGraphMaterial = ExtendedMaterial<StandardMaterial, ShaderGraphMaterial>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[uuid = "121439ac-81a5-11ee-8d06-d3da473fad43"]
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
  fn fragment_shader() -> ShaderRef {
    "shaders/shader_graph.wgsl".into()
  }

  fn deferred_fragment_shader() -> ShaderRef {
    "shaders/shader_graph.wgsl".into()
  }

  fn specialize(
      _pipeline: &MaterialExtensionPipeline,
      descriptor: &mut RenderPipelineDescriptor,
      _layout: &MeshVertexBufferLayout,
      key: MaterialExtensionKey<Self>
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
