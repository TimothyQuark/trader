use bevy::prelude::*;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::{
    components::{
        combat::SufferDamage,
        common::GameName,
        map::Position,
        ships::{Pirate, Player, Ship, ShipStats},
    },
    spawner::spawn_debris,
    AppState,
};

use super::{terminal::GameLog, time::GameTime};

/// System for calculating damage. Checks if an entity has
/// any of the damage components attached to it, and then processes
/// and removes them\
/// Once system is done, transitions to the next state
pub fn damage_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut query: Query<(Entity, &mut ShipStats, &SufferDamage)>,
) {
    // println!("Damage System running!");

    for (entity, mut ship_stats, suffer) in query.iter_mut() {
        ship_stats.curr_health -= suffer.sum_dmg() as i32;

        // println!(
        //     "Entity {} has taken {} damage",
        //     entity.index(),
        //     suffer.sum_dmg()
        // );

        // Entity has now taken damage, rmeove component
        commands.entity(entity).remove::<SufferDamage>();

        // Preemptively delete entity if it runs out of health
        // if ship_stats.health < 1 {
        //     println!("Preemptively deleting entity {} with subzero health", entity.index());
        //     commands.entity(entity).despawn();
        // }
    }

    // Transition to next state, DeleteDead
    state.set(AppState::DeleteDead).unwrap();
}

/// System that deletes entities if they have zero or subzero health.\
/// Once completed, transitions to next state
pub fn delete_the_dead(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut log: ResMut<GameLog>,
    time: Res<GameTime>,
    ship_query: Query<
        (
            Entity,
            &ShipStats,
            &Position,
            Option<&Player>,
            &GameName,
            Option<&Pirate>,
        ),
        With<Ship>,
    >,
) {
    // TODO: A better method is to instead have a component IsDead, that is added to an Entity if a system determines they are dead
    // Other systems will check if this Option exists, and if it does, then the system skips them.
    // When a turn is done, this system will then despawn all dead entities

    // println!("Deleting the Dead!");

    let mut rng = SmallRng::from_entropy();

    for (entity, ship_stats, pos, player, name, pirate) in ship_query.iter() {
        if let Some(_) = player {
            if ship_stats.curr_health < 1 {
                println!("The player has died! Game over");
                let s = format!("You have died! Game Over");
                log.new_log(s, time.tick);
                state.set(AppState::GameOver).unwrap();
                return;
            }
        } else if ship_stats.curr_health < 1 {
            println!("Deleting entity {} with subzero health", entity.index());
            commands.entity(entity).despawn();
            let s = format!("The {} has died", name.name);
            log.new_log(s, time.tick);

            // Check if pirate. Chance to spawn debris that can be looted for items
            if let Some(_) = pirate {
                if rng.gen_bool(0.5) {
                    println!("Pirate has spawned debris!");
                    spawn_debris(&mut commands, pos.x, pos.y);
                }
            }
        }
    }

    // Transition to next state, IncrementTime
    state.set(AppState::RunTimers).unwrap();
}
