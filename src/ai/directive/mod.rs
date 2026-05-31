use bevy::prelude::*;

use crate::{ai::directive::systems::*, core::states::SimulationState};

pub mod components;
mod systems;

pub struct DirectivePlugin;

impl Plugin for DirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_follow_player_directive, update_blackboard_directive)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
