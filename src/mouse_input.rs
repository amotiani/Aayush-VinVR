use std::f32::consts::FRAC_PI_2;

use bevy::{ecs::query::QuerySingleError, input::mouse::AccumulatedMouseMotion, prelude::*, window::{CursorGrabMode, CursorOptions, PrimaryWindow}};

use crate::player::FpsPlayer;

pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseLookSettings::default());
        app.add_systems(Update, mouse_focus_toggle);
        app.add_systems(Update, mouse_player_look);
    }
}

// A Resource is a single global variable, unlike having 100s of entities with components
#[derive(Resource, Debug, Clone, Reflect)]
pub struct MouseLookSettings {
    pub sensitivity: Vec2,
    pub focus: MouseTabFocus,
}

impl Default for MouseLookSettings {
    fn default() -> Self {
        Self {
            sensitivity: Vec2::new(0.003, 0.002),
            focus: MouseTabFocus::None,
        }
    }
}

#[derive(Reflect, Default, Clone, Debug)]
pub enum MouseTabFocus {
    InGame,
    #[default]
    None,
}

fn mouse_player_look(
    mut accum_mouse: ResMut<AccumulatedMouseMotion>,
    settings: Res<MouseLookSettings>,
    mut fps_players: Query<&mut Transform, With<FpsPlayer>>,
    mut main_cam: Query<&mut Transform, Without<FpsPlayer>>,
) {
    let delta = accum_mouse.delta;
    if delta == Vec2::ZERO {
        return;
    }

    let delta_yaw   = -delta.x * settings.sensitivity.x;
    let delta_pitch = -delta.y * settings.sensitivity.y;
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

    match settings.focus {
        MouseTabFocus::InGame => {
            for mut player in &mut fps_players {
                apply_look(&mut player, delta_yaw, delta_pitch, PITCH_LIMIT);
            }
        }
        MouseTabFocus::None => {}
    }
}

fn mouse_focus_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<MouseLookSettings>,
    mut window: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !keys.just_pressed(KeyCode::KeyQ) {
        return;
    }

    let Ok(mut cursor) = window.single_mut() else { return };

    settings.focus = match settings.focus {
        // If we are not focused,
        // Then switch to in-game focus
        MouseTabFocus::None => {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;

            MouseTabFocus::InGame
        }

        // If we are focused in-game,
        // Then switch to no focus
        MouseTabFocus::InGame => {
            cursor.grab_mode = CursorGrabMode::None;
            cursor.visible = true;

            MouseTabFocus::None
        }
    };
}

fn apply_look(
    transform: &mut Transform,
    delta_yaw: f32,
    delta_pitch: f32,
    pitch_limit: f32,
) {
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    let yaw   = yaw + delta_yaw;
    let pitch = (pitch + delta_pitch).clamp(-pitch_limit, pitch_limit);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}