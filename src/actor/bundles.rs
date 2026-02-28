use bevy::prelude::*;

use crate::{
    actor::components::*, combat::health::components::Health, core::components::GameEntity,
    simulation::collision::components::Collision,
};

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
    pub health: Health,
    pub collision: Collision,
    pub game_entity: GameEntity,
}

impl CoreActorBundle {
    pub fn default_with_translation(translation: Vec3) -> Self {
        CoreActorBundle {
            actor: Actor,
            transform: Transform::from_translation(translation),
            health: Health::default(),
            collision: Collision::from_radius(16.),
            game_entity: GameEntity,
        }
    }
}
