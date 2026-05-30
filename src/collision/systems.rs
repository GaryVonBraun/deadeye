use std::sync::Arc;

use bevy::{ecs::batching::BatchingStrategy, platform::collections::HashSet, prelude::*};

use crate::{
    actor::{components::Actor, locomotion::components::Locomotion},
    collision::components::{Collision, CollisionShape, CollisionShape2d},
    combat::health::components::Dead,
    map::resources::ActiveMap,
    props::resources::PropSpatialHash,
};


pub fn actor_obstruction_collision(
    mut actor_query: Query<(&mut Transform, &Collision), With<Actor>>,
    prop_spatial_hash: Res<PropSpatialHash>,
) {
    let cell_size = prop_spatial_hash.cell_size;
    actor_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(50000))
        .for_each(|(mut actor_transform, actor_collision)| {
            let pos = actor_transform.translation.truncate();
            let cx = (pos.x / cell_size).floor() as i32;
            let cy = (pos.y / cell_size).floor() as i32;

            let mut seen: HashSet<(u32, u32)> = HashSet::new();
            let mut total_push = Vec2::ZERO;

            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    for (neighbor_pos, neighbor_collision) in
                        prop_spatial_hash.neighbors(cx + dx, cy + dy)
                    {
                        let key = (neighbor_pos.x.to_bits(), neighbor_pos.y.to_bits());
                        if !seen.insert(key) {
                            continue;
                        }
                        if let Some(push) = calculate_push_vector(
                            pos,
                            actor_collision,
                            *neighbor_pos,
                            neighbor_collision,
                        ) {
                            total_push += push;
                        }
                    }
                }
            }
            actor_transform.translation += total_push.extend(0.);
        });
}

const GRID_CELL_SIZE: f32 = 16.;
const ACTOR_SEPARATION_DISTANCE: f32 = 12.0;
pub fn actor_vs_actor_collision(
    mut moving_query: Query<
        (Entity, &mut Transform, &Locomotion, &Collision),
        (With<Actor>, Without<Dead>),
    >,
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
    let pairs: Vec<(Entity, Vec2)> = moving_query
        .iter()
        .map(|(e, t, l, c)| (e, t.translation.truncate() + c.offset))
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

    moving_query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(50000))
        .for_each(|(entity, mut transform, mut locomotion, collision)| {
            let (flat, offsets, pairs) = &*grid_data;
            let pos = transform.translation.truncate() + collision.offset;
            let (cx, cy) = (
                ((pos.x + west_offset) / GRID_CELL_SIZE).floor() as i32,
                ((-pos.y + north_offset) / GRID_CELL_SIZE).floor() as i32,
            );

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
                        let min_dist = ACTOR_SEPARATION_DISTANCE;

                        if dist_sq >= min_dist * min_dist || dist_sq == 0.0 {
                            continue;
                        }

                        let distance = dist_sq.sqrt();
                        let push = diff / distance * (min_dist - distance); // how much to correct

                        let movement_dir = locomotion.move_direction.normalize_or_zero();
                        let dot = push.normalize_or_zero().dot(movement_dir);

                        if dot < 0.0 {
                            // push opposes movement — we walked into this neighbor
                            transform.translation += push.extend(0.0) * 0.1;
                        }
                    }
                }
            }
        });
}

fn calculate_push_vector(
    pos_a: Vec2,
    collision_a: &Collision,
    pos_b: Vec2,
    collision_b: &Collision,
) -> Option<Vec2> {
    let offset_pos_a = pos_a + collision_a.offset;
    let offset_pos_b = pos_b + collision_b.offset;

    match collision_a.shape {
        CollisionShape::Circle { radius: radius_a } => match collision_b.shape {
            CollisionShape::Circle { radius: radius_b } => {
                circles_push_vector(offset_pos_a, radius_a, offset_pos_b, radius_b)
            }
            CollisionShape::Rect { width, height } => {
                circle_rect_push_vector(offset_pos_a, radius_a, offset_pos_b, width, height)
            }
        },
        CollisionShape::Rect {
            width: width_a,
            height: height_a,
        } => match collision_b.shape {
            CollisionShape::Circle { radius } => {
                circle_rect_push_vector(offset_pos_b, radius, offset_pos_a, width_a, height_a)
            }
            CollisionShape::Rect {
                width: _width_b,
                height: _height_b,
            } => {
                //NOTE - this might be added or not, it will depend if actors will ever have a rect collision shape
                error!("Haven't implemented rect actor collision yet");
                todo!()
            }
        },
    }
}

fn circles_push_vector(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> Option<Vec2> {
    let delta = pos_a - pos_b;
    let distance = delta.length();

    let penetration = radius_a + radius_b - distance;

    if distance == 0. {
        return Some(Vec2::X * penetration);
    }
    if penetration < 0. {
        return None;
    }
    return Some(delta.normalize() * penetration);
}

fn circle_rect_push_vector(
    pos_a: Vec2,
    radius_a: f32,
    pos_b: Vec2,
    width: f32,
    height: f32,
) -> Option<Vec2> {
    let closest_pos = Vec2 {
        x: f32::clamp(pos_a.x, pos_b.x - width / 2., pos_b.x + width / 2.),
        y: f32::clamp(pos_a.y, pos_b.y - height / 2., pos_b.y + height / 2.),
    };

    let delta = pos_a - closest_pos;
    let distance = delta.length();
    let penetration = radius_a - distance;

    if distance == 0. {
        return Some(Vec2::X * penetration);
    }
    if penetration < 0. {
        return None;
    }
    return Some(delta.normalize() * penetration);
}
