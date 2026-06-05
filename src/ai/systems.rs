use core::f32;
use std::sync::Arc;

use bevy::{ecs::batching::BatchingStrategy, prelude::*};

use crate::{
    actor::{
        components::{Actor, Team},
        teams::{TeamStanding, get_standing},
    },
    ai::components::{
        AiActionIntent, AiController, AiLocomotionIntent, AiMovementIntent, AmmoAmount,
        SeekNearestHostile,
    },
    collision::{components::Collision, utility::check_collision},
    combat::{
        components::{EquippedWeapon, MeleeIntent, MeleeState, ShootingIntent},
        health::components::{Dead, Hitbox, Hurtbox},
        messages::{ReloadMessage, ShootMessage},
        weapon::components::{Weapon, WeaponRuntime},
    },
    map::{
        resources::ActiveMap,
        utility::{grid_to_world, world_to_grid},
    },
    navigation::{
        astar::components::AStarPath, components::NavigationTargetTile,
        flow_field::components::FlowFieldNavigator,
    },
};

pub fn ai_movement_system(
    mut ai_query: Query<(&AiController, &Transform, &mut AiMovementIntent, &Collision)>,
    actor_query: Query<(&Transform, &Collision), With<Actor>>,
) {
    for (controller, ai_transform, mut movement_intent, collision) in ai_query.iter_mut() {
        match controller.intent.locomotion {
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

pub fn target_navigation(
    mut ai_query: Query<
        (
            &Transform,
            &mut AiMovementIntent,
            &Collision,
            &NavigationTargetTile,
        ),
        Without<Dead>,
    >,
    active_map: Res<ActiveMap>,
) {
    // Pre-compute tile bounds once so each parallel task doesn't recompute them.
    let tile_cols = active_map.map.tiles.first().map_or(0, |row| row.len());
    let tile_rows = active_map.map.tiles.len();

    ai_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(100000))
        .for_each(
            |(ai_transform, mut movement_intent, collision, navigation_target)| {
                let Some(target) = navigation_target.value else {
                    movement_intent.move_direction = Vec2::ZERO;
                    return;
                };

                let offset_position = ai_transform.translation.truncate() + collision.offset;

                let current_tile = world_to_grid(
                    offset_position,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                // this ensures that the entity is on the grid, it would crash if it was not
                if current_tile.x as usize >= tile_cols || current_tile.y as usize >= tile_rows {
                    return;
                }

                let tile_position = grid_to_world(
                    target.x,
                    target.y,
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                // get the direction to current tile target
                let direction = (tile_position - offset_position).normalize_or_zero();

                // centering: keep entity on the path centerline for cardinal movement.
                // scaled to zero for diagonal directions — cross-track correction skews
                // diagonal angles into cardinals, so we disable it there.
                // flow field only produces 8 directions, so this is effectively binary.
                let diagonal_amount = direction.x.abs() * direction.y.abs() * 2.0;
                let cardinal_scale = 1.0 - diagonal_amount;

                let tile_center = grid_to_world(
                    current_tile.x,
                    current_tile.y,
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

//Deprecated -
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
    mut ai_query: Query<(Entity, &AiController, &mut ShootingIntent), With<AiController>>,
    actor_query: Query<&Transform, With<Actor>>,
    mut messages: MessageWriter<ShootMessage>,
) {
    for (ai_entity, controller, mut ai_shooting_intent) in ai_query.iter_mut() {
        match controller.intent.action {
            AiActionIntent::Shoot(target) => {
                if let Ok(target_transform) = actor_query.get(target) {
                    ai_shooting_intent.target_position = target_transform.translation.truncate();

                    messages.write(ShootMessage {
                        owner: ai_entity,
                        target_position: ai_shooting_intent.target_position,
                        just_pressed: false,
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn ai_reloading_system(
    mut ai_query: Query<(Entity, &AiController), With<AiController>>,
    mut messages: MessageWriter<ReloadMessage>,
) {
    for (entity, controller) in ai_query.iter_mut() {
        match controller.intent.action {
            AiActionIntent::Reload => {
                messages.write(ReloadMessage { entity });
            }
            _ => {}
        }
    }
}

pub fn ai_weapon_awareness_system(
    mut ai_query: Query<(&mut AiController, &EquippedWeapon), With<AiController>>,
    weapon_query: Query<(&Weapon, &WeaponRuntime), With<Weapon>>,
) {
    for (mut controller, equipped_weapon) in ai_query.iter_mut() {
        let Ok((weapon, runtime)) = weapon_query.get(equipped_weapon.entity) else {
            continue;
        };

        let reserve = match runtime.ammo {
            0 => AmmoAmount::Empty,
            a if a <= weapon.magazine_size / 3 => AmmoAmount::Low,
            a if a <= weapon.magazine_size / 3 * 2 => AmmoAmount::Medium,
            _ => AmmoAmount::Full,
        };

        controller.black_board.weapon_info.magazine_ammo = reserve;
    }
}

pub fn seek_nearest_hostile(
    mut query_seeking_actor: Query<
        (&mut AiController, &Transform, &Team),
        (With<SeekNearestHostile>, Without<Dead>),
    >,
    query_actors: Query<
        (Entity, &Transform, &Team),
        (With<Actor>, (Without<SeekNearestHostile>, Without<Dead>)),
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

        seeker_controller.black_board.nearest_hostile = nearest_entity;
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
        match controller.intent.action {
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

pub fn follow_target_actor(
    mut ai_query: Query<(
        &Transform,
        &AiController,
        &mut NavigationTargetTile,
        &mut AStarPath,
    )>,
    actor_query: Query<&Transform, With<Actor>>,
    active_map: Res<ActiveMap>,
) {
    for (transform, controller, mut target_tile, mut astar) in ai_query.iter_mut() {
        match controller.intent.locomotion {
            AiLocomotionIntent::Follow { target, distance } => {
                let Ok(actor_transform) = actor_query.get(target) else {
                    warn!("Could not find actor {:?} needed for following", target);
                    continue;
                };

                let actor_tile_position = world_to_grid(
                    actor_transform.translation.truncate(),
                    active_map.tileset.tile_size,
                    &active_map.map.bounds,
                );

                //FIXME - Following around corners the ai can be a little funky
                // direct targeting
                if controller.black_board.visible_actors.contains(&target) {
                    if let Some(commit_index) = astar.commit_until_index {
                        if astar.current_index < commit_index {
                            continue;
                        }

                        astar.commit_until_index = None;
                    }

                    if Vec2::distance(
                        transform.translation.truncate(),
                        actor_transform.translation.truncate(),
                    ) < distance
                    {
                        target_tile.value = None;
                        continue;
                    }
                    astar.target = None;
                    target_tile.value = Some(actor_tile_position);
                } else {
                    if astar.target != Some(actor_tile_position) {
                        astar.commit_until_index = Some(astar.current_index + 1);
                        astar.target = Some(actor_tile_position);
                    }
                    // A* path
                    if astar.target == Some(actor_tile_position) {
                        continue;
                    }
                    target_tile.value = None;
                    astar.target = Some(actor_tile_position);
                }
            }
            _ => {}
        }
    }
}
