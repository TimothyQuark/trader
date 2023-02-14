// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;

mod map_builders;
use map_builders::build_new_map;

pub mod components;

mod systems;
use systems::{
    camera::init_camera,
    input::player_input,
    map::init_map,
    map_indexing::map_indexing,
    player::init_player,
    terminal::{init_terminal, render_terminal, Terminal},
};

mod geometry;
mod text;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    NewGame,
    InGame,
    AwaitingInput,
    MonsterTurn,
}

fn main() {
    // Terminal resource
    let terminal = Terminal::default();
    let (screen_width, screen_height) = terminal.get_screen_dim();

    // App Builder.
    App::new()
        // Starting State
        .add_state(AppState::NewGame)
        // Resources
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(terminal)
        // Important plugins and debug helpers
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "DWorld".to_string(),
                width: screen_width as f32,
                height: screen_height as f32,
                resizable: false,
                mode: WindowMode::Windowed,
                ..Default::default()
            },
            ..default()
        }))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::window::close_on_esc)
        //Startup systems
        .add_startup_system(init_camera.label("init_camera"))
        .add_startup_system(init_terminal)
        .add_startup_system(init_map)
        .add_startup_system(init_player)
        .add_system_set(SystemSet::on_enter(AppState::NewGame).with_system(build_new_map))
        // Normal Systems
        .add_system(render_terminal)
        .add_system(map_indexing)
        .add_system_set(SystemSet::on_update(AppState::AwaitingInput).with_system(player_input))
        .run();
}
