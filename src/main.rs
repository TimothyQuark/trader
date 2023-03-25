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
use bevy::window::WindowMode;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Resource, Default)]
// #[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Resource, Default, InspectorOptions)]
// #[reflect(Resource, InspectorOptions)]
pub enum AppState {
    MainMenu,
    #[default]
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
        // Starting State
        .add_state(AppState::NewGame)
        // Resources
        .insert_resource(ClearColor(Color::BLACK)) // App bg color. Will see if there are problems
        .insert_resource(terminal)
        .insert_resource(GameTime { tick: 0 })
        .insert_resource(GameLog::default())
        // .register_type::<AppState>() // use for ResourceInspectorPlugin
        // Plugins & Debuggers
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Space Trader".to_string(),
                width: screen_width as f32,
                height: screen_height as f32,
                resizable: false,
                mode: WindowMode::Windowed,
                ..Default::default()
            },
            ..default()
        }))
        // .add_plugin(WorldInspectorPlugin)
        // .register_type::<Renderable>()
        // .add_plugin(ResourceInspectorPlugin::<AppState>::default()) // Debug a resource
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(debug_states)
        // .init_resource::<ReportExecutionOrderAmbiguities>() // Use to look at execution order in LogPlugin
        .add_system(bevy::window::close_on_esc) // Used for debugging
        // Startup systems
        .add_startup_system(init_camera.label("init_camera"))
        .add_startup_system(init_terminal)
        .add_startup_system(init_map)
        .add_startup_system(init_player)
        // Render Systems
        .add_system_set(
            SystemSet::new()
                .label("RenderTerminal")
                .with_system(render_terminal)
                .with_system(update_sidebars)
                .with_system(map_indexing),
        )
        .add_system_set(
            SystemSet::new()
                .label("MapTooltip")
                .with_system(map_tooltip)
                .with_run_criteria(run_map_tooltip),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InventoryMenu)
                .label("InventoryMenu")
                .with_system(inventory_menu),
        )
        // Game Systems
        // TODO: On new game, clear the World
        .add_system_set(
            SystemSet::on_enter(AppState::NewGame)
                .label("NewGame")
                .with_system(build_new_map),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::NextLevel)
                .label("NextLevel")
                .with_system(build_new_map),
        )
        .add_system_set(
            SystemSet::on_update(AppState::IncrementTime)
                .label("IncrementTime")
                .with_system(increment_time),
        )
        .add_system_set(
            SystemSet::on_update(AppState::AwaitingInput)
                .label("PlayerTurn")
                .with_system(player_input),
        )
        .add_system_set(
            SystemSet::on_update(AppState::RunAI)
                .label("RunAI")
                .with_system(pirate_ai),
        )
        .add_system_set(
            SystemSet::on_update(AppState::RunCombat)
                .label("RunCombat")
                .with_system(melee_combat_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::RunDamage)
                .label("RunDamage")
                .with_system(damage_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::DeleteDead)
                .label("DeleteDead")
                .with_system(delete_the_dead),
        )
        .add_system_set(
            SystemSet::on_update(AppState::RunTimers)
                .label("RunTimers")
                .with_system(regen_health),
        )
        .run();
}
