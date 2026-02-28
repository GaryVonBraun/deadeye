use bevy::prelude::*;

use crate::{core::states::AppState, ui::main_menu::systems::{behavior::main_menu_interactions, layout::*}};

mod systems;
mod components;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
        app.add_systems(Update, main_menu_interactions);
    }
}