use bevy::prelude::*;

use crate::combat::projectiles::component::Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: Sprite,
    pub transform: Transform,
}
