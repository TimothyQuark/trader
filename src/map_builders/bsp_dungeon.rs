use bevy::prelude::*;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::{
    common::{apply_room_to_map, draw_corridor},
    MapBuilder,
};
use crate::components::map::Position;
use crate::geometry::Rect;
use crate::spawner::spawn_room;
use crate::systems::map::{Map, MapTileType};

pub struct BspDungeonBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    rooms: Vec<Rect>,
    // history: Vec<Map>,
    rects: Vec<Rect>,
}

impl MapBuilder for BspDungeonBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn build_map(&mut self) {
        self.build();
    }

    // Don't spawn anything in the first room (room with Player in it)
    fn spawn_entities(&mut self, commands: &mut Commands) {
        for room in self.rooms.iter().skip(1) {
            spawn_room(commands, room, &self.map, self.depth);
        }
    }
}

impl BspDungeonBuilder {
    pub fn new(new_depth: i32) -> Self {
        // println!("New BspDungeonBuilder created (map needs to be built)");
        BspDungeonBuilder {
            // TODO: Decouple map size from screen dimensions
            map: Map::new(40, 24),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            // history: Vec::new(),
            rects: Vec::new(),
        }
    }

    fn build(&mut self) {
        // let mut rng = SmallRng::seed_from_u64(100);
        let mut rng = SmallRng::from_entropy();
        // println!("{}", rng.gen_range(0..100));

        self.rects.clear();
        self.rects.push(Rect::new(
            1,
            1,
            self.map.width as i32 - 4,
            self.map.height as i32 - 4,
        )); // Start with a single map-sized rectangle
        let first_room: Rect = self.rects[0];
        self.add_subrects(first_room); // Divide the first room

        // // REMOVE THIS AFTER TESTING
        // apply_room_to_map(&mut self.map, &first_room);
        // println!(
        //     // "x1: {}, x2: {}, y1: {}, y2: {}",
        //     first_room.x1, first_room.x2, first_room.y1, first_room.y2
        // );

        // Up to 360 times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        let mut n_rooms = 0;
        while n_rooms < 360 {
            let rect = self.get_random_rect(&mut rng);
            let candidate = self.get_random_sub_rect(rect, &mut rng);

            if self.is_possible(candidate) {
                apply_room_to_map(&mut self.map, &candidate);
                self.rooms.push(candidate);
                self.add_subrects(rect);
                // self.take_snapshot();
            }

            n_rooms += 1;
        }

        // Now we sort the rooms
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));

        // Now we want corridors, which connect room to it's nearest left neighbor
        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.x1 + (rng.gen_range(1..=i32::abs(room.x1 - room.x2)));
            let start_y = room.y1 + (rng.gen_range(1..=i32::abs(room.y1 - room.y2)));
            let end_x = next_room.x1 + (rng.gen_range(1..=i32::abs(next_room.x1 - next_room.x2)));
            let end_y = next_room.y1 + (rng.gen_range(1..=i32::abs(next_room.y1 - next_room.y2)));
            draw_corridor(&mut self.map, start_x, start_y, end_x, end_y);
            // self.take_snapshot();
        }

        // Don't forget the stairs
        let stairs = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.xy_idx(stairs.0, stairs.1);
        self.map.tiles[stairs_idx as usize] = MapTileType::DownStairs;
        // self.take_snapshot();

        // Set player start
        let start = self.rooms[0].center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
        };

        // println!("BspDungeonBuilder has built a dungeon");
    }

    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::new(rect.x1, rect.y1, half_width, half_height));
        self.rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    fn get_random_rect(&mut self, rng: &mut SmallRng) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = rng.gen_range(1..=self.rects.len()) - 1;
        // println!("rand_rect idx: {}", idx);
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut SmallRng) -> Rect {
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.gen_range(1..=i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.gen_range(1..=i32::min(rect_height, 10)) - 1) + 1;

        result.x1 += rng.gen_range(1..=6) - 1;
        result.y1 += rng.gen_range(1..=6) - 1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                if x > self.map.width as i32 - 2 {
                    can_build = false;
                }
                if y > self.map.height as i32 - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    let idx = self.map.xy_idx(x, y);
                    if self.map.tiles[idx as usize] != MapTileType::Wall {
                        can_build = false;
                    }
                }
            }
        }
        // println!("Rectangle allowed: {}", can_build);
        can_build
    }
}
