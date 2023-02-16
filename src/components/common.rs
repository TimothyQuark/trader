use bevy::prelude::Component;

/// Component that reveals short and long name of entity
/// Name is used by Bevy to identify entities non-uniquely,\
/// hence this name
#[derive(Component)]
pub struct GameName {
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
