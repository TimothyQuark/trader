use crate::AppState;
use bevy::prelude::*;

// Marker component for menu entities
#[derive(Component)]
pub struct MainMenuUI;

// Marker for the new game button
#[derive(Component)]
pub struct NewGameButton;

// Marker for the quit button
#[derive(Component)]
pub struct QuitButton;

pub fn setup_main_menu(mut commands: Commands) {
    // Root node for the menu
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("TRADER"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));

            // New Game Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("New Game"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Quit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

pub fn button_system(
    mut new_game_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NewGameButton>),
    >,
    mut quit_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<QuitButton>,
            Without<NewGameButton>,
        ),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    // Handle New Game button
    for (interaction, mut color) in &mut new_game_query {
        match *interaction {
            Interaction::Pressed => {
                color.0 = Color::srgb(0.35, 0.75, 0.35);
                next_state.set(AppState::NewGame);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.35, 0.35, 0.35);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
        }
    }

    // Handle Quit button
    for (interaction, mut color) in &mut quit_query {
        match *interaction {
            Interaction::Pressed => {
                color.0 = Color::srgb(0.75, 0.35, 0.35);
                exit.write(AppExit::Success);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.35, 0.35, 0.35);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}
