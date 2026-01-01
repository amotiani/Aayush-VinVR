// Feel free to spawn some objects in here yourself.
// This is just an empty plugin for now.

use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::player::FpsPlayer;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_object_on_key_e);
    }
}

pub fn spawn_object_on_key_e(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_head: Query<&GlobalTransform, With<FpsPlayer>>,
) {
    let mut player_g_transform = match player_head.single() {
        Ok(transform) => transform,
        Err(QuerySingleError::NoEntities(_)) => {
            error!("No FpsPlayer found (0 players)");
            return;
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            error!("Multiple FpsPlayer entities found (>1 players)");
            return;
        }
    };

    // FILL IN HERE WITH YOUR OWN GLTF SPAWNING CODE


    // When you spawn SceneRoot component
    // Add a transform to the spawn function
    // and adjust that transform based on playe position

    // load gltf example:
    // https://bevy.org/examples/3d-rendering/load-gltf/

}