use bevy::prelude::*;

/// Component that identifies the player ship
#[derive(Component)]
pub struct Player;

/// Component that identifies a pirate ship
#[derive(Component)]
pub struct Pirate {}

#[derive(Component)]
pub struct ShipStats {
    // Misc Stats
    pub fuel: u32,
    pub speed: u64, // Movement speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub storage: u32,

    // Combat Stats
    pub health: u32,
    pub armor: u32,
    pub shields: u32,
}

#[derive(Component)]
pub struct CombatStats {
    pub melee_speed: u64, // Melee weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub ranged_speed: u64, // Ranged weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
}
