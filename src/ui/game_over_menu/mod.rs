use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::game_over_menu::systems::{behavior::game_over_menu_interactions, layout::*},
};

mod components;
mod systems;
pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
        app.add_systems(
            Update,
            game_over_menu_interactions.run_if(in_state(AppState::GameOver)),
        );
    }
}
