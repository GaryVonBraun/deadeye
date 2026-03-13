use bevy::prelude::*;

use crate::ui::{
    common::button::ui_button_interaction, main_menu::MainMenuPlugin, map::MapUiPlugin,
};

mod common;
mod main_menu;
pub mod map;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, MapUiPlugin));
        app.add_systems(Update, ui_button_interaction);
    }
}
