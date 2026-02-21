use bevy::prelude::*;

use crate::actor::components::Actor;

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
}
