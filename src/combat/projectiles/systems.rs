use bevy::prelude::*;

use crate::{
    combat::{
        health::{components::Health, messages::DamageMessage},
        projectiles::component::Projectile,
    },
    simulation::collision::components::Collision,
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
    projectile_query: Query<(Entity, &Projectile, &Transform)>,
    health_query: Query<(Entity, &Collision, &Transform), With<Health>>,
    mut message: MessageWriter<DamageMessage>,
) {
    for (entity, projectile, transform) in projectile_query.iter() {
        for (health_entity, collision, health_transform) in health_query.iter() {
            if projectile.owner == health_entity {
                break;
            }
            if Vec2::distance(
                transform.translation.truncate(),
                health_transform.translation.truncate(),
            ) < collision.radius
            {
                commands.entity(entity).despawn();
                info!("{:?} hit something", entity);
                message.write(DamageMessage {
                    target: health_entity,
                    amount: projectile.damage,
                });
            }
        }
    }
}
