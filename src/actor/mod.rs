use bevy::prelude::*;

use crate::{actor::{humanoid::{HumanoidPlugin, factories::spawn_basic_humanoid}, locomotion::LocomotionPlugin}, core::states::AppState};


mod bundles;
mod humanoid;
pub mod locomotion;
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HumanoidPlugin, LocomotionPlugin));
        app.add_systems(OnEnter(AppState::MainMenu), spawn_basic_humanoid);
    }
}