use bevy::prelude::*;

use crate::components::{
    combat::WantsToMelee,
    map::Position,
    ships::{Player, Ship, ShipStats},
    timers::WaitTimer,
};
use crate::systems::{
    map::{Map, MapTileType},
    terminal::GameLog,
};
use crate::AppState;

use super::time::GameTime;

#[derive(Debug, PartialEq)]
enum PlayerAction {
    NoAction, // ex: player bumps into wall
    WaitTurn, // Player pressed button to wait in place for single game turn
    Moved,
    MeleeAttack,
    OpenInventoryMenu, // Player enters the inventory menu
    EnterWormhole,     // Player tries to enter wormhole and go to next level
}

/// System which checks if there was any keyboard/mouse input
pub fn player_input(
    mut last_time: Local<f64>, // Local variables are kept between System calls, Bevy is cool!
    commands: Commands,
    time: Res<Time>,          // Bevy time
    game_time: Res<GameTime>, // Game time in turns
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut state: ResMut<State<AppState>>,
    mut log: ResMut<GameLog>,
    // mut key_evr: EventReader<KeyboardInput>, // Used for debugging // input::{keyboard::KeyboardInput, ButtonState},
    mut set: ParamSet<(
        //p0: player, p1: other ships
        Query<(Entity, &mut Position, &mut WaitTimer, &ShipStats), (With<Player>, With<Ship>)>,
        Query<&mut Position, (Without<Player>, With<Ship>)>,
    )>,
) {
    // See https://bevy-cheatbook.github.io/input/keyboard.html

    // TODO: Match statement using current game state, and then use either
    // single key events, or text input for entire sentences (see https://bevy-cheatbook.github.io/input/char.html)
    // TODO: Add fast travel (space + arrow key?). Maybe not directly inside input mod.

    /* (Following is not entirely relevant anymore)
    Once player input is detected and accepted, we switch to next state.
    MonsterTurn is usually very fast and returns to AwaitingInput, which means
    player_input system seems to be triggered multiple times in a single frame update.
    That results in duplicate player inputs being detected (ex: Player moves 5 spaces when arrows
    key only pressed once.). Hence, player_input has a built in delay, which returns
    nothing if the time since last player_input run was too recent.

    Tried to solve this by using system run criteria,
    .add_system_set(SystemSet::on_update(AppState::AwaitingInput).with_run_criteria(FixedTimestep::step(0.5)).with_system(player_input))
    , but Bevy does not seem to like triggering a system on both a state change and fixed time interval.
    */

    // println!("{:?}", state);

    // As state transitions are so fast, it can cause to sort of hang when in this system. Hence a small time delay is added
    // Delay only needs to be very small, 1ms.
    let delay: f64 = 0.001;
    let passed_time: f64 = time.elapsed_seconds_f64() - *last_time;
    // println!("Passed time: {passed_time}"); // Game will lag with print statement
    if passed_time < delay {
        // If time interval too short, exit function prematurely
        return;
    }

    // Update old time, to use for next user input
    // println!("Time passed: {}", passed_time);
    *last_time = time.elapsed_seconds_f64();

    // No longer needed as IncrementTime skips player systems if WaitTime is not 0.
    // Instead, panic if this is not true
    if set.p0().single_mut().2.as_mut().turns > 0 {
        panic!("Player's WaitTime is not 0, but we are taking a player turn!");
    }
    // // Player WaitingTime is not 0, so transition to IncrementTime
    // if set.p0().single_mut().2.as_mut().turns > 0 {
    //     // state.clear_schedule();
    //     // state.overwrite_replace(AppState::IncrementTime).unwrap();
    //     state.set(AppState::RunAI).unwrap();
    //     return;
    // }

    // Game will lag with this print statement
    // println!("Awaiting player input (Turn {})", game_time.tick);

    // TODO: Add keypad controls as well (number pad)
    // Check diagonal movements before normal movements. Alternatively, check if
    // Shift or control are NOT pressed.
    let mut moved = PlayerAction::NoAction;
    if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LShift) {
        // println!("Right Shift pressed");
        moved = try_move_player(1, 1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LControl) {
        // println!("Right Control pressed");
        moved = try_move_player(1, -1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LShift) {
        // println!("Right key pressed");
        moved = try_move_player(-1, 1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LControl) {
        // println!("Right key pressed");
        moved = try_move_player(-1, -1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Down) {
        // println!("Down key pressed");
        moved = try_move_player(0, -1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Up) {
        // println!("Up key pressed");
        moved = try_move_player(0, 1, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Left) {
        // println!("Left key pressed");
        moved = try_move_player(-1, 0, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Right) {
        // println!("Right key pressed");
        moved = try_move_player(1, 0, &map, commands, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::Period) {
        // println!("Pressed full stop");
        moved = PlayerAction::WaitTurn;
    } else if keys.just_pressed(KeyCode::E) {
        // println!("Trying to enter location!");
        moved = try_enter_location(&map, &mut log, &game_time, &mut set);
    } else if keys.just_pressed(KeyCode::I) {
        println!("Entering the Inventory Menu");
        moved = PlayerAction::OpenInventoryMenu;
    }

    // Used for debugging, to figure out what a key is
    // for ev in key_evr.iter() {
    //     match ev.state {
    //         ButtonState::Pressed => {
    //             println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
    //         }
    //         ButtonState::Released => {
    //             println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
    //         }
    //     }
    // }

    match moved {
        PlayerAction::NoAction => {}
        PlayerAction::Moved => {
            set.p0().single_mut().2.as_mut().turns += set.p0().single_mut().3.speed;
        }
        PlayerAction::MeleeAttack => {
            set.p0().single_mut().2.as_mut().turns += set.p0().single_mut().3.melee_speed;
        }
        PlayerAction::WaitTurn => {
            set.p0().single_mut().2.as_mut().turns += 1;
        }
        PlayerAction::EnterWormhole => {}
        PlayerAction::OpenInventoryMenu => {}
    }

    // Check what action player undertook
    if moved == PlayerAction::EnterWormhole {
        println!("Player entered wormhole on turn {}", game_time.tick);
        state.set(AppState::NextLevel).unwrap();
    } else if moved == PlayerAction::OpenInventoryMenu {
        state.push(AppState::InventoryMenu).unwrap();
    } else if moved != PlayerAction::NoAction {
        // Player took action, skip TransitionTime and instead go to RunAI
        // println!("Player took action {:?} on turn {}", moved, game_time.tick);
        state.set(AppState::RunAI).unwrap();
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
    _game_time: &Res<GameTime>, // Game time in turns
    p_set: &mut ParamSet<(
        Query<(Entity, &mut Position, &mut WaitTimer, &ShipStats), (With<Player>, With<Ship>)>,
        Query<&mut Position, (Without<Player>, With<Ship>)>,
    )>,
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
        if let Ok(_ship_stats) = p_set.p1().get_mut(*potential_target) {
            // println!(
            //     "Player attack has found a target. Entity {} has stats {:?}",
            //     potential_target.index(),
            //     ship_stats
            // );
            if let Ok((player_entity, _, _, _)) = p_set.p0().get_single() {
                commands.entity(player_entity).insert(WantsToMelee {
                    target: *potential_target,
                });
                // println!(
                //     "Player wants to melee entity {} on turn {}",
                //     potential_target.index(),
                //     game_time.tick
                // );
            } else {
                panic!(
                    "Player tried to target entity {} but it failed!",
                    potential_target.index()
                );
            }
            // Attack is considered to be a
            return PlayerAction::MeleeAttack;
        } else {
            // The target is not a ship, there are different interactions for this
            // panic!(
            //     "Trying to target an entity {} that is not in the destination tile",
            //     potential_target.index()
            // );
        }
    }

    // Check if destination is blocked
    if !map.blocked_tiles[destination_idx] {
        p_set.p0().single_mut().1.x += delta_x;
        p_set.p0().single_mut().1.y += delta_y;

        return PlayerAction::Moved;
    } else {
        // println!("Player bumped into a blocked tile");
        return PlayerAction::NoAction;
    }
}

/// Function which checks if player is trying to enter a valid location, returns what kind of location
fn try_enter_location(
    map: &Map,
    log: &mut GameLog,
    game_time: &Res<GameTime>,
    p_set: &mut ParamSet<(
        Query<(Entity, &mut Position, &mut WaitTimer, &ShipStats), (With<Player>, With<Ship>)>,
        Query<&mut Position, (Without<Player>, With<Ship>)>,
    )>,
) -> PlayerAction {
    let destination_idx = map.xy_idx(
        p_set.p0().single_mut().1.as_ref().x,
        p_set.p0().single_mut().1.as_ref().y,
    );

    // Check what kind of location we are trying to enter, return based on action.
    // Add game log entry to confirm something happened
    let destination_tile = map.tiles[destination_idx];
    match destination_tile {
        MapTileType::Wormhole => {
            log.new_log("You enter the wormhole".to_string(), game_time.tick);
            return PlayerAction::EnterWormhole;
        }
        MapTileType::Placeholder
        | MapTileType::Wall
        | MapTileType::Space
        | MapTileType::Planet
        | MapTileType::Star
        | MapTileType::Moon
        | MapTileType::Asteroid => {}
    }

    log.new_log("You cannot enter here".to_string(), game_time.tick);
    return PlayerAction::NoAction;
}
