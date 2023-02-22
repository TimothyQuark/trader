use bevy::prelude::*;

use super::{
    map::{wall_glyph, Map, MapTileType},
    time::GameTime,
};
use crate::components::{
    common::GameName,
    rendering::{
        BackgroundTile, BottomSidebar, ForegroundTile, Renderable, RightSidebar, TerminalTile,
        TopSidebar,
    },
};
use crate::components::{
    map::Position,
    ships::{Player, ShipStats},
};
use crate::text::char_to_cp437;
use crate::text::{default_textstyle, DefaultTextStyle};

// Layer order for different entities. Tiles at the back, text at the front

const BACKGROUND_LAYER: f32 = 1.0;
const FOREGROUND_LAYER: f32 = 2.0;
pub const TEXT_LAYER: f32 = 3.0; // Use in other systems, kept here for organization

/// Resource that holds log entries of the game, which are printed to the bottom of the screen
#[derive(Resource, Default)]
pub struct GameLog {
    entries: Vec<String>,
}

impl GameLog {
    pub fn new_log(&mut self, entry: String, time: u64) {
        let s = format!("Turn {time}:  {entry}");
        self.entries.push(s);
    }
}

/// Terminal resource, contains all important information about the
/// Game Window, such as screen dimensions, screen tile dimensions etc.
#[derive(Resource)]
pub struct Terminal {
    // TODO: Tile is currently a square, change to be a rectangle
    pub tile_size: u32,
    screen_width: u32,
    screen_height: u32,
    pub terminal_width: u32,
    pub terminal_height: u32,

    // Tile layers. Each tile has ASCII code and color,
    // and its index is used to figure out where to render it
    // If None, then this tile is black (i.e. not rendered)
    pub foreground_tiles: Vec<(usize, Option<Color>)>, // Vec<(SpriteIndex, Color)
    pub background_tiles: Vec<(usize, Option<Color>)>, // Vec<(SpriteIndex, Color)

    // External systems can tell the Terminal to highlight tiles the next frame
    // Note this can be used to highlight tiles outside the map
    highlight_tiles: Vec<(usize, Color)>, // (terminal_idx, Color)

    // In number of tiles. Fully dimensions the terminal
    // TODO: Make private, accessible only with function. Also add calculation
    // for right_sidebar_width, not attribute but still useful.
    pub top_sidebar_height: u32,
    pub bottom_sidebar_height: u32,
    pub right_sidebar_width: u32,

    top_sidebar_text: String,
    bottom_sidebar_text: Vec<String>,
    right_sidebar_text: Vec<String>,
}

