// use bevy::ecs::archetype::Archetypes;
// use bevy::ecs::component::Components;
// use bevy::reflect::TypeRegistration;
// use bevy::prelude::*;

// use super::AppState;

// / System which prints all resources in the World to the console.
// / Useful for debugging
// #[allow(dead_code)]
// pub fn print_resources(archetypes: &Archetypes, components: &Components) {
//     let mut r: Vec<String> = archetypes
//         .resource()
//         .components()
//         .map(|id| components.get_info(id).unwrap())
//         // get_short_name removes the path information
//         // i.e. `bevy_audio::audio::Audio` -> `Audio`
//         // if you want to see the path info replace
//         // `TypeRegistration::get_short_name` with `String::from`
//         .map(|info| TypeRegistration::get_short_name(info.name()))
//         .collect();

//     println!("Resources currently in World: \n ------------------");
//     // sort list alphebetically
//     r.sort();
//     r.iter().for_each(|name| println!("{}", name));
// }

// /// Print current AppState to the command line
// pub fn log_current_state(state: Res<State<AppState>>)
// {
//     let state_str;

//     match state.current() {
//         AppState::NewMap => state_str = "NewMap",
//         AppState::InGame => state_str = "InGame",

//     }

//     println!("Game State changed to {}", state_str);
// }
