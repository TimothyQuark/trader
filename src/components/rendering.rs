use bevy::prelude::*;

/// Component used to identify what to draw to terminal (i.e map tiles)
#[derive(Component)]
pub struct TerminalTile {
    pub idx: usize,
}

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
#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub render_order: i32, // Lower means higher render priority (Player Sprite has 0)
}
