use bevy::prelude::*;

// Components that have to deal with time

/// Component that stores how many turns until Entity can take a new action
#[derive(Component)]
pub struct WaitTimer {
    pub turns: u64,
}

// Component that stores how many turns until Entity will regenerate a unit of health
#[derive(Component)]
pub struct HealthTimer {
    pub turns: u64,
}

// Component that stores how many turns until Entity will regenerate a unit of shield
#[derive(Component)]
pub struct ShieldTimer {
    pub turns: u64,
}
