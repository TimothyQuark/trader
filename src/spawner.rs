use std::collections::HashMap;

use bevy::prelude::{Color, Commands};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::components::{
    common::Name,
    living::Monster,
    map::{BlockTile, Position},
    rendering::Renderable,
};
use crate::geometry::Rect;
use crate::systems::map::{Map, MapTileType};

const MAX_MONSTERS: i32 = 4;

pub struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new<S: ToString>(name: S, weight: i32) -> RandomEntry {
        RandomEntry {
            name: name.to_string(),
            weight,
        }
    }
}

pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add<S: ToString>(mut self, name: S, weight: i32) -> RandomTable {
        if weight > 0 {
            self.total_weight += weight;
            self.entries
                .push(RandomEntry::new(name.to_string(), weight));
        }
        self
    }

    pub fn roll(&self, rng: &mut SmallRng) -> String {
        if self.total_weight == 0 {
            return "None".to_string();
        }
        let mut roll = rng.gen_range(1..self.total_weight);
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].name.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "None".to_string()
    }
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        // .add("Orc", 1 + map_depth)
        .add("Orc", 10)
        .add("Health Potion", 7)
}

pub fn spawn_room(commands: &mut Commands, room: &Rect, map: &Map, map_depth: i32) {
    let mut possible_targets: Vec<usize> = Vec::new();

    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == MapTileType::Floor {
                possible_targets.push(idx);
            }
        }
    }

    spawn_region(commands, &possible_targets, &map, map_depth);
}

pub fn spawn_region(commands: &mut Commands, area: &[usize], map: &Map, map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    let mut rng = SmallRng::from_entropy();
    let num_spawns = i32::min(
        areas.len() as i32,
        rng.gen_range(1..MAX_MONSTERS + 3) + (map_depth - 1) - 3,
    );
    if num_spawns == 0 {
        return;
    }

    for _ in 0..num_spawns {
        let array_index = if areas.len() == 1 {
            0usize
        } else {
            (rng.gen_range(1..areas.len()) - 1) as usize
        };

        let map_idx = areas[array_index];
        spawn_points.insert(map_idx, spawn_table.roll(&mut rng));
        areas.remove(array_index);
    }

    for spawn in spawn_points.iter() {
        spawn_entity(commands, &spawn, &map);
    }
}

fn spawn_entity(commands: &mut Commands, spawn: &(&usize, &String), map: &Map) {
    let x = (*spawn.0 as u32 % map.width) as i32;
    let y = (*spawn.0 as u32 / map.width) as i32;

    match spawn.1.as_ref() {
        "Goblin" => goblin(commands, x, y),
        "Orc" => orc(commands, x, y),
        "Health Potion" => health_potion(commands, x, y),
        "None" => {}
        _ => {
            panic!("Attempting to spawn unknown entity: {}", spawn.1);
        }
    }

    // println!("Spawned a {}", spawn.1);
}

fn orc(commands: &mut Commands, x: i32, y: i32) {
    monster(commands, x, y, 'o', "Orc");
}
/// Spawn a goblin. Public function because it is often used for testing
pub fn goblin(commands: &mut Commands, x: i32, y: i32) {
    monster(commands, x, y, 'g', "Goblin");
}

fn monster<S: ToString>(commands: &mut Commands, x: i32, y: i32, glyph: char, name: S) {
    // ecs.create_entity()
    //     .with(Position{ x, y })
    //     .with(Renderable{
    //         glyph,
    //         fg: RGB::named(rltk::RED),
    //         bg: RGB::named(rltk::BLACK),
    //         render_order: 1
    //     })
    //     .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
    //     .with(Monster{})
    //     .with(Name{ name : name.to_string() })
    //     .with(BlocksTile{})
    //     .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
    //     .marked::<SimpleMarker<SerializeMe>>()
    //     .build();

    commands
        .spawn()
        .insert(Position { x, y })
        .insert(Renderable {
            glyph,
            fg: Color::RED,
            bg: Color::BLACK,
            render_order: 2,
        })
        .insert(Monster {})
        .insert(Name {
            name: name.to_string(),
            l_name: None,
        })
        .insert(BlockTile {});
}

fn health_potion(commands: &mut Commands, x: i32, y: i32) {
    // ecs.create_entity()
    //     .with(Position { x, y })
    //     .with(Renderable {
    //         glyph: rltk::to_cp437('¡'),
    //         fg: RGB::named(rltk::MAGENTA),
    //         bg: RGB::named(rltk::BLACK),
    //         render_order: 2,
    //     })
    //     .with(Name {
    //         name: "Health Potion".to_string(),
    //     })
    //     .with(Item {})
    //     .with(Consumable {})
    //     .with(ProvidesHealing { heal_amount: 8 })
    //     .marked::<SimpleMarker<SerializeMe>>()
    //     .build();

    commands
        .spawn()
        .insert(Position { x, y })
        .insert(Renderable {
            glyph: '¡',
            fg: Color::YELLOW,
            bg: Color::BLACK,
            render_order: 1,
        })
        .insert(Name {
            name: "Health Potion".to_string(),
            l_name: Some("Rejuvenating Potion of Good Health".to_string()),
        });
}
