use bevy::{
    asset::{load_internal_asset, uuid_handle},
    //mesh::MeshVertexBufferLayoutRef,
    pbr::*,
    prelude::*,
    reflect::Reflect,
    render::render_resource::*,
    shader::*,
};

pub const GRAPH_VERTEX_HANDLE: Handle<Shader> =
    uuid_handle!("421ac834-1110-43f5-ac69-d64b37b19496");
pub const GRAPH_FRAGMENT_HANDLE: Handle<Shader> =
    uuid_handle!("e241aa47-6bec-4043-a945-1d70564554b4");

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderGraph {
    //pub vertex: Option<Handle<Shader>>,
    //pub fragment: Option<Handle<Shader>>,
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
    fn fragment_shader() -> ShaderRef {
        //"shaders/shader_graph.wgsl".into()
        GRAPH_FRAGMENT_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        //"shaders/shader_graph.wgsl".into()
        GRAPH_FRAGMENT_HANDLE.into()
    }

    /*
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
    // */
}

#[derive(Default, Clone, Debug)]
pub struct ShaderGraphMaterialPlugin;

impl Plugin for ShaderGraphMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GRAPH_FRAGMENT_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/shader_graph.wgsl"
            ),
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            GRAPH_VERTEX_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/shader_graph.wgsl"
            ),
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<StandardShaderGraphMaterial>::default())
            .register_asset_reflect::<StandardShaderGraphMaterial>()
            .register_asset_reflect::<ShaderGraphMaterial>();
    }
}
