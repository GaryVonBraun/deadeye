use bevy::prelude::*;

use crate::{
    animation::components::SpriteAnimator,
    combat::weapon::components::{Weapon, WeaponRuntime},
};

#[derive(Bundle)]
pub struct WeaponBundle {
    pub sprite: Sprite,
    pub weapon: Weapon,
    pub weapon_runtime: WeaponRuntime,
    pub transform: Transform,
    pub sprite_animator: SpriteAnimator,
}
