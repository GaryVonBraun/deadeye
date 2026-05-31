use bevy::prelude::*;

use crate::{
    ai::{AiSet, vision::systems::*},
    core::states::SimulationState,
};

pub mod components;
mod systems;
pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                collect_nearby_actors,
                compute_visible_actors,
                get_nearest_visible_hostile_system,
            )
                .chain()
                .in_set(AiSet::Perception)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
