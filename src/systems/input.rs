use bevy::prelude::*;

use crate::components::{living::Player, map::Position};
use crate::systems::map::Map;
use crate::AppState;

/// System which checks if there was any keyboard/mouse input
pub fn player_input(
    mut last_time: Local<f64>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut state: ResMut<State<AppState>>,
    mut query: Query<&mut Position, With<Player>>,
) {
    // See https://bevy-cheatbook.github.io/input/keyboard.html

    // TODO: Match statement using current game state, and then use either
    // single key events, or text input for entire sentences (see https://bevy-cheatbook.github.io/input/char.html)
    // TODO: Add fast travel (space + arrow key?). Maybe not directly inside input mod.

    /*
    Once player input is detected and accepted, we switch to MonsterTurn.
    MonsterTurn is usually very fast and returns to AwaitingInput, which means
    player_input system seems to be triggered multiple times in a single frame update.
    That results in duplicate player inputs being detected (ex: Player moves 5 spaces when arrows
    key only pressed once.). Hence, player_input has a built in delay, which returns
    nothing if the time since last player_input run was too recent.

    Tried to solve this by using system run criteria,
    .add_system_set(SystemSet::on_update(AppState::AwaitingInput).with_run_criteria(FixedTimestep::step(0.5)).with_system(player_input))
    , but Bevy does not seem to like triggering a system on both a state change and fixed time interval.
    */
    // Delay only needs to be very small, 10ms.
    let delay: f64 = 0.00;
    let passed_time: f64 = time.elapsed_seconds_f64() - *last_time;
    if passed_time < delay {
        // If time interval too short, exit function prematurely
        return;
    }
    // println!("Time passed: {}", passed_time);
    *last_time = time.elapsed_seconds_f64();

    // println!("Awaiting player input");

    // Check diagonal movements before normal movements. Alternatively, check if
    // Shift or control are NOT pressed.
    if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LShift) {
        // println!("Right Shift pressed");
        try_move_player(1, 1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LControl) {
        // println!("Right Control pressed");
        try_move_player(1, -1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LShift) {
        // println!("Right key pressed");
        try_move_player(-1, 1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LControl) {
        // println!("Right key pressed");
        try_move_player(-1, -1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Down) {
        // println!("Down key pressed");
        try_move_player(0, -1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Up) {
        // println!("Up key pressed");
        try_move_player(0, 1, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Left) {
        // println!("Left key pressed");
        try_move_player(-1, 0, &map, query.single_mut().as_mut(), &mut state);
    } else if keys.just_pressed(KeyCode::Right) {
        // println!("Right key pressed");
        try_move_player(1, 0, &map, query.single_mut().as_mut(), &mut state);
    }

    // println!("No player input detected");
}

/// Function which tries to move the player
fn try_move_player(
    delta_x: i32,
    delta_y: i32,
    map: &Map,
    player_pos: &mut Position,
    state: &mut State<AppState>,
) {
    // println!("Trying to move player");

    if player_pos.x + delta_x < 0
        || player_pos.x + delta_x > map.width as i32 - 1
        || player_pos.y + delta_y < 0
        || player_pos.y + delta_y > map.height as i32 - 1
    {
        return;
    }

    let destination_idx = map.xy_idx(player_pos.x + delta_x, player_pos.y + delta_y);

    // Check if destination is blocked
    if !map.blocked_tiles[destination_idx] {
        player_pos.x = player_pos.x + delta_x;
        player_pos.y = player_pos.y + delta_y;

        // Player movement succesful, end player turn. TODO: Right now commented out as there is no transition to AI state
        // state.overwrite_replace(AppState::AwaitingInput).unwrap();
    }
}
