use bevy::prelude::*;

use crate::combat::projectiles::component::Projectile;

pub fn update_projectiles(
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
