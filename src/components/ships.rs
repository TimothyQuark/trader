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
    pub curr_health: i32, // Health can be negative, means entity is dead and should be removed
    pub max_health: i32,
    pub health_regen: u64, // Number of turns to regenerate one unit of health
    pub curr_shields: i32, // Absorbs ranged attacks, recharges over time
    pub max_shields: i32,
    pub shield_regen: u64, // Number of turns to regenerate one unit of shield
    pub armor: u32,        // Flat reduction of melee attacks
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
