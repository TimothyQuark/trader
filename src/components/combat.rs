use bevy::prelude::*;

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

impl SufferDamage {}
