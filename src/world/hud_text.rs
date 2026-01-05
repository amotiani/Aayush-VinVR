use bevy::prelude::*;

pub struct HudTextPlugin;

impl Plugin for HudTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

// Notice how the hud text is fine in game mode
// But in editor mode the editor ui covers it up.
// When using the editor UI layer items will
// need to be moved to different cameras to work properly.
// See the player cursor section for more
fn spawn_text(mut commands: Commands) {
    let font = TextFont {
        font_size: 25.0,
        ..default()
    };
     commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (Text::new("Move the player with WASD"), font.clone()), 
            (Text::new("Rotate the camera with the mouse"), font)
        ],
    ));
}