use bevy::prelude::*;

use crate::components::{combat::SufferDamage, ships::ShipStats};

pub fn damage_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ShipStats, &SufferDamage)>,
) {
    for (entity, mut ship_stats, suffer) in query.iter_mut() {
        ship_stats.health -= suffer.sum_dmg() as i32;

        println!(
            "Entity {} has taken {} damage",
            entity.index(),
            suffer.sum_dmg()
        );

        // Entity has taken damage
        commands.entity(entity).remove::<SufferDamage>();
    }
}

pub fn delete_the_dead(mut commands: Commands, query: Query<(Entity, &ShipStats)>) {
    for (entity, ship_stats) in query.iter() {
        if ship_stats.health < 1 {
            println!("Deleting entity {} with subzero health", entity.index());
            commands.entity(entity).despawn();
        }
    }
}
