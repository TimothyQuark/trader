use bevy::{prelude::*, render::camera::RenderTarget};

use crate::components::rendering::MainCamera;

// // General functions that are used throughout the project

// pub fn convert_cursor_to_world_coords  (
//     wnds: Res<Windows>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
// ) -> Vec2 {
//     // get the camera info and transform
//     // assuming there is exactly one main camera entity, so query::single() is OK
//     let (camera, camera_transform) = q_camera.single();

//     // get the window that the camera is displaying to (or the primary window)
//     let wnd = if let RenderTarget::Window(id) = camera.target {
//         wnds.get(id).unwrap()
//     } else {
//         wnds.get_primary().unwrap()
//     };

//     // check if the cursor is inside the window and get its position
//     if let Some(screen_pos) = wnd.cursor_position() {
//         // get the size of the window
//         let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

//         // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
//         let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

//         // matrix for undoing the projection and camera transform
//         let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

//         // use it to convert ndc to world-space coordinates
//         let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

//         // reduce it to a 2D value
//         let world_pos: Vec2 = world_pos.truncate();

//         world_pos

//         // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
//     }
// }

pub fn convert_cursor_to_world_coords(
    wnds: &Res<Windows>,
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

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
