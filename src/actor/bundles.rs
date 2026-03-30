use bevy::prelude::*;

use crate::{
    actor::{components::*, system::ActorDefinition},
    combat::health::components::Health,
    core::components::GameEntity,
    simulation::collision::components::Collision,
};

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
    pub health: Health,
    pub collision: Collision,
    pub game_entity: GameEntity,
    pub team: Team,
}

impl CoreActorBundle {
    pub fn from_actor_with_position(translation: Vec3, actor: &ActorDefinition) -> Self {
        CoreActorBundle {
            actor: Actor,
            transform: Transform::from_translation(translation),
            health: Health::default(),
            collision: Collision::from_radius(actor.collision_radius.clone()),
            game_entity: GameEntity,
            team: actor.team.clone(),
        }
    }
}
