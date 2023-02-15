use bevy::prelude::*;

use crate::components::{
    common::{Name, WaitTime},
    living::Player,
    map::Position,
    rendering::Renderable,
};

/// Spawn the player entity
pub fn init_player(mut commands: Commands) {
    // println!("Player initialized");
    // Simple way to create an entity and return an id directly inside the function.
    let _player = commands
        .spawn_empty()
        .insert(Player)
        .insert(Name {
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
        .id();
}
