use bevy::prelude::*;

use crate::{
    collision::{
        components::{Collision, CollisionShape2d},
        utility::swept_collision,
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
    // actors use hurtbox
    actor_query: Query<(Entity, &Hurtbox, &Transform), (With<Health>, Without<Dead>)>,
    // props use collision
    prop_query: Query<(Entity, &Collision, &Transform), With<Prop>>,
    mut message: MessageWriter<DamageMessage>,
    time: Res<Time>,
) {
    for (entity, projectile, transform, hitbox) in projectile_query.iter() {
        let mut closest_hit: Option<(Vec2, Entity, f32)> = None; // position, entity, distance
        let next_pos = transform.translation.truncate()
            + projectile.direction * projectile.speed * time.delta_secs();

        for (target_entity, target_collision, target_transform) in actor_query.iter() {
            if projectile.owner == target_entity {
                continue;
            }

            let Some(position) = swept_collision(
                transform.translation.truncate(),
                next_pos,
                target_transform.translation.truncate(),
                target_collision,
            ) else {
                continue;
            };

            closest_hit = check_closer_position(
                transform.translation.truncate(),
                target_entity,
                position,
                closest_hit,
            )
        }

        for (prop_entity, prop_collision, prop_transform) in prop_query.iter() {
            let Some(position) = swept_collision(
                transform.translation.truncate(),
                next_pos,
                prop_transform.translation.truncate(),
                prop_collision,
            ) else {
                continue;
            };

            closest_hit = check_closer_position(
                transform.translation.truncate(),
                prop_entity,
                position,
                closest_hit,
            )
        }

        if let Some(closest) = closest_hit {
            commands.entity(entity).despawn();
            message.write(DamageMessage {
                target: closest.1,
                amount: projectile.damage,
            });
        }
    }
}

fn check_closer_position(
    projectile_position: Vec2,
    entity: Entity,
    target_position: Vec2,
    closest_hit: Option<(Vec2, Entity, f32)>,
) -> Option<(Vec2, Entity, f32)> {
    let new_distance: f32 = Vec2::distance(projectile_position, target_position);

    let Some(closest) = closest_hit else {
        return Some((target_position, entity, new_distance));
    };

    if new_distance < closest.2 {
        return Some((target_position, entity, new_distance));
    };
    closest_hit
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
