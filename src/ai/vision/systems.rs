use bevy::prelude::*;

use crate::{
    actor::components::{Actor, Team},
    ai::{components::AiController, vision::components::Vision},
    collision::{components::Collision, utility::swept_collision},
    combat::health::components::Dead,
    props::components::Prop,
};

pub fn nearby_detection_system(
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
            }
        }

        ai_controller.black_board.nearby_actors = visible_actors;
    }
}

pub fn get_visible_actors(
    mut ai_query: Query<(&Transform, &mut AiController, &Team), With<Vision>>,
    actor_query: Query<(Entity, &Transform, &Team), (With<Actor>, Without<Dead>)>,
    prop_query: Query<(&Collision, &Transform), With<Prop>>,
) {
    for (ai_transform, mut ai_controller, team) in ai_query.iter_mut() {
        let mut visible_actors: Vec<Entity> = vec![];
        for entity in ai_controller.black_board.nearby_actors.iter() {
            let Ok((nearby_entity, nearby_transform, nearby_team)) = actor_query.get(*entity)
            else {
                continue;
            };

            let mut vision_blocked = false;

            for (prop_collision, prop_transform) in prop_query.iter() {
                if swept_collision(
                    ai_transform.translation.truncate(),
                    nearby_transform.translation.truncate(),
                    prop_transform.translation.truncate(),
                    prop_collision,
                )
                .is_some()
                {
                    vision_blocked = true;
                    break;
                };
            }

            if !vision_blocked {
                visible_actors.push(nearby_entity);
            }
        }
        ai_controller.black_board.visible_actors = visible_actors
    }
}
