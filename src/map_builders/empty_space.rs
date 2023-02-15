use bevy::prelude::*;

use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

use super::{Map, MapBuilder};
use crate::components::map::Position;
use crate::systems::map::MapTileType;

pub struct EmptySpaceBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
}

impl MapBuilder for EmptySpaceBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn build_map(&mut self) {
        self.build()
    }

    fn spawn_entities(&mut self, commands: &mut Commands) {}
}

impl EmptySpaceBuilder {
    pub fn new(_: i32) -> Self {
        EmptySpaceBuilder {
            map: Map::new(40, 24),
            starting_position: Position { x: 0, y: 0 },
            depth: 0,
        }
    }

    fn build(&mut self) {
        let mut rng = SmallRng::from_entropy();
        // let mut rng = SmallRng::seed_from_u64(100);

        // Spawn star in middle of map
        let (center_x, center_y) = (self.map.width as i32 / 2, self.map.height as i32 / 2);
        let star_idx = self.map.xy_idx(center_x, center_y);
        self.map.tiles[star_idx as usize] = MapTileType::Star;

        // Spawn 1-5 planets with different (i.e. unique) radii around star
        let num_planets = rng.gen_range(1..=5);
        println!("Number of planets generated: {}", num_planets);
        let allowed_r = (1i32..i32::min(center_x, center_y)) // Don't allow radius out of screen range
            .collect::<Vec<_>>();
        let radii = allowed_r.iter().choose_multiple(&mut rng, num_planets);
        for &r in radii {
            let angle: f64 = rng.gen_range(0.0..360.0);
            let planet_x: i32 = center_x + angle.cos().round() as i32 * r;
            let planet_y = center_y + angle.sin().round() as i32 * r;
            let planet_idx = self.map.xy_idx(planet_x, planet_y);
            self.map.tiles[planet_idx as usize] = MapTileType::Planet;
        }

        // Spawn 1-40 asteroids (50% chance of no asteroids at all, 40% of light, 10% of heavy)
        match rng.gen_bool(0.5) {
            true => {
                let num_asteroids = if rng.gen_bool(0.8) {
                    rng.gen_range(1..=5) // Light asteroids
                } else {
                    rng.gen_range(20..=40) // Heavy asteroids
                };
                for _ in 0..num_asteroids {
                    let mut tries = 0; // Try to place asteroid through brute force
                    'outer: while tries < 500 {
                        let asteroid_idx = rng.gen_range(0..self.map.width * self.map.height);
                        let candidate = self.map.tiles[asteroid_idx as usize];
                        if candidate == MapTileType::Space {
                            self.map.tiles[asteroid_idx as usize] = MapTileType::Asteroid;
                            break 'outer;
                        } else {
                            tries += 1;
                        }
                    }
                }
            }
            false => {
                // no asteroids
            }
        }
    }
}
