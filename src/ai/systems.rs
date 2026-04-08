use core::f32;

use bevy::{platform::collections::HashMap, prelude::*};

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
    core::systems::world_to_hash,
    map::{
        resources::ActiveMap,
        utility::{grid_to_world, world_to_grid},
    },
    navigation::flow_field::components::{FlowFieldNavigator, FlowFieldTarget},
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

pub fn flow_field_navigation(
    mut ai_query: Query<
        (&AiController, &Transform, &mut AiMovementIntent),
        With<FlowFieldNavigator>,
    >,
    target_query: Query<&FlowFieldTarget>,
    active_map: Res<ActiveMap>,
) {
    for (controller, ai_transform, mut movement_intent) in ai_query.iter_mut() {
        match controller.black_board.locomotion_intent {
            AiLocomotionIntent::Chase(target) => {
                if let Ok(target_flow_field) = target_query.get(target) {
                    let current_tile = world_to_grid(
                        ai_transform.translation.truncate(),
                        active_map.tileset.tile_size,
                        &active_map.map.bounds,
                    );

                    // this ensures the flow field is calculated first
                    if target_flow_field.last_calculated_tile == None {
                        continue;
                    }

                    // this ensures that the entity is on the grid, it would crash if it was not
                    if current_tile.0 as usize >= active_map.map.tiles[0].len()
                        || current_tile.1 as usize >= active_map.map.tiles.len()
                    {
                        continue;
                    }

                    // get the direction of current tile position
                    //FIXME - currently if the entity is on the target tile it does not know what to do, potential solution is to fallback on direct steering
                    let Some(direction) = target_flow_field.directions[current_tile.1 as usize]
                        [current_tile.0 as usize]
                    else {
                        error!("Could not find direction");
                        continue;
                    };

                    // centering: keep entity on the path centerline for cardinal movement.
                    // scaled to zero for diagonal directions — cross-track correction skews
                    // diagonal angles into cardinals, so we disable it there.
                    // flow field only produces 8 directions, so this is effectively binary.
                    let diagonal_amount = direction.x.abs() * direction.y.abs() * 2.0;
                    let cardinal_scale = 1.0 - diagonal_amount;

                    let tile_center = grid_to_world(
                        current_tile.0,
                        current_tile.1,
                        active_map.tileset.tile_size,
                        &active_map.map.bounds,
                    );
                    let to_center = tile_center - ai_transform.translation.truncate();
                    let cross_track = to_center - to_center.dot(direction) * direction;
                    let centering =
                        cross_track / (active_map.tileset.tile_size / 2.0) * cardinal_scale * 0.5;

                    movement_intent.move_direction = direction + centering;
                }
            }
            _ => movement_intent.move_direction = Vec2::ZERO,
        }
    }
}

const SEPARATION_RADIUS: f32 = 32.0;
const SEPARATION_WEIGHT: f32 = 500.0;
const GRID_CELL_SIZE: f32 = 32.;

pub fn separation_steering(
    mut intent_query: Query<(Entity, &Transform, &mut AiMovementIntent), With<FlowFieldNavigator>>,
    neighbor_query: Query<(Entity, &Transform), With<FlowFieldNavigator>>,
    mut frame_counter: Local<u32>,
) {
    let mut grid: HashMap<(i32, i32), Vec<(Entity, Vec2)>> = HashMap::new();

    for (entity, transform) in neighbor_query.iter() {
        let cell = world_to_hash(transform.translation.truncate(), GRID_CELL_SIZE);
        grid.entry(cell)
            .or_default()
            .push((entity, transform.translation.truncate()));
    }

    *frame_counter = frame_counter.wrapping_add(1);
    for (index, (entity, transform, mut movement_intent)) in intent_query.iter_mut().enumerate() {
        // if index % 2 != (*frame_counter % 2) as usize {
        //     continue;
        // }
        let hash_pos = world_to_hash(transform.translation.truncate(), GRID_CELL_SIZE);
        let mut separation_force = Vec2::ZERO;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let Some(neighbors) = grid.get(&(hash_pos.0 + dx, hash_pos.1 + dy)) else {
                    continue;
                };

                for (neighbor_entity, neighbor_pos) in neighbors.iter() {
                    // cannot separate from self
                    if &entity == neighbor_entity {
                        continue;
                    }
                    let distance = Vec2::distance(transform.translation.truncate(), *neighbor_pos);

                    // if too far away we don't apply any force
                    if distance > SEPARATION_RADIUS {
                        continue;
                    }

                    let away = transform.translation.truncate() - *neighbor_pos;

                    // cubic falloff: nearly nothing at the edge, overwhelmingly strong up close
                    // approaches zero smoothly at the radius boundary so there is no snap/jitter
                    let t = 1.0 - (distance / SEPARATION_RADIUS);
                    separation_force += away.normalize_or_zero() * t * t * t;
                }
            }
        }

        movement_intent.move_direction += separation_force * SEPARATION_WEIGHT;
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
    query_actors: Query<(Entity, &Transform, &Team), (With<Actor>, Without<SeekNearestTarget>)>,
    mut frame_counter: Local<u32>,
) {
    *frame_counter = frame_counter.wrapping_add(1);
    for (index, (mut seeker_controller, seeker_transform, seeker_team)) in
        query_seeking_actor.iter_mut().enumerate()
    {
        if index % 10 != (*frame_counter % 10) as usize {
            continue;
        }

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
