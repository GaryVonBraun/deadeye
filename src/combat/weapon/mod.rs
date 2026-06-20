use bevy::prelude::*;

use crate::{combat::weapon::systems::*, core::states::SimulationState};

mod bundles;
pub mod components;
pub mod factories;
mod io;
mod systems;
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                weapon_runtime_system,
                shoot_weapon,
                rotate_weapons,
                reload_weapon,
            )
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
