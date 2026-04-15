use bevy::prelude::*;

use crate::ui::{
    common::button::ui_button_interaction, game_over_menu::GameOverMenuPlugin,
    main_menu::MainMenuPlugin, missions_menu::MissionDevMenuPlugin,
    victory_menu::VictoryMenuPlugin,
};

mod common;
mod game_over_menu;
mod main_menu;
pub mod missions_menu;
mod victory_menu;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            MissionDevMenuPlugin,
            GameOverMenuPlugin,
            VictoryMenuPlugin,
        ));
        app.add_systems(Update, ui_button_interaction);
    }
}
