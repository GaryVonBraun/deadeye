use bevy::prelude::*;

use crate::collision::components::{Collision, CollisionShape, CollisionShape2d};

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

pub fn expand_collision(collision: &Collision, expand_by: f32) -> Collision {
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
