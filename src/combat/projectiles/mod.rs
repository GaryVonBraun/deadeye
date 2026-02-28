use bevy::prelude::*;

use crate::{combat::projectiles::systems::*, core::states::SimulationState};

pub mod bundles;
pub mod component;
mod systems;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_projectiles, projectile_collision)
                .chain()
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
