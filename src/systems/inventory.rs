use bevy::prelude::*;

use crate::AppState;

enum InventoryActions {
    ExitInventory,
}

/// System responsible for drawing the Inventory Menu, as well as processing user input while
/// in the Inventory Menu
pub fn inventory_menu(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    println!("Inside the inventory menu!");

    if keys.just_pressed(KeyCode::KeyX) {
        println!("Leaving the inventory menu!");
        // Return to AwaitingInput state when exiting inventory
        next_state.set(AppState::AwaitingInput);
    }
}
