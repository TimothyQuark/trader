use bevy::prelude::*;

use rand::{
    distributions::WeightedIndex, prelude::*, rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng,
};

use super::{common::apply_room_to_map, Map, MapBuilder};
use crate::components::map::Position;
use crate::geometry::Rect;
use crate::spawner::spawn_room;
use crate::systems::map::MapTileType;

/// The most basic Map Builder\
/// Creates an empty room, and populates it with points of interest
pub struct EmptySpaceBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    room: Rect,
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

    fn spawn_entities(&mut self, commands: &mut Commands) {
        spawn_room(commands, &self.room, &self.map, self.depth);
    }
}

impl EmptySpaceBuilder {
    pub fn new(_: i32) -> Self {
        EmptySpaceBuilder {
            map: Map::new(40, 24),
            starting_position: Position { x: 0, y: 0 },
            depth: 0,
            room: Rect::new(0, 0, 1, 1),
        }
    }

    fn build(&mut self) {
        let mut rng = SmallRng::from_entropy();
        // let mut rng = SmallRng::seed_from_u64(100); // Static seed

        // Create edge of map (explicit so player knows)
        self.room = Rect::new(1, 1, self.map.width as i32 - 4, self.map.height as i32 - 4);
        apply_room_to_map(&mut self.map, &self.room);

        // Spawn star in middle of map
        let (center_x, center_y) = (self.map.width as i32 / 2, self.map.height as i32 / 2);
        let star_idx = self.map.xy_idx(center_x, center_y);
        self.map.tiles[star_idx as usize] = MapTileType::Star;

        // Spawn 1-5 planets with different (i.e. unique) radii around star.
        // Don't spawn planets within 2 units of star, looks cleaner
        let num_planets = rng.gen_range(0..=10);
        // println!("Number of planets generated: {}", num_planets);
        // Planets must be inside of system wall (-2) and min 2 tiles away from star (3)
        let allowed_r = (3i32..i32::min(center_x - 2, center_y - 2)).collect::<Vec<_>>();
        let radii = allowed_r.iter().choose_multiple(&mut rng, num_planets);
        for &r in radii {
            let angle: f64 = rng.gen_range(0.0..360.0);
            let planet_x: i32 = center_x + angle.cos().round() as i32 * r;
            let planet_y = center_y + angle.sin().round() as i32 * r;
            let planet_idx = self.map.xy_idx(planet_x, planet_y);
            self.map.tiles[planet_idx as usize] = MapTileType::Planet;

            // Spawn moons around planets. 50% of no moon, 40% 1 moon, 10% 2 moon
            let moon_choices = [0, 1, 2];
            let moon_weights = [0.6, 0.3, 0.1];
            let moon_dist = WeightedIndex::new(&moon_weights).unwrap();

            let num_moons = moon_choices[moon_dist.sample(&mut rng)];
            // println!("Number of moons generated: {}", num_moons);

            for _ in 0..num_moons {
                // Check random neighbors for empty spaces, place moons
                let pos = Position {
                    x: planet_x,
                    y: planet_y,
                };
                let neighbors = pos.neighbors(&self.map, MapTileType::Space);
                let rand_neighbor = neighbors.iter().choose(&mut rng).unwrap();
                let idx = self.map.xy_idx(rand_neighbor.x, rand_neighbor.y);
                self.map.tiles[idx as usize] = MapTileType::Moon;
            }
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

        // Spawn wormhole in random location.
        for attempt in 0..500 {
            let candidate_idx = rng.gen_range(0..self.map.total_tiles());
            if self.map.tiles[candidate_idx as usize] == MapTileType::Space {
                self.map.tiles[candidate_idx as usize] = MapTileType::Wormhole;
                break;
            } else if attempt == 500 {
                // If for some reason we cannot find a place for wormhole, replace star
                // Must always be possible to leave the system
                self.map.tiles[star_idx as usize] = MapTileType::Wormhole;
            }
        }

        // Spawn player as close to bottom left as possible
        'outer: for (idx, tile) in self.map.tiles.iter_mut().enumerate() {
            if *tile == MapTileType::Space {
                let (x, y) = self.map.idx_xy(idx);
                self.starting_position.x = x as i32;
                self.starting_position.y = y as i32;
                break 'outer;
            }
        }
    }
}
