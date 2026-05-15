use core::f32;
use std::sync::Arc;

use bevy::{ecs::batching::BatchingStrategy, prelude::*};

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
        health::components::{Dead, Hitbox, Hurtbox},
        messages::ShootMessage,
    },
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
        (&AiController, &Transform, &mut AiMovementIntent, &Collision),
        (Without<Dead>, With<FlowFieldNavigator>),
    >,
    target_query: Query<(Entity, &FlowFieldTarget)>,
    active_map: Res<ActiveMap>,
) {
    // Pre-compute tile bounds once so each parallel task doesn't recompute them.
    let tile_cols = active_map.map.tiles.first().map_or(0, |row| row.len());
    let tile_rows = active_map.map.tiles.len();

    // Collect flow field targets — typically just 1 (the player).
    // Vec<(Entity, &FlowFieldTarget)> is Send because FlowFieldTarget: Sync.
    let targets: Vec<(Entity, &FlowFieldTarget)> = target_query.iter().collect();

    ai_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(64))
        .for_each(
            |(controller, ai_transform, mut movement_intent, collision)| {
                let offset_position = ai_transform.translation.truncate() + collision.offset;

                let AiLocomotionIntent::Chase(target) = controller.black_board.locomotion_intent
                else {
                    movement_intent.move_direction = Vec2::ZERO;
                    return;
                };

                // Linear scan — fine since there are very few targets (usually just 1)
                let target_flow_field = match targets.iter().find(|(e, _)| *e == target) {
                    Some((_, ff)) => *ff,
                    None => return,
                };

                // this ensures the flow field is calculated first
                if target_flow_field.last_calculated_tile.is_none() {
                    return;
                }

                let current_tile = world_to_grid(
                    offset_position,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                // this ensures that the entity is on the grid, it would crash if it was not
                if current_tile.0 as usize >= tile_cols || current_tile.1 as usize >= tile_rows {
                    return;
                }

                // get the direction of current tile position
                //FIXME - currently if the entity is on the target tile it does not know what to do, potential solution is to fallback on direct steering
                let Some(direction) =
                    target_flow_field.directions[current_tile.1 as usize][current_tile.0 as usize]
                else {
                    // error!("Could not find direction");
                    return;
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
                let to_center = tile_center - offset_position;
                let cross_track = to_center - to_center.dot(direction) * direction;
                let centering =
                    cross_track / (active_map.tileset.tile_size / 2.0) * cardinal_scale * 0.5;

                movement_intent.move_direction = direction + centering;
            },
        );
}

const SEPARATION_RADIUS: f32 = 32.0;
const SEPARATION_RADIUS_SQ: f32 = SEPARATION_RADIUS * SEPARATION_RADIUS;
const SEPARATION_WEIGHT: f32 = 500.0;
const GRID_CELL_SIZE: f32 = 32.;

pub fn separation_steering(
    mut intent_query: Query<
        (Entity, &Transform, &mut AiMovementIntent, &Collision),
        With<FlowFieldNavigator>,
    >,
    neighbor_query: Query<(Entity, &Transform, &Collision), With<FlowFieldNavigator>>,
    active_map: Res<ActiveMap>,
) {
    let west_offset = active_map.map.bounds.west as f32 * active_map.tileset.tile_size;
    let north_offset = active_map.map.bounds.north as f32 * active_map.tileset.tile_size;

    let grid_width = ((active_map.map.bounds.east + active_map.map.bounds.west) as f32
        * active_map.tileset.tile_size
        / GRID_CELL_SIZE)
        .ceil() as usize
        + 2;
    let grid_height = ((active_map.map.bounds.north + active_map.map.bounds.south) as f32
        * active_map.tileset.tile_size
        / GRID_CELL_SIZE)
        .ceil() as usize
        + 2;
    let num_cells = grid_width * grid_height;

    // Collect entity + position pairs once. Consistent y-negation converts world-space
    // (y-up) to grid-space (y-down) the same way in both the build and lookup passes.
    let pairs: Vec<(Entity, Vec2)> = neighbor_query
        .iter()
        .map(|(e, t, c)| (e, t.translation.truncate() + c.offset))
        .collect();

    let world_to_cell = |pos: Vec2| -> (i32, i32) {
        let cx = ((pos.x + west_offset) / GRID_CELL_SIZE).floor() as i32;
        let cy = ((-pos.y + north_offset) / GRID_CELL_SIZE).floor() as i32;
        (cx, cy)
    };

    // --- counting sort spatial hash ---
    // Pass 1: count entities per cell.
    let mut counts = vec![0u32; num_cells];
    for &(_, pos) in &pairs {
        let (cx, cy) = world_to_cell(pos);
        if cx >= 0 && cy >= 0 && (cx as usize) < grid_width && (cy as usize) < grid_height {
            counts[cy as usize * grid_width + cx as usize] += 1;
        }
    }

    // Pass 2: exclusive prefix sum → start offset for each cell.
    let mut offsets = vec![0u32; num_cells + 1];
    for i in 0..num_cells {
        offsets[i + 1] = offsets[i] + counts[i];
    }

    // Pass 3: fill flat index array.
    let total = offsets[num_cells] as usize;
    let mut flat = vec![0u32; total];
    let mut cursors = offsets[..num_cells].to_vec();
    for (i, &(_, pos)) in pairs.iter().enumerate() {
        let (cx, cy) = world_to_cell(pos);
        if cx >= 0 && cy >= 0 && (cx as usize) < grid_width && (cy as usize) < grid_height {
            let cell = cy as usize * grid_width + cx as usize;
            flat[cursors[cell] as usize] = i as u32;
            cursors[cell] += 1;
        }
    }

    let grid_data = Arc::new((flat, offsets, pairs));

    intent_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(50000))
        .for_each(|(entity, transform, mut movement_intent, collision)| {
            let (flat, offsets, pairs) = &*grid_data;
            let pos = transform.translation.truncate() + collision.offset;
            let (cx, cy) = (
                ((pos.x + west_offset) / GRID_CELL_SIZE).floor() as i32,
                ((-pos.y + north_offset) / GRID_CELL_SIZE).floor() as i32,
            );

            let desired = movement_intent.move_direction;
            let mut movement_scale = 1.0f32;

            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx < 0
                        || ny < 0
                        || (nx as usize) >= grid_width
                        || (ny as usize) >= grid_height
                    {
                        continue;
                    }
                    let cell = ny as usize * grid_width + nx as usize;
                    let start = offsets[cell] as usize;
                    let end = offsets[cell + 1] as usize;

                    for &idx in &flat[start..end] {
                        let (neighbor_entity, neighbor_pos) = pairs[idx as usize];
                        if neighbor_entity == entity {
                            continue;
                        }

                        let diff = pos - neighbor_pos;
                        let dist_sq = diff.length_squared();
                        // cheap sq check avoids sqrt for most neighbors
                        if dist_sq >= SEPARATION_RADIUS_SQ || dist_sq == 0.0 {
                            continue;
                        }
                        let distance = dist_sq.sqrt();
                        // cubic falloff: nearly nothing at the edge, overwhelmingly strong up close
                        let neighbor_dir = -diff / distance; // direction toward neighbor
                        let dot = desired.dot(neighbor_dir);
                        if dot > 0.0 {
                            let blocking = dot * (1.0 - distance / SEPARATION_RADIUS);
                            movement_scale -= blocking;
                        }
                    }
                }
            }

            movement_intent.move_direction = desired * movement_scale.clamp(0.0, 1.0);
        });
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
                        just_pressed: false,
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn seek_nearest_target(
    mut query_seeking_actor: Query<
        (&mut AiController, &Transform, &Team),
        (With<SeekNearestTarget>, Without<Dead>),
    >,
    query_actors: Query<
        (Entity, &Transform, &Team),
        (With<Actor>, (Without<SeekNearestTarget>, Without<Dead>)),
    >,
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
    mut ai_query: Query<
        (&AiController, &mut MeleeIntent, &Transform, &Hitbox),
        (With<Actor>, Without<Dead>),
    >,
    actor_query: Query<(&Transform, &Hurtbox), (With<Actor>, Without<Dead>)>,
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
