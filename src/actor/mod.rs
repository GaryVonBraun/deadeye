use bevy::prelude::*;

use crate::{actor::{humanoid::{HumanoidPlugin, factories::*}, locomotion::LocomotionPlugin}, core::states::AppState};


mod bundles;
mod humanoid;
pub mod locomotion;
mod appearance;
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HumanoidPlugin, LocomotionPlugin));
        app.add_systems(OnEnter(AppState::MainMenu), spawn_player_humanoid);
    }
}