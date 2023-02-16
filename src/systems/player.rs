use bevy::prelude::*;

use crate::components::{
    common::{GameName, WaitTime},
    map::Position,
    rendering::Renderable,
    ships::{Player, ShipStats},
};

/// Spawn the player entity
pub fn init_player(mut commands: Commands) {
    // println!("Player initialized");
    // Simple way to create an entity and return an id directly inside the function.
    let _player = commands
        .spawn_empty()
        .insert(Player)
        .insert(Name::new("Player")) // Used by Bevy, can see name in WorldDebugger
        .insert(GameName {
            name: String::from("Player"),
            l_name: None,
        })
        .insert(WaitTime { turns: 0 })
        .insert(Renderable {
            glyph: '@',
            fg: Color::WHITE,
            bg: Color::BLACK,
            render_order: 0,
        })
        .insert(Position { x: 0, y: 0 })
        .insert(ShipStats {
            fuel: 100,
            speed: 10,
            storage: 1000,
            health: 17,
            armor: 5,
            shields: 13,
        })
        .id();
}
