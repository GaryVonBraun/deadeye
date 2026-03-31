use bevy::prelude::*;

use crate::{combat::weapon::components::Weapon, core::components::GameEntity};

#[derive(Bundle)]
pub struct WeaponBundle {
    pub sprite: Sprite,
    pub weapon: Weapon,
    pub transform: Transform,
}
