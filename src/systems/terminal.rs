use bevy::color::palettes::css::{self, WHITE};
use bevy::sprite;
use bevy::{color, color::palettes, prelude::*};
// use super::{
// map::{wall_glyph, Map, MapTileType},
// time::GameTime,
// };

use crate::components::{
    //     common::GameName,
    rendering::{
        BackgroundTile, BottomSidebar, ForegroundTile, Renderable, RightSidebar, TerminalTile,
        TopSidebar,
    },
    //     map::Position,
    //     ships::{Player, ShipStats},
};

use crate::text::char_to_cp437;
use crate::text::{default_textstyle, DefaultTextStyle};

// Layer order for different entities. Tiles at the back, text at the front

const BACKGROUND_LAYER: f32 = 1.0;
const FOREGROUND_LAYER: f32 = BACKGROUND_LAYER + 1.0;
pub const TEXT_LAYER: f32 = FOREGROUND_LAYER + 1.0; // Use in other systems, kept here for organization

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
    pub foreground_tiles: Vec<(usize, Option<Srgba>)>, // Vec<(SpriteIndex, Color)
    pub background_tiles: Vec<(usize, Option<Srgba>)>, // Vec<(SpriteIndex, Color)

    // External systems can tell the Terminal to highlight tiles the next frame
    // Note this can be used to highlight tiles outside the map
    highlight_tiles: Vec<(usize, Srgba)>, // (terminal_idx, Color)

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
                (0, Some(Srgba::BLUE));
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            background_tiles: vec![
                (0, Some(css::PINK));
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
                (0, Some(Srgba::BLUE));
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
            background_tiles: vec![
                (0, Some(css::PINK));
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
    pub fn highlight_tiles(&mut self, tiles: &[(usize, Srgba)]) {
        self.highlight_tiles.extend(tiles);
    }
}

pub fn init_terminal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    terminal: Res<Terminal>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Initializing the terminal");

    // Load the default tile sheet
    // Load sprite sheet into a texture atlas
    // Using texture_handle and atlas_handle, you can now spawn sprites with Sprite::from_atlas_image
    let texture_handle: Handle<Image> = asset_server.load("cp437_20x20_transparent.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(20, 20), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load the default font and text style, and add as a resource.
    // Note that resources may not be accessible to startup systems.
    let default_text_style = default_textstyle(asset_server);
    commands.insert_resource(DefaultTextStyle(default_text_style.0.clone()));

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
            commands.spawn((
                Sprite {
                    color: Color::Srgba(css::PINK),
                    ..Sprite::from_atlas_image(
                        texture_handle.clone(), // Cloning the handle is cheap
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(), // Cloning the handle is cheap
                            index: 3,                             // Debug sprite
                        },
                    )
                },
                Transform {
                    // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                    translation: Vec3::new(x as f32, y as f32, FOREGROUND_LAYER),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                TerminalTile { idx },
                ForegroundTile,
                Name::new("ForegroundTile"),
            ));

            // Spawn background glyph tiles
            commands.spawn((
                Sprite {
                    color: Color::Srgba(css::GREEN),
                    ..Sprite::from_atlas_image(
                        texture_handle.clone(), // Cloning the handle is cheap
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(), // Cloning the handle is cheap
                            index: 10,                            // Debug sprite
                        },
                    )
                },
                Transform {
                    // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                    translation: Vec3::new(x as f32, y as f32, BACKGROUND_LAYER),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                TerminalTile { idx },
                BackgroundTile,
                Name::new("BackgroundTile"),
            ));

            idx += 1;
        }
    }

    // Spawn top sidebar text
    commands.spawn((
        Text2d::new("You should not be seeing this text"),
        default_text_style.clone(),
        Transform {
            translation: Vec3::new(
                x_min as f32 - (terminal.tile_size / 2) as f32,
                y_max as f32 - (terminal.tile_size / 2) as f32,
                TEXT_LAYER,
            ),
            scale: Vec3::ONE,
            ..Default::default()
        },
        sprite::Anchor::CenterLeft, // How a sprite is positioned relative to its Transform.
        TopSidebar,
        Name::new("TopSideBar"),
    ));

    // Spawn bottom sidebar text
    // TODO: See https://github.com/bevyengine/bevy/blob/v0.15.2/examples/2d/text2d.rs and https://bevyengine.org/learn/migration-guides/0-14-to-0-15/#textbundle-and-text-styling
}
