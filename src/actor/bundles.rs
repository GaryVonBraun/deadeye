use bevy::prelude::*;

use crate::{
    actor::components::*, combat::health::components::Health,
    simulation::collision::components::Collision,
};

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
    pub health: Health,
    pub collision: Collision,
}
