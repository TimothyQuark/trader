use bevy::prelude::*;

/// Resource that holds the game time
#[derive(Resource)]
pub struct Time {
    pub tick: u64,
}

// pub fn transition_time(mut commands)
