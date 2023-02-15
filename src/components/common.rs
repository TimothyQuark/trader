use bevy::prelude::Component;

/// Component that reveals short and long name of entity
#[derive(Component)]
pub struct Name {
    /// Short name
    pub name: String,
    /// Long name
    pub l_name: Option<String>,
}

// Component that stores how many turns until entity can take a new action
#[derive(Component)]
pub struct WaitTime {
    pub turns: u64,
}
