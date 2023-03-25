use bevy::prelude::*;

use crate::AppState;

enum InventoryActions {
    ExitInventory,
}

/// System responsible for drawing the Inventory Menu, as well as processing user input while
/// in the Inventory Menu
pub fn inventory_menu(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    println!("Inside the inventory menu!");

    if keys.just_pressed(KeyCode::X) {
        println!("Leaving the inventory menu!");
        state.pop().unwrap();
    }
}
