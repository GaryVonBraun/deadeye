use bevy::prelude::*;

use crate::{
    actor::{
        components::{Actor, Team},
        teams::{TeamStanding, get_standing},
    },
    ai::{components::AiController, vision::components::Vision},
    collision::{components::Collision, utility::swept_collision},
    combat::health::components::Dead,
    props::components::Prop,
};

pub fn collect_nearby_actors(
    mut ai_query: Query<(Entity, &Transform, &Vision, &mut AiController)>,
    actor_query: Query<(Entity, &Transform), With<Actor>>,
) {
    for (ai_entity, ai_transform, ai_vision, mut ai_controller) in ai_query.iter_mut() {
        let mut visible_actors: Vec<Entity> = [].to_vec();

        for (actor_entity, actor_transform) in actor_query.iter() {
            // skip self
            if ai_entity == actor_entity {
                continue;
            }

            // if entity is within vision range it collects
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

pub fn compute_visible_actors(
    mut ai_query: Query<(&Transform, &mut AiController), With<Vision>>,
    actor_query: Query<(Entity, &Transform), (With<Actor>, Without<Dead>)>,
    prop_query: Query<(&Collision, &Transform), With<Prop>>,
) {
    for (ai_transform, mut ai_controller) in ai_query.iter_mut() {
        let mut visible_actors: Vec<Entity> = vec![];
        for entity in ai_controller.black_board.nearby_actors.iter() {
            let Ok((nearby_entity, nearby_transform)) = actor_query.get(*entity) else {
                continue;
            };

            let mut vision_blocked = false;

            // perform "raycast" like collision check to see if prop intersects between 2 actors
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

pub fn get_nearest_visible_hostile_system(
    mut ai_query: Query<(&Transform, &mut AiController, &Team), With<Vision>>,
    actor_query: Query<(Entity, &Transform, &Team), (With<Actor>, Without<Dead>)>,
) {
    for (ai_transform, mut ai_controller, team) in ai_query.iter_mut() {
        let mut closest_distance = f32::MAX;
        let mut closest_entity: Option<Entity> = None;

        if ai_controller.black_board.visible_actors.is_empty() {
            ai_controller.black_board.nearest_visible_hostile = None;
            continue;
        }

        for visible_entity in ai_controller.black_board.visible_actors.iter() {
            let Ok((actor_entity, actor_transform, target_team)) = actor_query.get(*visible_entity)
            else {
                continue;
            };

            if !matches!(get_standing(&team.0, &target_team.0), TeamStanding::Hostile) {
                continue;
            }

            let distance = Vec2::distance(
                ai_transform.translation.truncate(),
                actor_transform.translation.truncate(),
            );

            if distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(actor_entity);
            }
        }
        ai_controller.black_board.nearest_visible_hostile = closest_entity;
    }
}
