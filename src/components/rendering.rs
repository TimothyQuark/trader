use bevy::prelude::*;

// Component attached to the 2D camera to make it easily identifiable
#[derive(Component)]
pub struct MainCamera;

/// Component used to identify what to draw to terminal (i.e map tiles)
#[derive(Component)]
pub struct TerminalTile {
    pub idx: usize, // Index of the tile (used to place it correctly on the screen)
}

#[derive(Component)]
pub struct ForegroundTile;

#[derive(Component)]
pub struct BackgroundTile;

// struct LeftSidebarText;

/// Component to identify the RightSidebar entity
#[derive(Component)]
pub struct RightSidebar;

/// Component to identify the BottomSidebar entity
#[derive(Component)]
pub struct BottomSidebar;

/// Component to identify the TopSidebar entity
#[derive(Component)]
pub struct TopSidebar;

/// Component that identifies entities that should be rendered to the terminal
/// Not all Renderables have a background color (ex: Player)
#[derive(Component, Reflect)]

pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Option<Color>,
    pub render_order: i32, // Lower means higher render priority (Player Sprite has 0)
}

/// Component that identifies an Entity as being the tooltip when the mouse hovers over something
/// on the map
#[derive(Component)]
pub struct MouseTooltip;
