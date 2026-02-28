use bevy::prelude::*;

use crate::{combat::projectiles::component::Projectile, core::components::GameEntity};

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: Sprite,
    pub transform: Transform,
    pub game_entity: GameEntity,
}
