use bevy::prelude::*;

use super::ships::{ShipStats, Ship};


/// Component that identifies if an entity wants to attack another entity
#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// Component that records how much damage an entity takes/heals in a turn
#[derive(Component)]
pub struct SufferDamage {
    amount: Vec<i32>, // Do not directly add to this vector, use new_damage instead
}

impl SufferDamage {
    pub fn new_damage(
        commands: &mut Commands,
        query: &mut Query<(&ShipStats, Option<&mut SufferDamage>), With<Ship>>,
        victim: Entity,
        dmg: u32,
    ) {
        if let Ok((_, opt)) = query.get_mut(victim) {
            // Entity has already taken damage, push additional damage
            if let Some(mut suffering) = opt {
                suffering.amount.push(dmg as i32);
            }
            // suffering.amount.push(dmg as i32);
        } else {
            // Entity has not taken damage this turn, add new component
            commands.entity(victim).insert(SufferDamage {
                amount: vec![dmg as i32],
            });
        }
    }
}
