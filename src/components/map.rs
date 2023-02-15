use bevy::prelude::*;

use crate::systems::map::Map;

/// Component that describes xy position on the Map
#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Distance between 2 Positions
    pub fn distance(&self, other: &Position) -> i32 {
        // u32 result because astar heuristics wants this instead of i32
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }

    /// Neighboring Positions, used for pathfinding. Uses blocking map to remove
    /// neighbors that cannot be moved to
    /// Returns Vec(Position, move_cost: u32)
    pub fn successors(&self, map: &Map) -> Vec<(Position, u32)> {
        // Parent position
        let &Position { x, y } = self;

        // Filter the tiles that do not block
        // Note that moving diagonally has the same movement cost as moving in cardinal directions.
        let list = vec![
            Position { x: x - 1, y: y + 1 },
            Position { x: x, y: y + 1 },
            Position { x: x + 1, y: y + 1 },
            Position { x: x - 1, y: y },
            Position { x: x + 1, y: y },
            Position { x: x - 1, y: y - 1 },
            Position { x: x, y: y - 1 },
            Position { x: x + 1, y: y + 1 },
        ]
        .into_iter()
        .filter(|p| !map.blocked_tiles[map.xy_idx(p.x, p.y)])
        .map(|p| (p, 1))
        .collect();

        list
    }

    // pub fn as_tuple(&self) -> (i32, i32) {
    //     (self.x, self.y)
    // }
}

/// Component that states an entity is a blocker. Note that this is not used
/// for map tiles, which are not entities (Map is a resource)
#[derive(Component)]
pub struct BlockTile {}
