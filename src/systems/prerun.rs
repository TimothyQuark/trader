use bevy::prelude::*;

use crate::AppState;

pub fn pre_run(mut state: ResMut<State<AppState>>) {
    println!("Prerun running");

    state.overwrite_replace(AppState::AwaitingInput).unwrap();
}
