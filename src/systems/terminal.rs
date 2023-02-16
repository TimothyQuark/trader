use bevy::prelude::*;

use super::{
    map::{wall_glyph, Map, MapTileType},
    time::GameTime,
};
use crate::components::map::Position;
use crate::components::rendering::{
    BottomSidebar, Renderable, RightSidebar, TerminalTile, TopSidebar,
};
use crate::text::char_to_cp437;
use crate::text::{default_textstyle, DefaultTextStyle};

// Layer order for different entities. Tiles at the back, text at the front
const TILE_LAYER: f32 = 0.0;
const TEXT_LAYER: f32 = 1.0;

/// Terminal resource, contains all important information about the
/// Game Window, such as screen dimensions, screen tile dimensions etc.
#[derive(Resource)]
pub struct Terminal {
    // TODO: Tile is currently a square, change to be a rectangle
    tile_size: u32,
    screen_width: u32,
    screen_height: u32,
    terminal_width: u32,
    terminal_height: u32,

    pub terminal_tiles: Vec<(usize, Color)>, // Vec<(SpriteIndex, Color)

    top_sidebar_text: String,
    bottom_sidebar_text: Vec<String>,
    right_sidebar_text: Vec<String>,

    // In number of tiles. Fully dimensions the terminal
    // TODO: Make private, accessible only with function. Also add calculation
    // for right_sidebar_width, not attribute but still useful.
    pub top_sidebar_height: u32,
    pub bottom_sidebar_height: u32,
    pub right_sidebar_width: u32,
}

impl Default for Terminal {
    /// Returns default Terminal resource.
    ///
    /// Tile size: 20 pixels
    /// Screen width: 1080 pixels
    /// Screen height: 720 pixels
    fn default() -> Self {
        let tile_size = 20;
        let screen_width = 1080;
        let screen_height = 720;

        let terminal_width = screen_width / tile_size;
        let terminal_height = screen_height / tile_size;

        Self {
            tile_size,
            screen_width,
            screen_height,
            terminal_width,
            terminal_height,
            terminal_tiles: vec![
                (0, Color::BLUE);
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            top_sidebar_text: "This is default text".to_string(),
            bottom_sidebar_text: vec!["Bottom sidebar text (From Terminal) \n".to_string(); 11],
            right_sidebar_text: vec!["Right sidebar text (From Terminal)\n".to_string(); 11],

            top_sidebar_height: 1,
            bottom_sidebar_height: 11,
            right_sidebar_width: 14,
        }
    }
}

impl Terminal {
    #![allow(dead_code)]
    /// Create Terminal resource with custom settings
    fn new(tile_size: u32, screen_width: u32, screen_height: u32) -> Self {
        // TODO: Other terminal settings should be customizable from here
        let terminal_width = screen_width / tile_size;
        let terminal_height = screen_height / tile_size;

        Self {
            tile_size,
            screen_width,
            screen_height,
            terminal_width,
            terminal_height,
            terminal_tiles: vec![
                (0, Color::BLUE);
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            top_sidebar_text: "This is default text".to_string(),
            ..Default::default()
        }
    }
    /// Returns screen dimensions, in pixels.
    ///
    /// (screen_width, screen_height)
    pub fn get_screen_dim(&self) -> (u32, u32) {
        (self.screen_width, self.screen_height)
    }

    /// Returns terminal dimensions, in tiles
    ///
    /// (terminal_width, terminal_height)
    pub fn get_terminal_dim(&self) -> (u32, u32) {
        (self.terminal_width, self.terminal_height)
    }

    /// Converts XY coordinate to index of terminal_tile
    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        ((y * self.terminal_width) + x) as usize
    }

    /// Converts terminal_tile index in tile vec to XY coordinate
    /// returns (x,y)
    pub fn idx_xy(&self, idx: u32) -> (u32, u32) {
        let x = idx % self.terminal_width;
        let y = (idx - x) / self.terminal_width;
        // let y = idx / self.width;

        (x, y)
    }

