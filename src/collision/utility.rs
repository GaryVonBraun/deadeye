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

pub fn swept_collision(
    ray_start: Vec2,
    ray_end: Vec2,
    target_pos: Vec2,
    target: &impl CollisionShape2d,
) -> Option<Vec2> {
    match target.shape() {
        crate::collision::components::CollisionShape::Circle { radius } => {
            let circle_center = target_pos + target.offset();

            let ray_dir = ray_end - ray_start;
            let ray_length = ray_dir.length();
            if ray_length == 0.0 {
                return None;
            }
            let ray_dir_normalized = ray_dir / ray_length;
            let proj_t = (circle_center - ray_start).dot(ray_dir_normalized);
            let closest_point = ray_start + ray_dir_normalized * proj_t.clamp(0.0, ray_length);
            let d_perp = Vec2::distance(closest_point, circle_center);
            if d_perp < *radius {
                let entry_t =
                    (proj_t - (radius * radius - d_perp * d_perp).sqrt()).clamp(0.0, ray_length);
                return Some(ray_start + ray_dir_normalized * entry_t);
            }
            None
        }
        crate::collision::components::CollisionShape::Rect { width, height } => {
            let rect_center = target_pos + target.offset();
            let half_w = width / 2.0;
            let half_h = height / 2.0;

            let tl = rect_center + Vec2::new(-half_w, half_h);
            let tr = rect_center + Vec2::new(half_w, half_h);
            let bl = rect_center + Vec2::new(-half_w, -half_h);
            let br = rect_center + Vec2::new(half_w, -half_h);

            let edges = [(tl, tr), (tr, br), (br, bl), (bl, tl)];

            let ray_dir = ray_end - ray_start;
            let ray_length = ray_dir.length();
            if ray_length == 0.0 {
                return None;
            }
            let ray_dir_normalized = ray_dir / ray_length;

            let mut closest_hit: Option<(f32, Vec2)> = None; // (distance, point)

            for (a, b) in edges {
                let segment_dir = b - a;
                let denom = ray_dir_normalized.perp_dot(segment_dir);
                if denom.abs() < 0.0001 {
                    continue;
                }

                let t = (a - ray_start).perp_dot(segment_dir) / denom;
                let u = (a - ray_start).perp_dot(ray_dir_normalized) / denom;

                if t >= 0.0 && t <= ray_length && u >= 0.0 && u <= 1.0 {
                    let hit_point = ray_start + ray_dir_normalized * t;
                    match closest_hit {
                        None => closest_hit = Some((t, hit_point)),
                        Some((prev_t, _)) if t < prev_t => closest_hit = Some((t, hit_point)),
                        _ => {}
                    }
                }
            }

            closest_hit.map(|(_, point)| point)
        }
    }
}
