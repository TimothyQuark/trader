use bevy::prelude::*;

use super::{
    common::GameName,
    ships::{Ship, ShipStats},
};

/// Component that identifies if an entity wants to attack another entity
#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// Component that records how much damage an entity takes. Healing should use HealDamage instead
#[derive(Component)]
pub struct SufferDamage {
    amount: Vec<u32>, // Do not directly add to this vector, use new_damage instead
}

impl SufferDamage {
    pub fn new_damage(
        commands: &mut Commands,
        query: &mut Query<(&ShipStats, Option<&mut SufferDamage>, &GameName), With<Ship>>,
        victim: Entity,
        dmg: u32,
    ) {
        // println!("Entity {} is taking damage!", victim.index());

        // Check for entity, panic if not found
        if let Ok((_, opt, _)) = query.get_mut(victim) {
            // Check if entity has SufferDamage component
            if let Some(mut suffering) = opt {
                // println!("Entity {} is taking dmg again this turn!", victim.index());
                suffering.amount.push(dmg);
            } else {
                // println!(
                //     "Entity {} is taking dmg for first time this turn",
                //     victim.index()
                // );
                commands
                    .entity(victim)
                    .insert(SufferDamage { amount: vec![dmg] });
            }
        } else {
            panic!(
                "Entity {} suffering damage, but does not exist!",
                victim.index()
            )
        }
    }

    // Get the sum of the damage done to the entity (or heal)
    pub fn sum_dmg(&self) -> u32 {
        return self.amount.iter().sum::<u32>();
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct HealDamage {
    amount: Vec<u32>, // Do not directly add to this vector, use new_damage instead
}
