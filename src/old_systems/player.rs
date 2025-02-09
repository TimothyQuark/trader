use bevy::prelude::*;

use crate::components::{
    common::GameName,
    map::Position,
    rendering::Renderable,
    ships::{Player, Ship, ShipStats},
    timers::{HealthTimer, WaitTimer},
};

/// Spawn the player entity
pub fn init_player(mut commands: Commands) {
    // println!("Player initialized");
    // Simple way to create an entity and return an id directly inside the function.
    let _player = commands
        .spawn_empty()
        .insert(Player)
        .insert(Ship)
        .insert(Name::new("Player")) // Used by Bevy, can see name in WorldDebugger
        .insert(GameName {
            name: String::from("Enterprise"),
            l_name: None,
        })
        .insert(WaitTimer { turns: 0 })
        .insert(Renderable {
            glyph: '@',
            fg: Color::WHITE,
            bg: None,
            render_order: 0,
        })
        .insert(Position { x: 0, y: 0 })
        .insert(ShipStats {
            fuel: 100,
            speed: 2,
            storage: 1000,
            max_health: 10,
            curr_health: 10,
            health_regen: 10,
            armor: 0,
            max_shields: 5,
            curr_shields: 5,
            shield_regen: 1,
            melee_speed: 2,
            melee_dmg: 4,
            ranged_speed: 5,
            ranged_dmg: 1,
        })
        .insert(HealthTimer { turns: 0 })
        .id();
}
