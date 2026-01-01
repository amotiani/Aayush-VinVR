/*
Note:
main.rs is like our header file, where we declear our functions
though plugins.

| Rust                               |
| ---------------------------------- | ----------------------------------------------------------- |
| `main.rs`                          | Central place that declares what parts of the program exist |
| `mod player;`                      | Declares that a module exists                               |
| `use crate::player::PlayerPlugin;` | Imports symbols so they can be used                         |
| `crate::player::PlayerPlugin`      | Fully qualified name                                        |
| `use crate::player::*;`            | Brings all names into scope                                 |
| `player.rs`                        | Actual code implementation                                  |
| Multiple `mod` lines               | Project structure definition                                |

*/

use bevy::prelude::*;

// Declare the modules
mod player;
mod world;
mod keyboard_input;
mod mouse_input;

// Import world items
use world::floor::FloorPlugin;
use world::light::LightPlugin;
use world::hud_text::HudTextPlugin;
use world::objects::ObjectsPlugin;

// Import logic plugins
use keyboard_input::KeyboardInputPlugin;
use player::PlayerPlugin;
use mouse_input::MouseInputPlugin;

// Only include the editor setup if the feature is enabled
#[cfg(feature = "space_editor")]
mod editor_setup;

fn main()
{
    // Create a new Bevy app
    let mut app = App::new();

    // Add the default Bevy plugins and our custom plugins
    app.add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            FloorPlugin,
            LightPlugin,
            KeyboardInputPlugin,
            HudTextPlugin,
            MouseInputPlugin,
            ObjectsPlugin,
    ));

    // Conditionally add the editor setup plugin    
    #[cfg(feature = "space_editor")]
    {
        use editor_setup::EditorSetupPlugin;
        use space_editor::SpaceEditorPlugin;
        // We must add space editor before our editor setup
        // So we can edit its global settings later
        app.add_plugins(SpaceEditorPlugin);
        app.add_plugins(EditorSetupPlugin);
    }

    app.run();
}