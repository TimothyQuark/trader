use bevy::prelude::*;

use crate::{
    components::{
        ships::{Player, ShipStats},
        timers::HealthTimer,
    },
    AppState,
};

use super::{terminal::GameLog, time::GameTime};

/// System which queries all Entities with HealthRegen, and regenerates their health if the timer
/// has reached zero. /
/// Once completed, transition to next state
pub fn regen_health(
    mut query: Query<(&mut HealthTimer, Option<&mut ShipStats>, Option<&Player>)>,
    mut state: ResMut<State<AppState>>,
    mut log: ResMut<GameLog>,
    time: Res<GameTime>,
) {
    // println!("Run Health Regen System!");

    for (mut timer, ship_stats, player) in query.iter_mut() {
        // Only regen if it is a ship, else panic. In future, more logic for things like stations, aliens etc.
        if let Some(mut stats) = ship_stats {
            // If timer not 0, increment. Otherwise, increase health and reset timer
            if timer.turns > 0 {
                timer.turns -= 1;
            } else if stats.curr_health < stats.max_health {
                stats.curr_health += 1;
                timer.turns = stats.health_regen;
                if let Some(_) = player {
                    let s = format!("You heal some health");
                    log.new_log(s, time.tick);
                }
            }
        } else {
            panic!("Trying to heal Entity that is not a ship!");
        }
    }

    // System done, transition to next state
    // state.set(AppState::AwaitingInput).unwrap();
    state.set(AppState::IncrementTime).unwrap();
}
