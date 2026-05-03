use bevy::prelude::*;

use crate::{animation::components::SpriteAnimator, combat::weapon::components::Weapon};

#[derive(Bundle)]
pub struct WeaponBundle {
    pub sprite: Sprite,
    pub weapon: Weapon,
    pub transform: Transform,
    pub sprite_animator: SpriteAnimator,
}
