// use bevy::prelude::*;
// use std::cmp::min;

// use super::components::gamelog::*;
// use super::components::rendering::*;
// use super::default_textstyle;

// // TODO: Move into systems mod and completely rewrite cleanly

// // Take the entries in the game log resource, and display them on the bottom
// // sidebar
// pub fn draw_gamelog_system(
//     log_resource: Res<GameLog>,
//     mut query: Query<&mut Text, With<BottomSidebar>>,
// ) {
//     // There should only be one bottom sidebar component in the World
//     let mut sidebar = query.single_mut();
//     // .expect("There should only be one bottom sidebar");
//     let idx = min(sidebar.sections.len(), log_resource.entries.len());

//     for line in 0..idx {
//         sidebar.sections[line] = log_resource.entries[line].clone();
//     }

//     // What I originally wanted to do, but this does not sit well with Rust
//     // for mut sect in &sidebar.sections {
//     //     sect.value = log_resource.entries[line].value.clone();
//     // }
// }

// /// Creates the GameLog resource, using a default text style
// pub fn init_gamelog_system(mut commands: Commands, assets: Res<AssetServer>) {
//     println!("Setup Gamelog");

//     // Cannot use DefaultTextStyle because resources not available to
//     // startup systems
//     let text_style = default_textstyle(assets);

//     commands.insert_resource(GameLog {
//         entries: vec![
//             TextSection {
//                 value: "This should be a new line in the gamelog\n".to_string(),
//                 style: text_style,
//             };
//             3
//         ],
//     });
// }
