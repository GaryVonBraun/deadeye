use bevy::prelude::*;

use crate::{
    collision::{components::Collision, systems::check_collision},
    combat::{
        health::{components::Health, messages::DamageMessage},
        projectiles::component::Projectile,
    },
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

pub fn projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile, &Transform, &Collision)>,
    health_query: Query<(Entity, &Collision, &Transform), With<Health>>,
    mut message: MessageWriter<DamageMessage>,
) {
    for (entity, projectile, transform, collision) in projectile_query.iter() {
        for (target_entity, target_collision, target_transform) in health_query.iter() {
            if projectile.owner == target_entity {
                continue;
            }

            if check_collision(
                transform.translation.truncate(),
                collision,
                target_transform.translation.truncate(),
                target_collision,
            ) {
                commands.entity(entity).despawn();
                info!("{:?} hit something", entity);
                message.write(DamageMessage {
                    target: target_entity,
                    amount: projectile.damage,
                });
            };
        }
    }
}
