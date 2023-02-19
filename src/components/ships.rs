use bevy::prelude::*;

// TODO: Add Ship component

/// Component that identifies the player ship
#[derive(Component)]
pub struct Player;

/// Component that identifies a pirate
#[derive(Component)]
pub struct Pirate;

/// Component that identifies a ship
#[derive(Component)]
pub struct Ship;

/// Component containing non combat stats of ships
#[derive(Component, Debug)]
pub struct ShipStats {
    // Misc Stats
    pub fuel: u32,
    pub speed: u64, // Movement speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub storage: u32,

    // Combat Stats
    pub health: u32,
    pub armor: u32,   // Flat reduction of melee attacks
    pub shields: u32, // Absorbs ranged attacks, recharges over time
}

/// Component containing combat stats of ships
#[derive(Component)]
pub struct CombatStats {
    pub melee_speed: u64, // Melee weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub melee_dmg: u32,
    pub ranged_speed: u64, // Ranged weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub ranged_dmg: u32,
}
