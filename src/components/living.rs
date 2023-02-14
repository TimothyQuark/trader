use bevy::prelude::*;

/// Component that identifies the player ship
#[derive(Component)]
pub struct Player;

/// Component that identifies a ship that is not the player
#[derive(Component)]
pub struct Ship {}

/// Component that identifies a planet
pub struct Planet {}
