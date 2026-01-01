/*
    Since this whole file is conditionally enabled by the editor feature,
    we don't need config flags in this file itself.
*/

use bevy::{camera::{CameraOutputMode, visibility::RenderLayers}, prelude::*, render::render_resource::BlendState};
use bevy_egui::{EguiGlobalSettings, PrimaryEguiContext};
use space_editor::{prelude::{EditorCameraMarker, EditorGameViewWorldCameraMarker, EditorRegistryExt, EditorState, simple_editor_setup}, space_editor_ui::{all_render_layers, ext::bevy_panorbit_camera::PanOrbitCamera}};
use transform_gizmo_bevy::GizmoCamera;

use crate::{mouse_input::MouseLookSettings, player::FpsPlayer};

pub struct EditorSetupPlugin;

impl Plugin for EditorSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_editor);

        app.register_type::<FpsPlayer>();
        app.register_type::<MouseLookSettings>();
        app.editor_registry::<FpsPlayer>();
    }
}
fn setup_editor(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
){
    egui_global_settings.auto_create_primary_context = false;

    // By default EditorState is Game. Set it to Editor to show editor ui
    // Spawn the main 3d editor camera
    commands.set_state(EditorState::Editor);

    // camera for the editor world view
    commands.spawn((
      Camera3d::default(),
      Camera {
          order: 100,
          ..default()
      },
      Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      PanOrbitCamera::default(),
      EditorCameraMarker,
      Name::from("Main Editor Camera"),
      EditorGameViewWorldCameraMarker,
      GizmoCamera,
      MeshPickingCamera,
      Msaa::Off,
    ));

    // Egui UI Camera for the editor
    commands.spawn((
      Name::from("Editor Egui Ui Camera"),
      Camera {
          order: 101,
          output_mode: CameraOutputMode::Write {
            blend_state: Some(BlendState::ALPHA_BLENDING),
            clear_color: ClearColorConfig::None,
          },
          clear_color: ClearColorConfig::Custom(Color::NONE),
          ..default()
      },
      // Cannot be a 2d cam or msaa causes issues
      Camera2d::default(),
      PrimaryEguiContext,
      EditorCameraMarker,
      // Set random render layer so egui 3d cam does minimal work
      RenderLayers::from_layers(&[10000]),  
      Msaa::Off,
    ));
}
