use bevy::prelude::Component;

#[derive(Component)]
pub struct Name {
    /// Short name
    pub name: String,
    /// Long name
    pub l_name: Option<String>,
}
