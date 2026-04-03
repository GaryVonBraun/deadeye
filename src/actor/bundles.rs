use bevy::prelude::*;

use crate::{
    actor::{components::*, system::ActorDefinition},
    collision::components::{Collision, CollisionShape},
    combat::health::components::{Health, HurtBox},
    core::components::GameEntity,
};

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
    pub health: Health,
    pub hurt_box: HurtBox,
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
            hurt_box: HurtBox {
                //TEMPORARY - for now its a hardcoded size, will probably be properly implement when actual sprites are done
                shape: CollisionShape::Rect {
                    width: 32.,
                    height: 32.,
                },
                offset: Vec2::default(),
            },
            collision: Collision::from_radius(actor.collision_radius.clone()),
            game_entity: GameEntity,
            team: actor.team.clone(),
        }
    }
}
