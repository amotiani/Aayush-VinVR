use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::player::FpsPlayer;

pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Update, update_keyboard_movement);
   }
}

fn update_keyboard_movement(
   keyboard_input: Res<ButtonInput<KeyCode>>,
   mut player_query: Query<&mut Transform, With<FpsPlayer>>,

) {
   let mut player_transform = match player_query.single_mut() {
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
   
   let mut movement = Vec3::ZERO;

   if keyboard_input.pressed(KeyCode::KeyW) {
      movement.z += 1.0;
   }

   if keyboard_input.pressed(KeyCode::KeyS) {
      movement.z -= 1.0;
   }

   if keyboard_input.pressed(KeyCode::KeyA) {
      movement.x -= 1.0;
   }

   if keyboard_input.pressed(KeyCode::KeyD) {
      movement.x += 1.0;
   }

   if keyboard_input.pressed(KeyCode::Space) {
      movement.y += 1.0;
   }

   if keyboard_input.pressed(KeyCode::ShiftLeft) {
      movement.y -= 1.0;
   }
   
   let mut speed = 0.22;
   
   // speed control
   if keyboard_input.pressed(KeyCode::ControlLeft) {
      speed = 1.0;
   } else {
      speed = 0.2;
   }

   // Can't normalize a zero vector
   if movement != Vec3::ZERO {
      movement = movement.normalize() * speed;
      let forward = player_transform.forward();
      let right = player_transform.right();
      let up = player_transform.up();

      let delta_translation = (forward * movement.z) + (right * movement.x) + (up * movement.y);
      player_transform.translation += delta_translation;
   }

}

//===== Commands =====
/*
Res<ButtonInput<KeyCode>>
- Tells bevy "Give me read access to the current keyboard state this frame
Res<T>
- Read only access to a resource
ButtonInput<KeyCode>
- Bevy resource that stores current state of keyboard

*/