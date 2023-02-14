use crate::geometry::Rect;
use crate::systems::map::Map;
use crate::systems::map::MapTileType;

pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = map.xy_idx(x, y);
            // println!("idx in apply_room_to_map. x: {}, y: {} idx: {}", x, y, idx);
            if idx > 0 && idx < ((map.width * map.height) - 1) as usize {
                map.tiles[idx as usize] = MapTileType::Floor;
            }
            // else {
            //     panic!(
            //         "Trying to apply room tile that is out of map range.x:{}, y:{}, idx:{}. max_idx: {}",
            //         x,
            //         y,
            //         idx,
            //         (map.width * map.height) - 1
            //     );
            // }
        }
    }
}

pub fn draw_corridor(map: &mut Map, x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.xy_idx(x, y);
        map.tiles[idx as usize] = MapTileType::Floor;
    }
}
