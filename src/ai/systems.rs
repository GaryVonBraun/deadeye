use core::f32;

use bevy::prelude::*;

use crate::{
    actor::{
        components::{Actor, Team},
        teams::{TeamStanding, get_standing},
    },
    ai::{
        components::{
            AiActionIntent, AiController, AiLocomotionIntent, AiMovementIntent, SeekNearestTarget,
        },
        vision::components::Vision,
    },
    collision::{components::Collision, systems::check_collision},
    combat::{
        components::{MeleeIntent, MeleeState, ShootingIntent},
        health::components::{Hitbox, Hurtbox},
        messages::ShootMessage,
    },
};

pub fn vision_targeting_system(
    mut ai_query: Query<(&Transform, &mut AiController), With<Vision>>,
    actor_query: Query<(Entity, &Transform), With<Actor>>,
) {
    for (ai_transform, mut ai_controller) in ai_query.iter_mut() {
        let mut closest_distance = f32::MAX;
        let mut closest_entity: Option<Entity> = None;

        for visible_entity in ai_controller.black_board.visible_actors.iter() {
            let Ok((actor_entity, actor_transform)) = actor_query.get(*visible_entity) else {
                continue;
            };
            let distance = Vec2::distance(
                ai_transform.translation.truncate(),
                actor_transform.translation.truncate(),
            );

            if distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(actor_entity);
            }
        }
        ai_controller.black_board.current_target = closest_entity;
    }
}

pub fn ai_movement_system(
    mut ai_query: Query<(&AiController, &Transform, &mut AiMovementIntent, &Collision)>,
    actor_query: Query<(&Transform, &Collision), With<Actor>>,
) {
    for (controller, ai_transform, mut movement_intent, collision) in ai_query.iter_mut() {
        match controller.black_board.locomotion_intent {
            AiLocomotionIntent::Chase(target) => {
                if let Ok((target_transform, target_collision)) = actor_query.get(target) {
                    let distance = Vec2::distance(
                        target_transform.translation.truncate(),
                        ai_transform.translation.truncate(),
                    );

                    //TEMPORARY - this is not good code and should be fixed
                    //FIXME - this is a mess

                    // if distance > keep_distance_at {
                    //     // go towards target
                    //     let direction = (target_transform.translation.truncate()
                    //         - ai_transform.translation.truncate())
                    //     .normalize();
                    //     movement_intent.move_direction = direction;
                    // } else if distance == keep_distance_at {
                    //     // don't move
                    //     movement_intent.move_direction = Vec2::default();
                    // } else {
                    //     // go in opposite direction

                    //     let direction = (ai_transform.translation.truncate()
                    //         - target_transform.translation.truncate())
                    //     .normalize();
                    // }

                    let direction = (target_transform.translation.truncate()
                        - ai_transform.translation.truncate())
                    .normalize();
                    movement_intent.move_direction = direction;
                }
            }
            _ => movement_intent.move_direction = Vec2::ZERO,
        }
    }
}

pub fn ai_shooting_system(
    mut ai_query: Query<
        (Entity, &AiController, &Transform, &mut ShootingIntent),
        With<AiController>,
    >,
    actor_query: Query<&Transform, With<Actor>>,
    mut messages: MessageWriter<ShootMessage>,
) {
    for (ai_entity, controller, ai_transform, mut ai_shooting_intent) in ai_query.iter_mut() {
        match controller.black_board.action_intent {
            AiActionIntent::Shoot(target) => {
                if let Ok(target_transform) = actor_query.get(target) {
                    let direction = (target_transform.translation.truncate()
                        - ai_transform.translation.truncate())
                    .normalize();

                    ai_shooting_intent.direction = direction;

                    messages.write(ShootMessage {
                        owner: ai_entity,
                        direction: ai_shooting_intent.direction,
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn seek_nearest_target(
    mut query_seeking_actor: Query<(&mut AiController, &Transform, &Team), With<SeekNearestTarget>>,
    query_actors: Query<(Entity, &Transform, &Team), With<Actor>>,
) {
    for (mut seeker_controller, seeker_transform, seeker_team) in query_seeking_actor.iter_mut() {
        let mut nearest_entity: Option<Entity> = None;
        let mut nearest_distance: f32 = f32::MAX;

        for (actor_entity, actor_transform, actor_team) in query_actors.iter() {
            if get_standing(&seeker_team.0, &actor_team.0) != TeamStanding::Hostile {
                continue;
            }

            if nearest_entity == None {
                nearest_entity = Some(actor_entity);
                nearest_distance = Vec2::distance(
                    seeker_transform.translation.truncate(),
                    actor_transform.translation.truncate(),
                );
                continue;
            }

            let actor_distance = Vec2::distance(
                seeker_transform.translation.truncate(),
                actor_transform.translation.truncate(),
            );

            if actor_distance < nearest_distance {
                nearest_distance = actor_distance;
                nearest_entity = Some(actor_entity);
            }
        }

        seeker_controller.black_board.current_target = nearest_entity;
    }
}

pub fn ai_melee_system(
    mut ai_query: Query<(&AiController, &mut MeleeIntent, &Transform, &Hitbox), With<Actor>>,
    actor_query: Query<(&Transform, &Hurtbox), With<Actor>>,
) {
    for (controller, mut melee_intent, transform, hitbox) in ai_query.iter_mut() {
        match controller.black_board.action_intent {
            AiActionIntent::Melee(entity) => {
                if melee_intent.melee_state != MeleeState::Ready {
                    continue;
                }

                let Ok((target_transform, target_hurtbox)) = actor_query.get(entity) else {
                    error!("Failed to find target entity");
                    return;
                };

                if check_collision(
                    transform.translation.truncate(),
                    hitbox,
                    target_transform.translation.truncate(),
                    target_hurtbox,
                ) {
                    melee_intent.melee_state = MeleeState::AttackDelay(melee_intent.delay);
                    melee_intent.target = Some(entity);
                }
            }
            _ => {}
        }
    }
}
