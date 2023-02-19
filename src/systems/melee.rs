use bevy::prelude::*;

use crate::components::{
    combat::{SufferDamage, WantsToMelee},
    ships::{CombatStats, ShipStats, Ship},
};

// TODO: Add With statements to ensure safety of Queries
pub fn melee_combat_system(
    mut commands: Commands,
    attack_q: Query<(Entity, &WantsToMelee, &ShipStats, &CombatStats)>,
    mut target_q: Query<(&ShipStats, Option<&mut SufferDamage>), With<Ship>>,
) {
    // All immutable to please the borrow checker
    for (entity, wants_melee, ship_stats, combat_stats) in attack_q.iter() {
        // Attack only attacks if their health is positive. So if entity killed this turn, does not do ghost attack
        if ship_stats.health > 0 {
            // Get target entity's stats
            println!(
                "Entity {} wants to attack target {}",
                entity.index(),
                wants_melee.target.index()
            );
            // You can only get components that are in the Query
            let target_stats = target_q.get_component::<ShipStats>(wants_melee.target).unwrap();

            let damage = u32::max(0, combat_stats.melee_dmg - target_stats.armor);
            if damage == 0 {
                // TODO: Print to game console, not terminal
                println!(
                    "Entity {} is unable to hurt entity {}",
                    entity.index(),
                    wants_melee.target.index()
                );
            } else {
                // TODO: Print to game console, not terminal
                println!(
                    "Entity {} hurts entity {} for {} damage",
                    entity.index(),
                    wants_melee.target.index(),
                    damage
                );
                SufferDamage::new_damage(&mut commands, &mut target_q, wants_melee.target, 2);
            }
        }
    }
}
