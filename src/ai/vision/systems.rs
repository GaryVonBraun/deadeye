use bevy::prelude::*;

use crate::{
    actor::components::Actor,
    ai::{components::AiController, vision::components::Vision},
};

pub fn vision_detection_system(
    mut ai_query: Query<(Entity, &Transform, &Vision, &mut AiController)>,
    actor_query: Query<(Entity, &Transform), With<Actor>>,
) {
    for (ai_entity, ai_transform, ai_vision, mut ai_controller) in ai_query.iter_mut() {

        let mut visible_actors: Vec<Entity> = [].to_vec();

        for (actor_entity, actor_transform) in actor_query.iter() {
            if ai_entity == actor_entity {
                continue;
            }

            if Vec2::distance(
                ai_transform.translation.truncate(),
                actor_transform.translation.truncate(),
            ) < ai_vision.range
            {
                visible_actors.push(actor_entity);
                // info!(
                //     "actor: {:?} in in vision of actor {:?}",
                //     ai_entity, actor_entity
                // );
            }
        }

        ai_controller.black_board.visible_actors = visible_actors;
    }
}
pub fn vision_debug_system(
    ai_query: Query<(&Transform, &Vision)>,
    mut gizmos: Gizmos,
) {
    for (transform, vision) in ai_query.iter() {
        gizmos.circle_2d(
            transform.translation.truncate(),
            vision.range,
            Color::srgba(1.0, 1.0, 0.0, 0.5),
        );
    }
}