impl Default for Terminal {
    /// Returns default Terminal resource.
    ///
    /// Tile size: 20 pixels\
    /// Screen width: 1080 pixels\
    /// Screen height: 720 pixels\
    /// Top Sidebar: 1 tile
    /// Bottom Sidebar: 11 tiles
    /// Right Sidebar: 14 tiles
    fn default() -> Self {
        let tile_size = 20;
        let screen_width = 1080;
        let screen_height = 720;

        let terminal_width = screen_width / tile_size;
        let terminal_height = screen_height / tile_size;

        let top_sidebar_height = 1;
        let bottom_sidebar_height = 11;
        let right_sidebar_width = 14;

        Self {
            tile_size,
            screen_width,
            screen_height,
            terminal_width,
            terminal_height,
            foreground_tiles: vec![
                (0, Some(Color::BLUE));
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            background_tiles: vec![
                (0, Some(Color::PINK));
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            top_sidebar_height,
            bottom_sidebar_height,
            right_sidebar_width,

            top_sidebar_text: "This is default text".to_string(),
            bottom_sidebar_text: vec!["\n".to_string(); bottom_sidebar_height as usize],
            right_sidebar_text: vec![
                "Right sidebar text (From Terminal)\n".to_string();
                (terminal_height - bottom_sidebar_height - top_sidebar_height)
                    as usize
            ],
            highlight_tiles: vec![],
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
            foreground_tiles: vec![
                (0, Some(Color::BLUE));
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            background_tiles: vec![
                (0, Some(Color::PINK));
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
        (x, y)
    }

    /// Converts map coordinates to terminal coordinates.\
    /// Note that this may return terminal coordinates that are out of bounds
    ///
    /// Returns: (term_x_idx, term_y_idx)
    pub fn map_coord_to_term_coord(&self, map_x_idx: u32, map_y_idx: u32) -> (u32, u32) {
        let term_y_idx = map_y_idx + self.bottom_sidebar_height;
        let term_x_idx = map_x_idx;
        (term_x_idx, term_y_idx)
    }

    /// Tell the Terminal to highlight terminal tiles in next frame\
    /// Arguments: Slice[terminal_idx, Color]
    pub fn highlight_tiles(&mut self, tiles: &[(usize, Color)]) {
        self.highlight_tiles.extend(tiles);
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

            // Spawn foreground glyph tiles
            commands
                .spawn(SpriteSheetBundle {
                    transform: Transform {
                        // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                        translation: Vec3::new(x as f32, y as f32, FOREGROUND_LAYER),
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
                .insert(ForegroundTile)
                .insert(Name::new("ForegroundTile"));

            // Spawn background glyph tiles
            commands
                .spawn(SpriteSheetBundle {
                    transform: Transform {
                        // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                        translation: Vec3::new(x as f32, y as f32, BACKGROUND_LAYER),
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
                .insert(BackgroundTile)
                .insert(Name::new("BackgroundTile"));

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
                    (terminal.terminal_height - terminal.top_sidebar_height - terminal.bottom_sidebar_height) as usize


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
        Query<(
            // &mut Transform,
            &mut TextureAtlasSprite,
            &TerminalTile,
            Option<&ForegroundTile>,
            Option<&BackgroundTile>,
        )>,
        Query<&mut Text, With<TopSidebar>>,
        Query<&mut Text, With<RightSidebar>>,
        Query<&mut Text, With<BottomSidebar>>,
    )>,
) {
    // Update text of the top sidebar
    p.p1().single_mut().sections[0].value = terminal.top_sidebar_text.clone();

    // Update text of the right sidebar
    for (idx, mut line) in p.p2().single_mut().sections.iter_mut().enumerate() {
        line.value = terminal.right_sidebar_text[idx].clone();
    }

    // Update text of the bottom sidebar
    for (idx, mut line) in p.p3().single_mut().sections.iter_mut().enumerate() {
        line.value = terminal.bottom_sidebar_text[idx].clone();
    }

    // Update the contents of the tile layers (foreground_tiles and background_tiles) stored in the Terminal
    // that are used to render the map. By default, the map renders background to black
    for (map_idx, map_tile) in map.tiles.clone().into_iter().enumerate() {
        // let (map_x_idx, map_y_idx) = map.idx_xy(map_idx as u32);

        // Shift map_y_idx up so it is not covered by the game log. Nothing need to
        // be done with map_x_idx for now.
        let (map_x_idx, map_y_idx) = map.idx_xy(map_idx);
        let (term_x_idx, term_y_idx) = terminal.map_coord_to_term_coord(map_x_idx, map_y_idx);

        if term_x_idx < (terminal.terminal_width - terminal.right_sidebar_width)
            && term_y_idx < terminal.terminal_height - terminal.top_sidebar_height
            && term_y_idx >= terminal.bottom_sidebar_height
        {
            // println!("map_idx: {}, map_x_idx: {}, map_y_idx: {}", map_idx, map_x_idx, map_y_idx);

            // Convert map_idx to terminal_idx
            let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);

            // Determine the correct glyph and color to show for the foreground_tiles
            match map_tile {
                // TODO: Change map tile color based on environment
                // Wall tiles change based on their neighbors
                MapTileType::Space => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437(' ');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::WHITE);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Wall => {
                    terminal.foreground_tiles[terminal_idx].0 =
                        wall_glyph(&map, map_x_idx as i32, map_y_idx as i32) as usize;
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::BLUE);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Placeholder => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('↓');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::GREEN);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Planet => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('O');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::SEA_GREEN);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Moon => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('o');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::GREEN);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Wormhole => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('!');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::FUCHSIA);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Star => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('$');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::YELLOW);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
                MapTileType::Asteroid => {
                    terminal.foreground_tiles[terminal_idx].0 = char_to_cp437('A');
                    terminal.foreground_tiles[terminal_idx].1 = Some(Color::SILVER);
                    terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
                    terminal.background_tiles[terminal_idx].1 = Some(Color::BLACK);
                }
            }
        }
    }

    /*
    Iterate through Entities that should be drawn (have Renderable and Position components).
    This would include entities such as the player, other ships etc.
    Sort Renderable entities by their render order. The lower the render order, the
    higher the priority (Player has priority 0)
     */
    let mut data = r_query.iter().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.0.render_order.cmp(&a.0.render_order));
    for (renderable, position) in data.iter() {
        // Calculate terminal coords and index
        let (term_x_idx, term_y_idx) =
            terminal.map_coord_to_term_coord(position.x as u32, position.y as u32);
        let terminal_idx = terminal.xy_idx(term_x_idx, term_y_idx);

        // Update foreground tiles. Overwrites what the map rendered
        terminal.foreground_tiles[terminal_idx].0 = char_to_cp437(renderable.glyph);
        terminal.foreground_tiles[terminal_idx].1 = Some(renderable.fg);
        // If the Renderable has a background color, set bg color to it and glyph to 219 (full square)
        // Otherwise make background transparent (set to empty space)
        if let Some(bg) = renderable.bg {
            terminal.background_tiles[terminal_idx].0 = char_to_cp437('█');
            terminal.background_tiles[terminal_idx].1 = Some(bg);
        } else {
            terminal.background_tiles[terminal_idx].0 = char_to_cp437(' ');
            terminal.background_tiles[terminal_idx].1 = None;
        }
    }

    // Check if there are any tiles that should be highlighted. Clear after rendering for a single frame
    let highlighted_tiles = terminal.highlight_tiles.clone();
    for (idx, color) in highlighted_tiles {
        terminal.background_tiles[idx].0 = char_to_cp437('█');
        terminal.background_tiles[idx].1 = Some(color);
    }
    terminal.highlight_tiles.clear();

    // All tile layers have been updated, now use their contents to update the terminal tiles
    // Render the glyphs and colors of the terminal tiles
    for (mut sprite, tile, fg, bg) in p.p0().iter_mut() {
        // Tile to update is foreground
        if let Some(_) = fg {
            sprite.index = terminal.foreground_tiles[tile.idx].0;
            sprite.color = terminal.foreground_tiles[tile.idx].1.unwrap();
        }
        // Tile to update is background
        else if let Some(_) = bg {
            sprite.index = terminal.background_tiles[tile.idx].0;
            if let Some(color) = terminal.background_tiles[tile.idx].1 {
                sprite.color = color;
            } else {
                // Do not render background, i.e. black tile
                // Sprite index has already been set to 219 previously
                sprite.color = Color::BLACK;
            }
            // println!("Rendering bg tile: index {}, color {:?}", sprite.index, sprite.color);
        } else {
            panic!("TerminalTile found that is missing an identification of its render layer");
        }
    }
}

/// System that updates contents of sidebars by updating text inside the Terminal resource
/// using data from other game resources
pub fn update_sidebars(
    mut terminal: ResMut<Terminal>,
    time: Res<GameTime>,
    mut log: ResMut<GameLog>,
    query: Query<(&ShipStats, &GameName), With<Player>>,
) {
    // Player info
    let (ship_stats, name) = query.single();

    // Update top sidebar
    terminal.top_sidebar_text = String::from(format!("Turn: {}", time.tick));

    // Update right sidebar
    for (idx, line) in &mut terminal.right_sidebar_text.iter_mut().enumerate() {
        line.clear();
        line.push_str("  ");
        match idx {
            0 => line.push_str(&format!("{}'s Stats\n", name.name)),
            1 => *line += "\n",
            2 => line.push_str(&format!(
                "Health: {}/{}\n",
                ship_stats.curr_health, ship_stats.max_health
            )),
            3 => line.push_str(&format!("Fuel: {}\n", ship_stats.fuel)),
            4 => line.push_str(&format!("Flying Speed: {}\n", ship_stats.speed)),
            5 => line.push_str(&format!(
                "Storage: {}/{}\n",
                ship_stats.storage, ship_stats.storage
            )), // TODO: Show this as a fraction (ex 45/245)
            6 => *line += "\n",
            7 => line.push_str(&format!("Armor: {}\n", ship_stats.armor)),
            8 => line.push_str(&format!(
                "Shields: {}/{}\n",
                ship_stats.curr_shields, ship_stats.curr_shields
            )), // TODO: Show as fraction
            9 => line.push_str(&format!("Gatling Firerate: {}\n", ship_stats.melee_speed)),
            10 => line.push_str(&format!("Gatling Damage: {}\n", ship_stats.melee_dmg)),
            11 => line.push_str(&format!("Laser Firerate: {}\n", ship_stats.ranged_speed)),
            12 => line.push_str(&format!("Laser Damage: {}\n", ship_stats.ranged_dmg)),
            _ => {}
        }
    }

    // Update bottom sidebar using GameLog
    // let last = (terminal.bottom_sidebar_height - 1) as usize;
    // TODO: Make log start from top of sidebar. Probably need to switch out the loops, iterate
    // over terminal.bottom_sidebar_text instead of log.entries

    // Hacky way to populate game log first time
    if log.entries.len() == 0 {
        log.new_log(String::from("Welcome to Space Trader"), time.tick);
        log.new_log(
            String::from("You can find more info in the README on Github"),
            time.tick,
        );
    }
    'outer: for (idx, s) in log.entries.iter().rev().enumerate() {
        if idx >= terminal.bottom_sidebar_height as usize {
            break 'outer;
        }
        terminal.bottom_sidebar_text[idx] = s.to_string() + "\n";
    }
}
