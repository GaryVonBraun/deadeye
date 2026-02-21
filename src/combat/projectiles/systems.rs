use bevy::prelude::*;

use crate::combat::projectiles::component::Projectile;

pub fn update_projectiles(mut query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    for (projectile, mut transform) in query.iter_mut() {
        let displacement = projectile.direction * projectile.speed * time.delta_secs();

        transform.translation += displacement.extend(0.0);
    }
}
