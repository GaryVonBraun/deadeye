use bevy::prelude::*;

use crate::{
    core::states::SimulationState,
    navigation::{flow_field::systems::*, resources::NavGrid},
};

pub mod components;
mod systems;
pub struct FlowFieldPlugin;

impl Plugin for FlowFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                build_flow_field,
                // flow_field_gizmos
            )
                .run_if(in_state(SimulationState::Running).and(resource_exists::<NavGrid>)),
        );
    }
}
