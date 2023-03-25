use bevy::prelude::Component;

/// Component that reveals short and long name of Entity.\
/// Name is reserved by Bevy to identify entities non-uniquely
/// hence this name
#[derive(Component)]
pub struct GameName {
    /// Short name
    pub name: String,
    /// Long name
    pub l_name: Option<String>,
}

/// Component that flags an Entity to be deleted when
/// transitioning to a new level
#[derive(Component)]
pub struct DeleteOnNewLevel;
