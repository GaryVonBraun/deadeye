use bevy::{ecs::batching::BatchingStrategy, prelude::*};

use crate::{
    actor::components::Actor,
    collision::components::{Collision, CollisionShape, CollisionShape2d},
    props::resources::PropSpatialHash,
};

pub fn check_collision<A: CollisionShape2d, B: CollisionShape2d>(
    pos_a: Vec2,
    collision_a: &A,
    pos_b: Vec2,
    collision_b: &B,
) -> bool {
    let offset_pos_a = pos_a + collision_a.offset();
    let offset_pos_b = pos_b + collision_b.offset();

    match collision_a.shape() {
        CollisionShape::Circle { radius: radius_a } => match collision_b.shape() {
            CollisionShape::Circle { radius: radius_b } => {
                circles_overlap(offset_pos_a, radius_a, offset_pos_b, radius_b)
            }
            CollisionShape::Rect { width, height } => {
                circle_rect_overlap(offset_pos_a, radius_a, offset_pos_b, width, height)
            }
        },
        CollisionShape::Rect {
            width: width_a,
            height: height_a,
        } => match collision_b.shape() {
            CollisionShape::Circle { radius } => {
                circle_rect_overlap(offset_pos_b, radius, offset_pos_a, width_a, height_a)
            }
            CollisionShape::Rect {
                width: width_b,
                height: height_b,
            } => rects_overlap(
                offset_pos_a,
                width_a,
                height_a,
                offset_pos_b,
                width_b,
                height_b,
            ),
        },
    }
}

fn circles_overlap(pos_a: Vec2, radius_a: &f32, pos_b: Vec2, radius_b: &f32) -> bool {
    Vec2::distance(pos_a, pos_b) <= radius_a + radius_b
}

fn circle_rect_overlap(
    pos_a: Vec2,
    radius_a: &f32,
    pos_b: Vec2,
    width: &f32,
    height: &f32,
) -> bool {
    let closest_pos = Vec2 {
        x: f32::clamp(pos_a.x, pos_b.x - width / 2., pos_b.x + width / 2.),
        y: f32::clamp(pos_a.y, pos_b.y - height / 2., pos_b.y + height / 2.),
    };

    Vec2::distance(pos_a, closest_pos) <= *radius_a
}

fn rects_overlap(
    pos_a: Vec2,
    width_a: &f32,
    height_a: &f32,
    pos_b: Vec2,
    width_b: &f32,
    height_b: &f32,
) -> bool {
    let overlap_x = f32::abs(pos_a.x - pos_b.x) < (width_a / 2. + width_b / 2.);
    let overlap_y = f32::abs(pos_a.y - pos_b.y) < (height_a / 2. + height_b / 2.);

    overlap_x && overlap_y
}

//NOTE - could be useful in the future if collision need to be expanded at runtime
pub fn _expand_collision(collision: &Collision, expand_by: f32) -> Collision {
    match collision.shape {
        CollisionShape::Circle { radius } => Collision {
            shape: CollisionShape::Circle {
                radius: radius + expand_by,
            },
            offset: collision.offset,
        },
        CollisionShape::Rect { width, height } => Collision {
            shape: CollisionShape::Rect {
                width: width + expand_by,
                height: height + expand_by,
            },
            offset: collision.offset,
        },
    }
}

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

            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    for (neighbor_pos, neighbor_collision) in
                        prop_spatial_hash.neighbors(cx + dx, cy + dy)
                    {
                        if let Some(push) = calculate_push_vector(
                            pos,
                            actor_collision,
                            *neighbor_pos,
                            neighbor_collision,
                        ) {
                            actor_transform.translation += push.extend(0.);
                        }
                    }
                }
            }
        });
}

pub fn actor_vs_actor_collision(mut actor_query: Query<(&mut Transform, &Collision), With<Actor>>) {
    let mut actors: Vec<(Mut<Transform>, &Collision)> = actor_query.iter_mut().collect();

    for i in 0..actors.len() {
        for j in (i + 1)..actors.len() {
            let (left, right) = actors.split_at_mut(j);
            let (transform_a, collision_a) = &mut left[i];
            let (transform_b, collision_b) = &mut right[0];

            if let Some(push) = calculate_push_vector(
                transform_a.translation.truncate(),
                collision_a,
                transform_b.translation.truncate(),
                collision_b,
            ) {
                transform_a.translation += push.extend(0.) / 2.;
                transform_b.translation -= push.extend(0.) / 2.;
            }
        }
    }
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
