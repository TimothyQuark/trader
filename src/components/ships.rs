use bevy::prelude::*;

/// Component that identifies the Player ship
#[derive(Component)]
pub struct Player;

/// Component that identifies a Pirate
#[derive(Component)]
pub struct Pirate;

/// Component that identifies a Ship
#[derive(Component)]
pub struct Ship;

/// Component containing ship stats, such as those used for combat
#[derive(Component, Debug)]
pub struct ShipStats {
    // Misc Stats
    pub fuel: u32,
    pub speed: u64, // Movement speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub storage: u32,

    // Combat Stats
    pub health: i32, // Health can be negative, means entity is dead and should be removed
    pub armor: u32,  // Flat reduction of melee attacks
    pub shields: u32, // Absorbs ranged attacks, recharges over time
    pub melee_speed: u64, // Melee weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub melee_dmg: u32,
    pub ranged_speed: u64, // Ranged weapon speed, lower is better. Corresponds to turns in WaitingTime / GameTime ticks
    pub ranged_dmg: u32,

    // TODO: Modifiers
    /*
    speed_modifier
    shield_regen_rate
    max_shields
    max_health
     */
}
