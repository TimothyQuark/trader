use bevy::prelude::*;

use pathfinding::prelude::astar;

use crate::components::{
    common::WaitTime,
    map::Position,
    ships::{CombatStats, Pirate, Player, ShipStats},
};
use crate::systems::map::Map;
use crate::AppState;

/// System responsible for pirate behaviour
/// Only runs when current State is TransitionTime
pub fn monster_ai(
    mut map: ResMut<Map>,
    mut state: ResMut<State<AppState>>,
    mut p: ParamSet<(
        Query<(&mut Position, &mut WaitTime, &ShipStats, &CombatStats), With<Pirate>>,
        Query<&Position, With<Player>>,
    )>,
) {
    // println!("Pirate AI running");

    // Keep the borrow checker happy, does not play well with QuerySets
    let goal: Position;
    {
        goal = p.p1().single().clone();
        // println!("Player at position: x: {}, y: {}", goal.x, goal.y);
    }

    let mut query = p.p0();

    for (mut mon_position, mut wait_time, ship_stats, combat_stats) in query.iter_mut() {
        // println!(
        //     "Monster at position: x: {}, y: {}",
        //     mon_position.x, mon_position.y
        // );

        // Monster only acts if it WaitTime is 0
        if wait_time.turns == 0 {
            let result = astar(
                mon_position.as_ref(),
                |p| p.successors(&map),
                |p| (p.distance(&goal) / 3) as u32,
                |p| *p == goal,
            );
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
                    println!("The monster attacks the player!");
                    wait_time.turns += combat_stats.melee_speed;
                } else {
                    panic!(
                        "Monster is on top of player.\n Player at position: x: {}, y: {}\n Monster at position: x: {}, y: {} \n
                        Path: {:?}",
                        goal.x, goal.y, mon_position.x, mon_position.y, path
                    )
                }
            }
        }
    }
}