    /// Converts map coordinates to terminal coordinates. Note that this max return
    /// terminal coordinates that are out of range
    ///
    /// Returns: (term_x_idx, term_y_idx)
    fn map_coord_to_term_coord(&self, map_x_idx: u32, map_y_idx: u32) -> (u32, u32) {
        // let (map_x_idx, map_y_idx) = map.idx_xy(map_idx);

        // Shift map_y_idx up so it is not covered by the game log. Nothing need to
        // be done with map_x_idx for now.
        let term_y_idx = map_y_idx + self.bottom_sidebar_height;
        let term_x_idx = map_x_idx;

        (term_x_idx, term_y_idx)
    }
}

pub fn init_terminal(
    mut commands: Commands,
    assets: Res<AssetServer>,
    terminal: Res<Terminal>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Note that this system does not actually create the terminal resource,
    // that is done in the main app.

    // println!("Initializing the terminal");

    // println!("{}", terminal.get_screen_dim().0);

    // Load the default tile sheet
    // Load sprite sheet into a texture atlas
    let texture_handle = assets.load("cp437_20x20_transparent.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load the default font and text style, and add as a resource.
    // Note that resources may not be accessible to startup systems.
    let default_text_style = default_textstyle(assets);
    commands.insert_resource(DefaultTextStyle(default_text_style.clone()));

    // Spawn the Terminal Tile entities, which will be used to draw terminal contents
    // (terminal_tiles) to the screen
    // Bevy uses coordinate system where center of screen is (0,0), also
    // sprite translation is center of sprite. Need lots of awful
    // coordinate shifting
    let x_min: i32 = (-1 * terminal.screen_width as i32 / 2) + terminal.tile_size as i32 / 2;
    let x_max: i32 = (terminal.screen_width as i32) / 2;
    let y_min: i32 = (-1 * terminal.screen_height as i32 / 2) + terminal.tile_size as i32 / 2;
    let y_max: i32 = (terminal.screen_height as i32) / 2;

    let mut idx: usize = 0;
    // Order of these loops matters because it sets idx
    for y in (y_min..y_max).step_by(terminal.tile_size as usize) {
        for x in (x_min..x_max).step_by(terminal.tile_size as usize) {
            // println!("x:{}, y: {}", x, y);
            commands
                .spawn(SpriteSheetBundle {
                    transform: Transform {
                        // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                        translation: Vec3::new(x as f32, y as f32, TILE_LAYER),
                        scale: Vec3::splat(1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        color: Color::PINK,
                        index: 10,
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                })
                .insert(TerminalTile { idx })
                .insert(Name::new("Tile"));

            idx += 1;
        }
    }

    // Spawn top sidebar text
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "You should not be seeing this text",
                default_text_style.clone(),
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            }),
            transform: Transform {
                translation: Vec3::new(
                    x_min as f32 - (terminal.tile_size / 2) as f32,
                    y_max as f32 - (terminal.tile_size / 2) as f32,
                    TEXT_LAYER,
                ),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TopSidebar)
        .insert(Name::new("TopSideBar"));

    // Spawn bottom sidebar text
    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "------------------------------------- Add log text here (Should not see this text)\n".to_string(),
                        style: default_text_style.clone(),
                    };
                    // Number of sections should be as many lines as in the log
                    terminal.bottom_sidebar_height as usize
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left,
                },
            },
            transform: Transform {
                // translation: Vec3::new(-half_x as f32, (-half_y as f32) + BOTTOM_SIDEBAR, 0.0),
                translation: Vec3::new(x_min as f32 - (terminal.tile_size as f32 / 2.0),y_min as f32 - (terminal.tile_size as f32 / 2.0), TEXT_LAYER),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BottomSidebar)
        .insert(Name::new("BottomSidebar"));

    // Spawn right sidebar text
    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Line on the right side (Should not see this text)\n".to_string(),
                        style: default_text_style.clone(),
                    };
                    // Number of sections should be as many lines as in the log
                    // (terminal.terminal_height - terminal.top_sidebar_height) as usize
                    3
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                },
            },
            transform: Transform {
                // Start one line below the top sidebar so they do not overlap
                translation: Vec3::new(
                    x_max as f32 - (terminal.right_sidebar_width * terminal.tile_size) as f32,
                    y_max as f32 - (terminal.top_sidebar_height * terminal.tile_size) as f32,
                    TEXT_LAYER,
                ),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RightSidebar)
        .insert(Name::new("RightSidebar"));
}

