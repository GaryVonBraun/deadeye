use bevy::prelude::*;

use crate::{
    collision::{
        components::{Collision, CollisionShape2d},
        systems::check_collision,
    },
    combat::{
        health::{
            components::{Dead, Health, Hitbox, Hurtbox},
            messages::DamageMessage,
        },
        projectiles::component::Projectile,
    },
    props::components::Prop,
};

pub fn move_projectiles(
    mut query: Query<(Entity, &mut Projectile, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut projectile, mut transform) in query.iter_mut() {
        let displacement = projectile.direction * projectile.speed * time.delta_secs();
        transform.translation += displacement.extend(0.0);

        if projectile.lifetime < 0. {
            commands.entity(entity).despawn();
        }

        projectile.lifetime -= time.delta_secs();
    }
}

pub fn projectile_hit(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile, &Transform, &Hitbox)>,
    health_query: Query<(Entity, &Hurtbox, &Transform), (With<Health>, Without<Dead>)>,
    mut message: MessageWriter<DamageMessage>,
    time: Res<Time>,
) {
    for (entity, projectile, transform, hitbox) in projectile_query.iter() {
        for (target_entity, target_collision, target_transform) in health_query.iter() {
            if projectile.owner == target_entity {
                continue;
            }

            let next_pos = transform.translation.truncate()
                + projectile.direction * projectile.speed * time.delta_secs();

            if swept_collision(
                transform.translation.truncate(),
                next_pos,
                target_transform.translation.truncate(),
                target_collision,
            )
            .is_some()
            {
                commands.entity(entity).despawn();
                message.write(DamageMessage {
                    target: target_entity,
                    amount: projectile.damage,
                });
                break;
            }
        }
    }
}

pub fn projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Hitbox, &Projectile), With<Projectile>>,
    health_query: Query<(&Collision, &Transform), With<Prop>>,
    time: Res<Time>,
) {
    for (entity, transform, hitbox, projectile) in projectile_query.iter() {
        for (target_collision, target_transform) in health_query.iter() {
            let next_pos = transform.translation.truncate()
                + projectile.direction * projectile.speed * time.delta_secs();

            if swept_collision(
                transform.translation.truncate(),
                next_pos,
                target_transform.translation.truncate(),
                target_collision,
            )
            .is_some()
            {
                commands.entity(entity).despawn();
                break;
            }
        }
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
            let t = (circle_center - ray_start)
                .dot(ray_dir_normalized)
                .clamp(0.0, ray_length);
            let closest_point = ray_start + ray_dir_normalized * t;
            if Vec2::distance(closest_point, circle_center) < *radius {
                return Some(closest_point);
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
