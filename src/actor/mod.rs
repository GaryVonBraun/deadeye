use bevy::prelude::*;

use crate::actor::{locomotion::LocomotionPlugin, messages::*, system::spawn_actor_handler};

mod appearance;
mod bundles;
pub mod components;
pub mod locomotion;
pub mod messages;
mod system;
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnActorMessage>();
        app.add_plugins(LocomotionPlugin);
        app.add_systems(
            Update,
            spawn_actor_handler.run_if(on_message::<SpawnActorMessage>),
        );
    }
}
