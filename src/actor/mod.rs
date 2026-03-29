use bevy::prelude::*;

use crate::{
    actor::{
        humanoid::{HumanoidPlugin, factories::*},
        locomotion::LocomotionPlugin,
        messages::SpawnPlayerMessage,
    },
    core::states::AppState,
};

mod appearance;
mod bundles;
pub mod components;
pub mod humanoid;
pub mod locomotion;
pub mod messages;
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnPlayerMessage>();
        app.add_plugins((HumanoidPlugin, LocomotionPlugin));
        app.add_systems(OnEnter(AppState::InGame), (spawn_multiple_test_ai));
        app.add_systems(
            Update,
            spawn_player_humanoid.run_if(on_message::<SpawnPlayerMessage>),
        );
    }
}
