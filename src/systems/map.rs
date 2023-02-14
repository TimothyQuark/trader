use bevy::prelude::*;

use crate::text::char_to_cp437;

/// Component that describes the kind of tile on a Map
#[derive(PartialEq, Copy, Clone)]
pub enum MapTileType {
    Placeholder, // Should never be rendered unless there are problems
    Wall,        // Walls in space :D
    Space,       // Empty space
    Planet,      // Planet entity
}

/// Resource that holds the game map
#[derive(Clone, Resource)]
pub struct Map {
    pub tiles: Vec<MapTileType>,
    pub width: u32,
    pub height: u32,
    pub revealed_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,

    // Each tile has a vec of entities that are ontop of it
    pub tile_content: Vec<Vec<Entity>>,
}

impl Default for Map {
    /// Create a tile map of walls
    fn default() -> Self {
        // println!("Default map initialized (still need to add as a resource)");
        let width: u32 = 40;
        let height: u32 = 24;

        // Downstairs tiles so it is obvious this should not be used
        Self {
            tiles: vec![MapTileType::Placeholder; (width * height) as usize],
            width,
            height,
            revealed_tiles: vec![true; (width * height) as usize],
            blocked_tiles: vec![true; (width * height) as usize],
            tile_content: vec![Vec::new(); (width * height) as usize],
        }
    }
}

impl Map {
    /// Create a new map consisting of only Wall tiles
    pub fn new(width: u32, height: u32) -> Map {
        let map = Map {
            tiles: vec![MapTileType::Wall; (width * height) as usize],
            width,
            height,
            revealed_tiles: vec![true; (width * height) as usize],
            blocked_tiles: vec![true; (width * height) as usize],
            tile_content: vec![Vec::new(); (width * height) as usize],
        };
        // println!("New Map created (still need to add as a resource)");

        map
    }

    /// Converts XY coordinate to index in tile vec
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width as i32) + x) as usize
    }

    /// Converts index in tile vec to XY coordinate
    /// returns (x,y)
    pub fn idx_xy(&self, idx: usize) -> (u32, u32) {
        let x = idx as u32 % self.width;
        let y = (idx as u32 - x) / self.width;
        // let y = idx / self.width;

        (x, y)
    }

    /// Iterate through all map tiles, and determine if the terrain is blocking
    /// (Ignores entities on top of the map tile)
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked_tiles[i] = *tile == MapTileType::Wall;
        }
    }

    /// Clear the tile_content list, which holds references to which entities
    /// are on top of every map tile
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

/// System that initializes a default map on app start
pub fn init_map(mut commands: Commands) {
    let map = Map::default();

    commands.insert_resource(map);
}

/// Check if a map tile is a wall and is revealed (useful for rendering)
fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    // println!("x: {}, y: {}, idx: {}", x, y, idx);
    map.tiles[idx] == MapTileType::Wall && map.revealed_tiles[idx]
}

/// Determines the correct wall glyph to be used for a wall tile,
/// based on how many neighboring wall tiles it has
pub fn wall_glyph(map: &Map, x: i32, y: i32) -> u8 {
    // Walls on edge of map will default to basic wall, because their neighbors
    // are out of bounds of map_tiles vec
    if x < 1 || x > map.width as i32 - 2 || y < 1 || y > map.height as i32 - 2 {
        return 35;
    }

    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    // The code from bracket tutorial has errors in the code, have fixed them
    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 187,  // Wall to the north and west
        6 => 188,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 201,  // Wall to the north and east
        10 => 200, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 203, // Wall to the east, west, and south
        14 => 202, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 0,    // We missed one?
    }
}

/// Convert a map tile to cp437 code
#[allow(dead_code)]
pub fn maptile_to_cp437(tile: MapTileType) -> usize {
    match tile {
        MapTileType::Wall => char_to_cp437('#'),
        MapTileType::Space => char_to_cp437('.'),
        MapTileType::Placeholder => char_to_cp437('↓'),
        MapTileType::Planet => char_to_cp437('O'),
    }
}
