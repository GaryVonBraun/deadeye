use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::victory_menu::systems::{behavior::victory_menu_interactions, layout::*},
};

mod components;
mod systems;
pub struct VictoryMenuPlugin;

impl Plugin for VictoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Victory), spawn_victory_menu);
        app.add_systems(OnExit(AppState::Victory), despawn_victory_menu);
        app.add_systems(
            Update,
            victory_menu_interactions.run_if(in_state(AppState::Victory)),
        );
    }
}
