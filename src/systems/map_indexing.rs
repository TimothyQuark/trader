use bevy::prelude::*;

use crate::components::map::{BlockTile, Position};
use crate::systems::map::Map;

/// System which updates important aspects of the map every frame,
/// such as which tiles are blocked, which contain Entities etc.
pub fn map_indexing(
    mut map: ResMut<Map>,
    mut query: Query<(Entity, &Position, Option<&BlockTile>), With<Position>>,
) {
    // Update which tiles are blocked (based on map tiles and entities)
    // populate_blocked will reset blocked_tiles to only consider map tiles
    map.populate_blocked();
    map.clear_content_index();

    for (entity, position, blocker) in query.iter_mut() {
        // println!("id: {}, position: {},{}", entity.id(), position.x, position.y);
        let idx = map.xy_idx(position.x, position.y);

        // If Entity blocks, update blocking list
        if let Some(_) = blocker {
            map.blocked_tiles[idx] = true;
        }

        // Add entity to the relevant map index
        map.tile_content[idx].push(entity);
    }
}
