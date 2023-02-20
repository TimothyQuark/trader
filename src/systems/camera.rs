use bevy::prelude::*;

use crate::components::rendering::MainCamera;

/// Inits camera used for 2D rendering. Mostly redundant because
/// Bevy has greatly simplified the code needed.
pub fn init_camera(mut commands: Commands) {
    // println!("Initialize camera bundles");

    // Spawn camera and UI Camera bundles
    // MainCamera component added to make finding this entity easier
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
