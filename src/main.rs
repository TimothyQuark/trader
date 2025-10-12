#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*
By default, Rust will spawn a console for Windows applications. This conditional config will check if
the game was compiled in release mode, and if yes, will suppress the console. Otherwise,
we can still use the console window for debugging in dev and test mode. In the future,
a logger should be used to check errors users experience.
More info:
https://doc.rust-lang.org/reference/conditional-compilation.html
https://github.com/rust-lang/rust/issues/67159
https://github.com/rust-lang/rust/issues/67159#issuecomment-987882771 (possible workaround if we want output when program started in cmd)
https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
 */

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
// use bevy::log::LogPlugin;
use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod map_builders;
use map_builders::build_new_map;

pub mod components;

mod systems;
use systems::{
    camera::init_camera,
    damage_system::{damage_system, delete_the_dead},
    hover_tooltip::{map_tooltip, run_map_tooltip},
    // debugging::debug_states,
    input::player_input,
    inventory::inventory_menu,
    map::init_map,
    map_indexing::map_indexing,
    melee::melee_combat_system,
    pirate_ai::pirate_ai,
    player::init_player,
    regen::regen_health,
    terminal::{init_terminal, render_terminal, update_sidebars, GameLog, Terminal},
    time::{increment_time, GameTime},
};

mod geometry;
mod spawner;
mod text;
mod utilities;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
// #[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Resource, Default, InspectorOptions)]
// #[reflect(Resource, InspectorOptions)]
pub enum AppState {
    #[default]
    LoadGame, // Basically everything that falls into StartUp
    MainMenu,
    NewGame,
    NextLevel,
    AwaitingInput,
    IncrementTime,
    RunAI,
    RunCombat,
    RunDamage,
    DeleteDead,
    RunTimers,
    GameOver,
    InventoryMenu,
}

fn main() {
    // Terminal resource
    let terminal = Terminal::default();
    let (screen_width, screen_height) = terminal.get_screen_dim();

    // App Builder.
    App::new()
        // Window example: https://github.com/bevyengine/bevy/blob/v0.17.2/examples/window/window_settings.rs
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Trader".to_string(),
                resolution: (screen_width as u32, screen_height as u32).into(),
                resizable: false,
                ..Default::default()
            }),
            ..default()
        }))
        // .add_plugins(WorldInspectorPlugin)
        // .register_type::<Renderable>()
        // .add_plugins(ResourceInspectorPlugin::<AppState>::default()) // Debug a resource
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_systems(Update, debug_states)
        // .init_resource::<ReportExecutionOrderAmbiguities>() // Use to look at execution order in LogPlugin
        // Resources
        .insert_resource(ClearColor(Color::BLACK)) // App bg color. Will see if there are problems
        .insert_resource(terminal)
        .insert_resource(GameTime { tick: 0 })
        .insert_resource(GameLog::default())
        // .register_type::<AppState>() // use for ResourceInspectorPlugin
        // Starting State
        .init_state::<AppState>()
        // Startup systems
        .add_systems(Startup, (init_camera, init_terminal, init_map, init_player))
        // Render Systems (run every frame in Update schedule)
        // .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, render_terminal)
        .add_systems(Update, update_sidebars)
        .add_systems(Update, map_indexing)
        // Map tooltip system with run condition
        .add_systems(Update, map_tooltip.run_if(run_map_tooltip))
        // Inventory menu system
        .add_systems(
            Update,
            inventory_menu.run_if(in_state(AppState::InventoryMenu)),
        )
        // Game Systems
        // TODO: On new game, clear the World
        .add_systems(OnEnter(AppState::NewGame), build_new_map)
        .add_systems(OnEnter(AppState::NextLevel), build_new_map)
        .add_systems(
            Update,
            increment_time.run_if(in_state(AppState::IncrementTime)),
        )
        .add_systems(
            Update,
            player_input.run_if(in_state(AppState::AwaitingInput)),
        )
        .add_systems(Update, pirate_ai.run_if(in_state(AppState::RunAI)))
        .add_systems(
            Update,
            melee_combat_system.run_if(in_state(AppState::RunCombat)),
        )
        .add_systems(Update, damage_system.run_if(in_state(AppState::RunDamage)))
        .add_systems(
            Update,
            delete_the_dead.run_if(in_state(AppState::DeleteDead)),
        )
        .add_systems(Update, regen_health.run_if(in_state(AppState::RunTimers)))
        .run();
}
