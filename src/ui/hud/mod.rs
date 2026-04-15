use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::hud::systems::{layout::*, sync::update_health_bar},
};

mod components;
mod systems;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hud);
        app.add_systems(OnExit(AppState::InGame), despawn_hud);
        app.add_systems(Update, update_health_bar.run_if(in_state(AppState::InGame)));
    }
}
