use bevy::prelude::*;

use crate::ui::{
    common::button::ui_button_interaction, main_menu::MainMenuPlugin,
    missions_menu::MissionDevMenuPlugin,
};

mod common;
mod main_menu;
pub mod missions_menu;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, MissionDevMenuPlugin));
        app.add_systems(Update, ui_button_interaction);
    }
}
