use bevy::prelude::*;

use crate::{
    components::{
        common::GameName,
        map::Position,
        rendering::{MainCamera, Renderable, MouseTooltip},
        ships::{Player, ShipStats},
    },
    utilities::convert_cursor_to_world_coords,
};

use super::{
    map::Map,
    terminal::{Terminal, TEXT_LAYER},
};

pub fn tooltip(
    mut commands: Commands,
    assets: Res<AssetServer>,
    windows: Res<Windows>,
    mut terminal: ResMut<Terminal>,
    map: Res<Map>,
    query: Query<(&Position, &ShipStats, &GameName, Option<&Player>), With<Renderable>>,
    h_query: Query<Entity, With<MouseTooltip>>,
    c_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();

    // Despawn all tooltips every, we will recreate them all every frame
    // TODO: This is wasteful, in future reuse the entities
    for entity in h_query.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(position) = window.cursor_position() {
        // println!("Cursor at position x:{} y:{}", position.x, position.y);
        let pix_x = position.x / terminal.tile_size as f32;
        let pix_y = position.y / terminal.tile_size as f32;
        let mouse_term_x = pix_x.floor() as u32;
        let mouse_term_y = pix_y.floor() as u32;
        // println!("Cursor at position x:{} y:{}", term_x, term_y);

        // Iterate through all map tiles, and check if mouse if is hovering over a tile
        for (map_idx, _map_tile) in map.tiles.clone().into_iter().enumerate() {
            // Shift map_y_idx up so it is not covered by the game log. Nothing need to
            // be done with map_x_idx for now.
            let (map_x_idx, map_y_idx) = map.idx_xy(map_idx);
            let (term_x_idx, term_y_idx) = terminal.map_coord_to_term_coord(map_x_idx, map_y_idx);

            if term_x_idx < (terminal.terminal_width - terminal.right_sidebar_width)
                && term_y_idx < terminal.terminal_height - terminal.top_sidebar_height
                && term_y_idx >= terminal.bottom_sidebar_height
            {
                // let tile = &map.tiles[map_idx];
                let entities = &map.tile_content[map_idx];
                let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);

                // Mouse is over a map tile
                if (mouse_term_x == term_x_idx) & (mouse_term_y == term_y_idx) {
                    // println!(
                    //     "Found a match! x:{} y:{} is a {:?} tile",
                    //     term_x_idx, term_y_idx, tile
                    // );
                    // println!("Entities found here: {:?}", entities);
                    terminal.highlight_tiles(&[(terminal_idx, Color::PINK)]);

                    // Convert mouse coordinates to world coordinates
                    let world_coords = convert_cursor_to_world_coords(&windows, &c_query).unwrap();

                    if entities.len() > 0 {
                        for e in entities {
                            show_entity_info(*e, &mut commands, &assets, &query, world_coords);
                        }
                    } else {
                    }

                    // map.tiles[map_idx] = MapTileType::Placeholder;
                }
                // println!("map_idx: {}, map_x_idx: {}, map_y_idx: {}", map_idx, map_x_idx, map_y_idx);
                // println!("term_x_idx: {}, term_y_idx: {}", term_x_idx, term_y_idx);
            } else {
                // println!("Not inside map");
            }
        }
    } else {
        // cursor is not inside the window
        // println!("Cursor outside window");
    }
}

fn show_entity_info(
    entity: Entity,
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    queue: &Query<(&Position, &ShipStats, &GameName, Option<&Player>), With<Renderable>>,
    world_coords: Vec2,
) {
    // TODO: This has not yet been tested yet for multiple entities on a single tile

    let font = assets.load("square.ttf");

    // Entities that are rendered should always have GameName, else panic
    let name = &queue.get_component::<GameName>(entity).unwrap().name;
    let mut lines = vec![name.clone()];

    if let Ok(ship_stats) = queue.get_component::<ShipStats>(entity) {
        lines.push(format!("HP: {}", ship_stats.health));
        lines.push(format!("SH: {}", ship_stats.shields));
    }

    if let Ok(_) = queue.get_component::<Player>(entity) {
        // println!("Found the player!");
    }

    // Shift the tooltip so it isn't directly over the entity
    let x = world_coords.x + 10.0;
    let y = world_coords.y + 10.0;

    // Spawn Mousetooltip entity
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                lines.join("\n"),
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            }),
            transform: Transform {
                translation: Vec3::new(x, y, TEXT_LAYER),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MouseTooltip)
        .insert(Name::new("MouseTooltip"));

    // println!("{:?}", lines);
}
