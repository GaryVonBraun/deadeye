use bevy::prelude::*;

use crate::{
    core::states::{AppState, SimulationState},
    player::systems::*,
};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement_controller).run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            (player_aim_system, player_combat_input)
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            camera_follow_player.run_if(in_state(AppState::InGame)),
        );
    }
}
