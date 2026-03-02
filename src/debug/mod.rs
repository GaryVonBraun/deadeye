use bevy::prelude::*;

use crate::{core::states::SimulationState, debug::systems::*};

pub mod components;
mod systems;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_movement_controller,
                debug_vision_gizmo,
                debug_visible_entities_gizmo,
                debug_target_entity_gizmo,
            )
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
