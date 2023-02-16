use bevy::prelude::*;

/// Inits camera used for 2D rendering. Mostly redundant because
/// Bevy has greatly simplified the code needed.
pub fn init_camera(mut commands: Commands) {
    // println!("Initialize camera bundles");

    // Spawn camera and UI Camera bundles
    commands.spawn(Camera2dBundle::default());
}
