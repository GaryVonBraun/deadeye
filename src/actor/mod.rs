use bevy::prelude::*;

use crate::{
    actor::{
        humanoid::{HumanoidPlugin, factories::*},
        locomotion::LocomotionPlugin,
    },
    core::states::AppState,
};

mod appearance;
mod bundles;
pub mod components;
mod humanoid;
pub mod locomotion;
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HumanoidPlugin, LocomotionPlugin));
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_player_humanoid, spawn_training_dummy, spawn_test_ai, spawn_test_ai2),
        );
    }
}
