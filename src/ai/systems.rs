use bevy::prelude::*;

use crate::{
    actor::{components::Actor, locomotion::components::AiMovementIntent},
    ai::{
        components::{AiController, AiIntent},
        vision::components::Vision,
    },
};

pub fn ai_targeting_system(
    mut ai_query: Query<(Entity, &Transform, &mut AiController)>,
    actor_query: Query<(Entity, &Transform), With<Actor>>,
) {
    for (ai_entity, ai_transform, mut ai_controller) in ai_query.iter_mut() {
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
        // info!("current target of {:?} is {:?}", ai_entity, closest_entity)
    }
}

pub fn ai_movement_system(
    mut ai_query: Query<(&AiController, &Transform, &mut AiMovementIntent, &Vision)>,
    actor_query: Query<&Transform, With<Actor>>,
) {
    for (controller, ai_transform, mut movement_intent, ai_vision) in ai_query.iter_mut() {
        match controller.black_board.intent {
            AiIntent::Chase(target) => {
                if let Ok(target_transform) = actor_query.get(target) {
                    let distance = Vec2::distance(
                        target_transform.translation.truncate(),
                        ai_transform.translation.truncate(),
                    );

                    let keep_distance_at = ai_vision.range / 2.;

                    //TEMPORARY - this is not good code and should be fixed
                    //FIXME - this is a mess

                    if distance > keep_distance_at {
                        // go towards target
                        let direction = (target_transform.translation.truncate()
                            - ai_transform.translation.truncate())
                        .normalize();
                        movement_intent.move_direction = direction;
                    } else if distance == keep_distance_at {
                        // don't more
                        movement_intent.move_direction = Vec2::default();
                    } else {
                        // go in opposite direction
                        let direction = (ai_transform.translation.truncate()
                            - target_transform.translation.truncate())
                        .normalize();
                        movement_intent.move_direction = direction;
                    }
                }
            }
            _ => movement_intent.move_direction = Vec2::ZERO,
        }
    }
}
