use anyhow::Result;

use bevy::{
  prelude::*,
  render::mesh::*,
  input::common_conditions,
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_egui::EguiPlugin;

use bevy_shader_graph::*;

fn main() -> Result<()> {
  let file = std::env::args().nth(1);
  let mut editor = ShaderGraphEditor::new();
  if let Some(file) = file {
    editor.load(file)?;
  }

  let mut app = App::new();

  app.add_plugins((
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Shader graph editor".into(),
            ..default()
        }),
        ..default()
    }).set(
      AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
      }
    ),
    EguiPlugin,
    LogDiagnosticsPlugin::default(),
    FrameTimeDiagnosticsPlugin,
  ));

  app.add_plugins(ShaderGraphMaterialPlugin)
    .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new()
      .run_if(common_conditions::input_toggle_active(false, KeyCode::KeyE))
    )
    .add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin)
    .insert_resource(editor)
    .add_systems(Startup, setup)
    .add_systems(Update, (
      shader_editor,
      handle_quit,
    ));

  app.run();

  Ok(())
}

fn handle_quit(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
  if input.pressed(KeyCode::KeyQ) {
    exit.send(AppExit::Success);
  }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut graph_materials: ResMut<Assets<StandardShaderGraphMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -0.5, 1.0)
          .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    let mat1 = graph_materials.add(StandardShaderGraphMaterial {
      base: StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/test_room_E.png")),
        reflectance: 1.0,
        ..default()
      },
      extension: ShaderGraphMaterial::default(),
    });
    /*
    let mat1 = graph_materials.add(StandardShaderGraphMaterial {
      base: StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/1024x1024_test.png")),
        ..default()
      },
      extension: ShaderGraphMaterial::default(),
    });
    let cube = Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(1.0)).with_generated_tangents().unwrap()));
    // back cube
    commands.spawn((
        cube.clone(),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(-1.0, 0.0, -1.0),
    Name::new("Back cube")));
    // front cube
    commands.spawn((
        cube.clone(),
        MeshMaterial3d(mat1.clone()),
        Transform::from_xyz(1.0, 0.0, 0.8),
    Name::new("Front cube")));
    */

    let mesh = Mesh3d(meshes.add(PlaneMeshBuilder::from_length(1.0).subdivisions(0).build()
      .with_generated_tangents().unwrap()));
    // wall
    let mut wall = commands.spawn((
        mesh.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0)
          .with_rotation(Quat::from_rotation_x(1.570796)),
        MeshMaterial3d(mat1.clone()),
    ));
    wall.insert(Name::new("Wall"));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    let mut cam = commands.spawn((
        Camera3d::default(),
      Transform::from_xyz(5.0, 0.0, 0.0)
        .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    DistanceFog {
        color: Color::srgba(0.25, 0.25, 0.25, 1.0),
        falloff: FogFalloff::Linear {
            start: 5.0,
            end: 20.0,
        },
        ..default()
    }));

    cam.insert(bevy_panorbit_camera::PanOrbitCamera {
        focus: Vec3::new(0.0, 0.0, 0.0),
        radius: Some(5.0),
        yaw: Some(0.00),
        pitch: Some(0.0),
        ..default()
      },
    );
    cam.insert(Name::new("Camera"));
}
