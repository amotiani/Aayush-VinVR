use bevy::{camera::visibility::RenderLayers, color::palettes::css::RED, prelude::*, render::Render};

/*
    I've come across a good way to handle cameras and
    characters or other items in bevy.

    So, in space editor, I like to spawn stuff quick with
    one component and have the rest set itself up.

    This is done so that when saving and loading, we can
    save spawn information to a single component and have
    the rest of the setup done automatically.

    So, we create a component called FpsPlayer.
    When we spawn an entity with FpsPlayer, we can have
    a trigger spawn the child camera and child models.

    I keep the models, cameras, and player designation
    in seperate entities so that we can easily
    manipulate them later on if needed.
*/

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, spawn_character_startup);
        app.add_observer(spawn_character_trigger);
   }
}

// We save a link to the camera and feet entities
// so that we can easily access them later if needed.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FpsPlayer {
    camera_entity: Option<Entity>,
    head_entity: Option<Entity>,
   
    // ADD CROSSHAIR ENTITY HERE LATER
    crosshair_entity: Option<Entity>,
}

// Notice how simple spawn character setup is.
// We just spawn an entity with the FpsPlayer component.
// Or when we are in the editor, we can add FpsPlayer
// to an empty entity
// or make a custom editor button that really easily spawns
// the player entity.
fn spawn_character_startup(
    mut commands: Commands,
){
    // If we had our own custom camera we
    // could insert it instead.

    commands.spawn((
        Name::new("Startup FPS Player"),
        FpsPlayer {
            camera_entity: None,
            head_entity: None,
            crosshair_entity: None,
        }
    ));
}

// Here is where the real action happens.
fn spawn_character_trigger(
   trigger: On<Add, FpsPlayer>,
   mut commands: Commands,
   mut players: Query<&mut FpsPlayer>,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>,
   mut material_2d: ResMut<Assets<ColorMaterial>>
) {
   // We get the entity from the startup system
   let player_root = trigger.entity;

   // We must get the FpsPlayer component mutably
   // from a query still
   let Ok(mut player) = players.get_mut(player_root) else {
       error!(
          "FpsPlayer added to entity {:?} but query failed",
          player_root
       );
       return;
   };

   // Spawn the root player entity
   // Note the insert_if_new function.
   // When using insert, all the listed components
   // overwrite any existing components.
   // insert_if_new only adds the components if they
   // don't already exist on the entity.
   // that way if we already have a Transform or a Name
   // saved or designated, the trigger won't overide it.
   let player_root = commands.entity(player_root).insert_if_new((
       Name::new("Player"), // Create a component Name and stores value Player
       Transform {
          translation: Vec3::new(0.0,1.5,0.0),
          rotation: Quat::IDENTITY, //Placeholder for default
          scale: Vec3::splat(3.5),
       },
       Visibility::default(),
   )).id();

   // If we already have a known camera, we won't
   // create a new one. Future improvement could be
   // to check if the existing camera is valid.
   if player.camera_entity.is_none() {

       let player_cam = commands.spawn((
          Name::new("Player Camera"),
          Camera3d::default(),
          // Camera transform will be at origin of FpsPlayer
          Transform::from_xyz(0.0,0.0,0.0).looking_at(Vec3::ZERO, Vec3::Y),

          // For space_editor compatibility
          #[cfg(feature = "space_editor")]
          (
              space_editor::prelude::PlaymodeCamera {},
          ),
      
       )).id();

       commands.entity(player_root).add_child(player_cam);
       player.camera_entity = Some(player_cam);
   }

   // Similarly, we only create the head entity
   // if we don't already have one.
   if player.head_entity.is_none() {

       // The player head sphere is not visible from inside
       // the sphere so this is ok for now
       // In the fututre we will go into RenderLayers
       // especially when we get to portals and multiplayer
       let player_head = commands.spawn((
          Name::new("Player Head"),
          Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
          MeshMaterial3d(materials.add(Color::srgba(0.0, 0.5, 0.0, 1.0))),
          Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
       )).id();

       commands.entity(player_root).add_child(player_head);
       player.head_entity = Some(player_head);
   }

   // SPAWN CROSSHAIR ENTITY HERE LATER
   // You have two options on how to do this.
   // 1: Spawn a child entity as a flat circle mesh dot in front of the camera
   // and attach this to the camera entity.
   // 2. Use bevy ui to create a crosshair in the center of the screen.
   // You may notice the crosshair is not properly placed in the camera center
   // if you use the editor, I can go into how to fix that later if needed.
   // I recommend option 1 for simplicity, I tried option 2 but you need
   // custom logic to see if the camera is used in the editor view or not.
   if player.crosshair_entity.is_none() {
      let crosshair: Entity = commands.spawn((
         Name::new("Crosshair"),
         Mesh2d(meshes.add(Circle::default())),
         MeshMaterial2d(material_2d.add(Color::from(RED))),
         Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
      )).id();

      if let Some(camera_entity) = player.camera_entity {
            commands.entity(camera_entity).add_child(crosshair);
            player.crosshair_entity = Some(crosshair);
      }
   }

   // You'll notice a delay in the crosshair option 1 following the camera
   // When using space editor. Space editor spawns a 4th camera
   // for the playmode camera view, and the crosshair lags
   // behind a frame. Not a big deal for now.
     
}
