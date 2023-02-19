use bevy::prelude::*;

use crate::components::{
    combat::WantsToMelee,
    common::WaitTime,
    map::Position,
    ships::{CombatStats, Player, ShipStats},
};
use crate::systems::map::Map;
use crate::AppState;

use super::time::GameTime;

#[derive(Debug, PartialEq)]
enum PlayerAction {
    NoAction, // ex: player bumps into wall
    WaitTurn, // Player pressed button to wait in place for single game turn
    Moved,
    MeleeAttack,
}

/// System which checks if there was any keyboard/mouse input
pub fn player_input(
    mut last_time: Local<f64>, // Local variables are kept between System calls, Bevy is cool!
    mut commands: Commands,
    time: Res<Time>, // Bevy time
    game_time: Res<GameTime>, // Game time in turns
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut state: ResMut<State<AppState>>,

    mut set: ParamSet<(
        Query<
            (
                Entity,
                &mut Position,
                &mut WaitTime,
                &ShipStats,
                &CombatStats,
            ),
            With<Player>,
        >,
        Query<(&mut Position, &mut WaitTime, &ShipStats), Without<Player>>,
    )>,
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
    // println!("Passed time: {passed_time}"); // Game will lag with print statement
    if passed_time < delay {
        // If time interval too short, exit function prematurely
        return;
    }

    // Update old time, to use for next user input
    // println!("Time passed: {}", passed_time);
    *last_time = time.elapsed_seconds_f64();

    // Player WaitingTime is not 0, so skip turn
    if set.p0().single_mut().2.as_mut().turns > 0 {
        // Player action succesful, end player turn.
        state.clear_schedule();
        state.overwrite_replace(AppState::TransitionTime).unwrap();
        return;
    }

    // println!("Awaiting player input (Turn {})", game_time.tick);

    // Check diagonal movements before normal movements. Alternatively, check if
    // Shift or control are NOT pressed.
    let mut moved = PlayerAction::NoAction;
    if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LShift) {
        // println!("Right Shift pressed");
        moved = try_move_player(1, 1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LControl) {
        // println!("Right Control pressed");
        moved = try_move_player(1, -1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LShift) {
        // println!("Right key pressed");
        moved = try_move_player(-1, 1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LControl) {
        // println!("Right key pressed");
        moved = try_move_player(-1, -1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Down) {
        // println!("Down key pressed");
        moved = try_move_player(0, -1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Up) {
        // println!("Up key pressed");
        moved = try_move_player(0, 1, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Left) {
        // println!("Left key pressed");
        moved = try_move_player(-1, 0, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Right) {
        // println!("Right key pressed");
        moved = try_move_player(1, 0, &map, commands, &mut set, &mut state);
    } else if keys.just_pressed(KeyCode::Period) {
        // println!("Pressed full stop");
        moved = PlayerAction::WaitTurn;
    }

    match moved {
        PlayerAction::NoAction => {}
        PlayerAction::Moved => {
            set.p0().single_mut().2.as_mut().turns += set.p0().single_mut().3.speed;
        }
        PlayerAction::MeleeAttack => {
            set.p0().single_mut().2.as_mut().turns += set.p0().single_mut().4.melee_speed;
        }
        PlayerAction::WaitTurn => {
            set.p0().single_mut().2.as_mut().turns += 1;
        }
    }

    // Used for debugging
    if moved != PlayerAction::NoAction {
        println!("Player took action {:?} on turn {}", moved, game_time.tick);
    }



    // println!("No player input detected");
}

/// Function which tries to move the player, checks for collisions and out of bounds.
/// Returns true if player was able to move
fn try_move_player(
    delta_x: i32,
    delta_y: i32,
    map: &Map,
    mut commands: Commands,
    p_set: &mut ParamSet<(
        Query<
            (
                Entity,
                &mut Position,
                &mut WaitTime,
                &ShipStats,
                &CombatStats,
            ),
            With<Player>,
        >,
        Query<(&mut Position, &mut WaitTime, &ShipStats), Without<Player>>,
    )>,
    state: &mut State<AppState>,
) -> PlayerAction {
    // println!("Trying to move player");

    // Check if player tries to go out of bounds
    if p_set.p0().single().1.x + delta_x < 0
        || p_set.p0().single().1.x + delta_x > map.width as i32 - 1
        || p_set.p0().single().1.y + delta_y < 0
        || p_set.p0().single().1.y + delta_y > map.height as i32 - 1
    {
        return PlayerAction::NoAction;
    }

    let destination_idx = map.xy_idx(
        p_set.p0().single_mut().1.as_ref().x + delta_x,
        p_set.p0().single_mut().1.as_ref().y + delta_y,
    );

    // Check if player is trying to attack a neighboring tile
    for potential_target in map.tile_content[destination_idx].iter() {
        if let Ok((pos, wait_time, ship_stats)) = p_set.p1().get_mut(*potential_target) {
            println!(
                "Player attack has found a target. Entity {} has stats {:?}",
                potential_target.index(), ship_stats
            );
            if let Ok((player_entity, _, _, _, _)) = p_set.p0().get_single() {
                commands.entity(player_entity).insert(WantsToMelee {
                    target: *potential_target,
                });
            } else {
                panic!("Player tried to target an enemy but it failed!");
            }
            // Attack is considered to be a
            return PlayerAction::MeleeAttack;
        } else {
            panic!("Trying to target an entity that is not in the destination tile")
        }
        // TODO: Add waiting time
    }

    // Check if destination is blocked
    if !map.blocked_tiles[destination_idx] {
        // TODO: Make this += line
        p_set.p0().single_mut().1.x += delta_x;
        p_set.p0().single_mut().1.y += delta_y;

        return PlayerAction::Moved;
    } else {
        println!("Player bumped into a blocked tile");
        return PlayerAction::NoAction;
    }
}
