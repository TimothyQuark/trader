use bevy::prelude::*;

pub fn init_camera(mut commands: Commands) {
    // println!("Initialize camera bundles");

    // Spawn camera and UI Camera bundles
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn(Camera2dBundle::default());
}
