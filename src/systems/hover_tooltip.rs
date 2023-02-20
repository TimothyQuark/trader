use bevy::prelude::*;

use super::{
    map::{Map, MapTileType},
    terminal::Terminal,
};

pub fn tooltip(windows: Res<Windows>, mut terminal: ResMut<Terminal>, mut map: ResMut<Map>) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        // println!("Cursor at position x:{} y:{}", position.x, position.y);
        let pix_x = position.x / terminal.tile_size as f32;
        let pix_y = position.y / terminal.tile_size as f32;
        let term_x = pix_x.floor() as u32;
        let term_y = pix_y.floor() as u32;
        // println!("Cursor at position x:{} y:{}", term_x, term_y);

        // Iterate through all map tiles, and check if mouse if hovering over them
        for (map_idx, map_tile) in map.tiles.clone().into_iter().enumerate() {
            // Shift map_y_idx up so it is not covered by the game log. Nothing need to
            // be done with map_x_idx for now.
            let (map_x_idx, map_y_idx) = map.idx_xy(map_idx);
            let (term_x_idx, term_y_idx) = terminal.map_coord_to_term_coord(map_x_idx, map_y_idx);

            if term_x_idx < (terminal.terminal_width - terminal.right_sidebar_width)
                && term_y_idx < terminal.terminal_height - terminal.top_sidebar_height
                && term_y_idx >= terminal.bottom_sidebar_height
            {
                // let tile = &map.tiles[map_idx];
                // let entities = &map.tile_content[map_idx];
                let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);

                if (term_x == term_x_idx) & (term_y == term_y_idx) {
                    // println!(
                    //     "Found a match! x:{} y:{} is a {:?} tile",
                    //     term_x_idx, term_y_idx, tile
                    // );
                    // println!("Entities found here: {:?}", entities);
                    terminal.highlight_tiles(&[(terminal_idx, Color::PINK)]);
                    // terminal.background_tiles[terminal_idx].1 = Some(Color::PINK);
                    // map.tiles[map_idx] = MapTileType::Placeholder;
                }
                // println!("map_idx: {}, map_x_idx: {}, map_y_idx: {}", map_idx, map_x_idx, map_y_idx);
                // println!("term_x_idx: {}, term_y_idx: {}", term_x_idx, term_y_idx);
            } else {
                println!("Not inside map");
            }
        }
    } else {
        // cursor is not inside the window
        // println!("Cursor outside window");
    }
}
