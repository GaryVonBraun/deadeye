use bevy::prelude::*;

use crate::{
    actor::locomotion::systems::*, collision::sets::PhysicsSet, core::states::SimulationState,
};

pub mod components;
mod systems;
pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (resolve_movement, integrate_movement)
                .in_set(PhysicsSet::Movement)
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
