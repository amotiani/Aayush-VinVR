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
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

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

    let player_pos = player_g_transform.translation();
    let forward = player_g_transform.forward();
    let spawn_pos = player_pos + (forward * 2.0);

    let loaded_asset = asset_server.load("models/scene.gltf#Scene0");

    commands.spawn((
        Name::new("Spawned Boat"),
        SceneRoot(loaded_asset), 
        Transform{
            translation: spawn_pos,
            // You might need to scale the object if the GLTF is too big/small
            scale: Vec3::splat(1.0), 
            ..default()
        }
    ));

}