use crate::AppState;
use bevy::prelude::*;

use crate::components::rendering::MainCamera;

// General functions that are used throughout the project

// TODO: Really this should be inside camera.rs
pub fn convert_cursor_to_world_coords(
    wnd: Single<&mut Window>,
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    let (camera, camera_transform) = q_camera.single().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        // Note: screen Y is 0 at top and increases downward, but NDC Y is -1 at bottom and +1 at top
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc = Vec2::new(ndc.x, -ndc.y); // Flip Y axis
        // println!("x {}, y {}", ndc.x, ndc.y);

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.to_matrix() * camera.clip_from_view().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        return Some(world_pos);
    } else {
        // Cursor is not inside the window
        None
    }
}

/// Utility function that runs after Startup to start the game
pub fn new_game(mut next_state: ResMut<NextState<AppState>>) {
    println!("Starting a new game!");
    next_state.set(AppState::NextLevel);
}
