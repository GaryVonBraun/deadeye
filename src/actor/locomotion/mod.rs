use bevy::prelude::*;

use crate::{actor::locomotion::systems::*, core::states::SimulationState};

pub mod components;
mod systems;
pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (resolve_movement, integrate_movement)
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
