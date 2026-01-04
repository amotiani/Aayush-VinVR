# Analysis of Code Structure

## General structure:
- ```main.rs``` : Declares all plugins(custom and default) and adds them to our app, also optionally adds some plugins for space editor feature if enabled.
- ```player.rs``` : Spawns an entity with FPS Player on startup. Also includes an observer which, whenever an FPS Player is added to an entity (including startup), it checks if the entity FPS Player was added to (the root entity) has name, transform, visibility and adds them if it doesn't. Then it checks if the FPS Player has camera & head entities and adds them as child if it doesn't. If player crosshair doesn't exist, it adds a flat 2d mesh of a circle in front of the camera (crosshair).
- ```keyboard_input.rs``` and ```mouse_input.rs``` : Handle the logic for keyboard input for movement(WASD) and mouse input for mouse look.
- floor, hud_text, light, objects : Handle spawning things at startup.
- ```editor_setup.rs``` : Sets up the editor at startup, 2d and 3d cameras for 3d orbit controls and 2d UI setup.

## Small tips:
In ```keyboard_input.rs```, rename ```movement``` to ```movement_direction```. In the part where you scale the movt. vector with speed, store it in ```scaled_movement``` or something similar.
Store magic numbers like speed in constants.

## Bugs / Shortcomings:
In ```keyboard_input.rs```:
- Check the mouse settings focus because WASD will still work when mouse focus is out of game.
- The movement vector's scaling with speed, and adding delta translation to player transform, both are still framerate dependent, make them independent by multiplying them by delta time.
- Also, for the WASD movement, we apply forward & right movement not just in the x-z planes, but also the y plane. This makes player move up when W is pressed & player is looking down. Thus, we should flatten the WASD movement to x-z planes by
making the y value of forward & right vectors 0. 

In code structure:
- If this project scales, keeping scalability and modularity in mind, a good idea would be to add state management for player, game state, and UI. This would help remove checks of mouse focus settings for ```mouse_input.rs``` and the input files will be
- able to use the game state to keep track of cursor and movement enabling/disabling.

NOTE: I preferred the Crosshair Spawning option 1 because in the editor it helps for easy visual debugging and also in game view, it is basically the same as option 2. With option 2, in editor it looks weird since it is centered to the screen.
