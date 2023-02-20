use bevy::prelude::*;

use pathfinding::prelude::astar;

use crate::components::{
    combat::WantsToMelee,
    common::WaitTime,
    map::Position,
    ships::{Pirate, Player, ShipStats},
};
use crate::systems::map::Map;
use crate::AppState;

use super::time::GameTime;

/// System responsible for pirate behaviour\
/// Transitions to next AppState when finished
pub fn pirate_ai(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut state: ResMut<State<AppState>>,
    _time: Res<GameTime>,
    mut p: ParamSet<(
        Query<(Entity, &mut Position, &mut WaitTime, &ShipStats), With<Pirate>>,
        Query<(Entity, &Position), With<Player>>,
    )>,
) {
    // println!("Pirate AI running");

    // Keep the borrow checker happy, does not play well with QuerySets
    let goal: Position;
    let player_entity: Entity;
    {
        goal = p.p1().single().1.clone();
        player_entity = p.p1().single().0;
        // println!("Player at position: x: {}, y: {}", goal.x, goal.y);
    }

    let mut query = p.p0();

    // When not debugging, use _mon_ent instead of _ so we remember next time
    for (mon_ent, mut mon_position, mut wait_time, ship_stats) in query.iter_mut() {
        // println!(
        //     "Monster at position: x: {}, y: {}",
        //     mon_position.x, mon_position.y
        // );

        // Monster only acts if its WaitTime is 0
        if wait_time.turns == 0 {
            let result = astar(
                mon_position.as_ref(),
                |p| p.successors(&map),
                |p| (p.distance(&goal) / 3) as u32,
                |p| *p == goal,
            );

            // println!(
            //     "Monster {} takes a action on turn {}",
            //     mon_ent.index(),
            //     time.tick
            // );

            // println!("Path: {:?}", result);

            // Path to player found
            if let Some((path, _total_cost)) = result {
                if path.len() > 2 {
                    let mut idx = map.xy_idx(mon_position.x, mon_position.y);

                    // Old position of monster is no longer blocked
                    map.blocked_tiles[idx] = false;

                    // Update monster position to first position in path (0th is starting position)
                    mon_position.x = path[1].x;
                    mon_position.y = path[1].y;

                    // New position of monster is now blocked
                    idx = map.xy_idx(mon_position.x, mon_position.y);
                    map.blocked_tiles[idx] = true;

                    wait_time.turns += ship_stats.speed;
                } else if path.len() == 2 {
                    // Monster next to player (path is player_pos and mon_pos, hence len == 2), use melee attack
                    // println!(
                    //     "The monster {} attacks the player on turn {}",
                    //     mon_ent.index(),
                    //     time.tick
                    // );

                    commands.entity(mon_ent).insert(WantsToMelee {
                        target: player_entity,
                    });

                    wait_time.turns += ship_stats.melee_speed;
                } else {
                    panic!(
                        "Monster {} is on top of player.\n Player at position: x: {}, y: {}\n Monster at position: x: {}, y: {} \n
                        Path: {:?}",
                        mon_ent.index(), goal.x, goal.y, mon_position.x, mon_position.y, path
                    )
                }
            }
        }
    }

    // Pirate AI is done, so transition to next state, RunCombat
    state.set(AppState::RunCombat).unwrap();
}
