use bevy::prelude::*;

use crate::{core::states::SimulationState, debug::systems::debug_movement_controller};

mod systems;
pub mod components;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_movement_controller.run_if(in_state(SimulationState::Running)));
    }
}