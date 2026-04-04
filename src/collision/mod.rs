use bevy::prelude::*;

use crate::{
    collision::{sets::PhysicsSet, systems::actor_obstruction_collision},
    core::states::SimulationState,
};

pub mod components;
pub mod sets;
pub mod systems;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (PhysicsSet::Movement, PhysicsSet::CollisionResolution).chain(),
        );
        app.add_systems(
            Update,
            actor_obstruction_collision
                .in_set(PhysicsSet::CollisionResolution)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
