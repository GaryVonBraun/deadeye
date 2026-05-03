use bevy::prelude::*;

use crate::{
    actor::{components::*, system::ActorDefinition},
    collision::components::{Collision, CollisionShape},
    combat::health::components::{Health, Hitbox, Hurtbox},
    core::components::GameEntity,
};

#[derive(Bundle)]
pub struct CoreActorBundle {
    pub transform: Transform,
    pub actor: Actor,
    pub health: Health,
    pub hurtbox: Hurtbox,
    pub hitbox: Hitbox,
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
            hurtbox: Hurtbox {
                //TEMPORARY - for now its a hardcoded size, will probably be properly implement when actual sprites are done
                shape: CollisionShape::Rect {
                    width: 10.,
                    height: 16.,
                },
                offset: Vec2 { x: 0., y: 0. },
            },
            hitbox: Hitbox {
                //TEMPORARY - for now its a hardcoded size, will probably be properly implement when actual sprites are done
                shape: CollisionShape::Circle {
                    radius: 8. + actor.melee_range,
                },
                offset: Vec2::default(),
            },
            collision: Collision::from_offset_radius(
                Vec2 { x: 0., y: -8. },
                actor.collision_radius.clone(),
            ),
            game_entity: GameEntity,
            team: actor.team.clone(),
        }
    }
}
