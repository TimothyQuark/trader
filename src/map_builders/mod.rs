use bevy::prelude::*;

// use rand::rngs::SmallRng;
// use rand::{Rng, SeedableRng};

use crate::components::{map::Position, ships::Player};
use crate::systems::map::Map;
use crate::AppState;

// mod empty_room;
// use empty_room::EmptyRoomBuilder;
mod empty_space;
use empty_space::EmptySpaceBuilder;

mod common;
// use common::apply_room_to_map;

// Since this system is very large, warrents its own folder

/// System which generates map, populates it with entities and does other tasks.
pub fn build_new_map(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut query: Query<&mut Position, With<Player>>,
) {
    let new_depth = 1;

    // let mut rng_gen = SmallRng::seed_from_u64(100);
    // let mut rng_gen = SmallRng::from_entropy();
    // If 0 included, empty room can spawn
    // let rng = rng_gen.gen_range(0..=1);
    let rng = 0;
    let mut result: Box<dyn MapBuilder>;

    match rng {
        // 0 so rng will never select this builder
        0 => {
            result = Box::new(EmptySpaceBuilder::new(new_depth));
            result.build_map();
        }

        _ => {
            panic!("Undefined map builder selected: {}", rng);
        }
    }

    // This will rewrite the previous map resource
    let mut new_map = result.get_map();
    new_map.populate_blocked();
    commands.insert_resource(new_map);

    // Move the player to starting position
    let player_pos = result.get_starting_position();
    query.single_mut().x = player_pos.x;
    query.single_mut().y = player_pos.y;

    // Spawn entities on the map
    result.spawn_entities(&mut commands);

    // Change Game State to awaiting input
    state.overwrite_replace(AppState::AwaitingInput).unwrap();

    // println!("New map created and inserted as a resource");
}

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, commands: &mut Commands);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    // fn get_snapshot_history(&self) -> Vec<Map>;
    // fn take_snapshot(&mut self);
}
