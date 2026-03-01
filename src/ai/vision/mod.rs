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
            (vision_detection_system, vision_debug_system)
                .in_set(AiSet::Perception)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
