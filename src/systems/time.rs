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
        if t.0.turns > 0 {
            t.0.turns -= 1;
        }

        if let Some(_) = t.1 {
            if t.0.turns == 0 {
                state.overwrite_replace(AppState::AwaitingInput).unwrap();
            }
        }
    }

    // Increment game time
    time.tick += 1;
}