/// System that renders the terminal every frame.
pub fn render_terminal(
    // mut commands: Commands,
    map: Res<Map>,
    mut terminal: ResMut<Terminal>,
    r_query: Query<(&Renderable, &Position), With<Renderable>>,
    // QuerySet limited to 4 QueryState
    mut p: ParamSet<(
        Query<(&mut Transform, &mut TextureAtlasSprite, &TerminalTile), With<TerminalTile>>,
        Query<&mut Text, With<TopSidebar>>,
        Query<&mut Text, With<RightSidebar>>,
        Query<&mut Text, With<BottomSidebar>>,
    )>,
) {
    // Update text of the top sidebar
    // let mut top_sidebar = q.q1().single_mut();
    // top_sidebar.sections[0].value = terminal.top_sidebar_text.clone();
    p.p1().single_mut().sections[0].value = terminal.top_sidebar_text.clone();

    // Update text of the right sidebar
    for (idx, mut line) in p.p2().single_mut().sections.iter_mut().enumerate() {
        // line.value = "Test \n".to_string();
        line.value = terminal.right_sidebar_text[idx].clone();
    }

    // Update text of the bottom sidebar
    for (idx, mut line) in p.p3().single_mut().sections.iter_mut().enumerate() {
        // line.value = "Test \n".to_string();
        line.value = terminal.bottom_sidebar_text[idx].clone();
    }

    // Update the tiles that draw the map
    for (map_idx, map_tile) in map.tiles.clone().into_iter().enumerate() {
        // let (map_x_idx, map_y_idx) = map.idx_xy(map_idx as u32);

        // // Shift map_y_idx up so it is not covered by the game log. Nothing need to
        // // be done with map_x_idx for now.
        // let term_y_idx = map_y_idx + terminal.bottom_sidebar_height;
        // let term_x_idx = map_x_idx;
        let (map_x_idx, map_y_idx) = map.idx_xy(map_idx);
        let (term_x_idx, term_y_idx) = terminal.map_coord_to_term_coord(map_x_idx, map_y_idx);

        if term_x_idx < (terminal.terminal_width - terminal.right_sidebar_width)
            && term_y_idx < terminal.terminal_height - terminal.top_sidebar_height
            && term_y_idx >= terminal.bottom_sidebar_height
        {
            // println!("map_idx: {}, map_x_idx: {}, map_y_idx: {}", map_idx, map_x_idx, map_y_idx);

            // Convert map_idx to terminal_idx
            let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);

            // Determine the correct glyph to show for the tile
            // Default map tile color is blue
            match map_tile {
                // TODO: Change map tile color based on environment
                // Wall tiles change based on their neighbors
                MapTileType::Space => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('.');
                    terminal.terminal_tiles[terminal_idx].1 = Color::BLACK;
                }
                MapTileType::Wall => {
                    terminal.terminal_tiles[terminal_idx].0 =
                        wall_glyph(&map, map_x_idx as i32, map_y_idx as i32) as usize;
                    terminal.terminal_tiles[terminal_idx].1 = Color::BLUE;
                }
                MapTileType::Placeholder => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('↓');
                    terminal.terminal_tiles[terminal_idx].1 = Color::GREEN;
                }
                MapTileType::Planet => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('O');
                    terminal.terminal_tiles[terminal_idx].1 = Color::SEA_GREEN;
                }
                MapTileType::Moon => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('o');
                    terminal.terminal_tiles[terminal_idx].1 = Color::BLUE;
                }
                MapTileType::Wormhole => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('!');
                    terminal.terminal_tiles[terminal_idx].1 = Color::RED;
                }
                MapTileType::Star => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('$');
                    terminal.terminal_tiles[terminal_idx].1 = Color::YELLOW;
                }
                MapTileType::Asteroid => {
                    terminal.terminal_tiles[terminal_idx].0 = char_to_cp437('A');
                    terminal.terminal_tiles[terminal_idx].1 = Color::SILVER;
                }
            }
        }
    }

    // Update the tiles that draw Renderable entities. Note that this replaces
    // what was drawn by the map.
    // Sort Renderable entities by their render_order. The lower the render_order,
    // the higher the priority. Thus, Player should have priority 0.
    let mut data = r_query.iter().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.0.render_order.cmp(&a.0.render_order));
    for (renderable, position) in data.iter() {
        // println!("Found a renderable!");

        let (term_x_idx, term_y_idx) =
            terminal.map_coord_to_term_coord(position.x as u32, position.y as u32);
        let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);
        terminal.terminal_tiles[terminal_idx].0 = char_to_cp437(renderable.glyph);
        terminal.terminal_tiles[terminal_idx].1 = renderable.fg;
    }

    // Render the glyphs and colors of the terminal tiles
    // let query_iter = q.q0().iter_mut();
    for tile in p.p0().iter_mut() {
        let (_, mut sprite, tile_component) = tile;
        sprite.index = terminal.terminal_tiles[tile_component.idx].0;
        sprite.color = terminal.terminal_tiles[tile_component.idx].1;
    }
}

/// System that updates contents of sidebars by updating text inside the Terminal resource
/// using data from other game resources
pub fn update_sidebars(mut terminal: ResMut<Terminal>, time: Res<GameTime>) {
    // Update top sidebar
    terminal.top_sidebar_text = String::from(format!("Turn: {}", time.tick));
}
