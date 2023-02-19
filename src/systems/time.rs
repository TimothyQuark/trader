use bevy::prelude::*;

use crate::{
    components::{common::WaitTime, ships::Player},
    AppState,
};

/// Resource that holds the game time. Time resource is used by Bevy
#[derive(Resource)]
pub struct GameTime {
    pub tick: u64,
}

/// System that ticks down WaitingTime for all entities. If Player has WaitingTime = 0, then change
/// game state to AwaitingInput
pub fn transition_time(
    mut query: Query<(&mut WaitTime, Option<&Player>)>,
    mut state: ResMut<State<AppState>>,
    mut time: ResMut<GameTime>,
) {
    for mut t in query.iter_mut() {
        /*
        Note to self: on turn 0, we are in state AwaitingInput. The player then takes an action,
        and leaves player_input system. Then we increment time by 1. Thus, other entities' first action
        is on turn 1, not turn 0. This is sort of good, because it means the player will always have the first turn
        of the game, nobody else will do ANYTHING on this turn. But may want to change in the future
         */

        // Decrement remaining wait time for each entity
        if t.0.turns > 0 {
            t.0.turns -= 1;
        }

        //
        if let Some(_) = t.1 {
            if t.0.turns == 0 {
                // AwaitingInput will only become the state in next update step of game?
                state.overwrite_replace(AppState::AwaitingInput).unwrap();
                // println!("Player AwaitingInput on turn {}", time.tick);
            }
        }
    }

    // Increment game time
    time.tick += 1;
}
