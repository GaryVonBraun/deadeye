use bevy::prelude::*;

use crate::actor::locomotion::systems::apply_locomotion;

pub mod components;
mod systems;
pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_locomotion);
    }
}
