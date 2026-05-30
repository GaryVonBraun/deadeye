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
            (nearby_detection_system, get_visible_actors)
                .chain()
                .in_set(AiSet::Perception)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
