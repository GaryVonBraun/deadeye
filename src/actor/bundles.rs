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

impl CoreActorBundle {
    pub fn default_with_translation(translation: Vec3) -> Self {
        CoreActorBundle {
                actor: Actor,
                transform: Transform::from_translation(translation),
                health: Health::default(),
                collision: Collision::from_radius(16.),
            }
    }
}
