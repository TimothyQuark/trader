use bevy::prelude::*;

use crate::{
    components::{
        combat::{SufferDamage, WantsToMelee},
        ships::{CombatStats, Ship, ShipStats},
    },
    AppState,
};

// TODO: Add With statements to ensure safety of Queries
pub fn melee_combat_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    attack_q: Query<(Entity, &WantsToMelee, &ShipStats, &CombatStats)>,
    mut target_q: Query<(&ShipStats, Option<&mut SufferDamage>), With<Ship>>,
) {
    // println!("Melee Combat running");

    // Note that because of how the systems are run, dead units can still attack.
    // This is kind of thematic, as if a ship shoots a gun, then the bullets will still travel
    // even if the ship gets killed

    // All immutable to please the borrow checker
    for (entity, wants_melee, ship_stats, combat_stats) in attack_q.iter() {
        // Attacker only attacks if their health is positive. So if entity killed this turn, does not do ghost attack
        if ship_stats.health > 0 {
            // Get target entity's stats
            println!(
                "Entity {} wants to attack target {} for {} melee damage (pre mitigation)",
                entity.index(),
                wants_melee.target.index(),
                combat_stats.melee_dmg
            );
            // You can only get components that are in the Query
            let target_stats = target_q
                .get_component::<ShipStats>(wants_melee.target)
                .unwrap();

            let damage = i32::max(0, combat_stats.melee_dmg as i32 - target_stats.armor as i32);
            if damage == 0 {
                // TODO: Print to game console, not terminal
                println!(
                    "Entity {} is unable to hurt entity {} (post mitigation)",
                    entity.index(),
                    wants_melee.target.index()
                );
            } else {
                // TODO: Print to game console, not terminal
                println!(
                    "Entity {} will hurt entity {} for {} melee damage (post mitigation)",
                    entity.index(),
                    wants_melee.target.index(),
                    damage
                );
                SufferDamage::new_damage(
                    &mut commands,
                    &mut target_q,
                    wants_melee.target,
                    damage as u32,
                );
            }
        }

        // Entity no longer wants to melee
        commands.entity(entity).remove::<WantsToMelee>();
    }

    // Combat done, transition to RunDamage
    state.set(AppState::RunDamage).unwrap();
}
