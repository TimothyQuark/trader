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
    pub speed: u32, // Lower is better
    pub storage: u32,

    // Combat Stats
    pub health: u32,
    pub armor: u32,
    pub shields: u32,
}
