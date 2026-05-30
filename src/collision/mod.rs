use bevy::prelude::*;

use crate::{
    collision::{sets::PhysicsSet, systems::*},
    core::states::SimulationState,
    map::resources::ActiveMap,
};

pub mod components;
pub mod sets;
mod systems;
pub mod utility;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (PhysicsSet::Movement, PhysicsSet::CollisionResolution).chain(),
        );
        app.add_systems(
            Update,
            (
                actor_obstruction_collision,
                actor_vs_actor_collision.run_if(resource_exists::<ActiveMap>),
            )
                .chain()
                .in_set(PhysicsSet::CollisionResolution)
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
