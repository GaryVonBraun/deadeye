use bevy::prelude::*;

use crate::combat::weapon::component::Weapon;

#[derive(Bundle)]
pub struct WeaponBundle{
    pub sprite: Sprite,
    pub weapon: Weapon,
    pub transform: Transform
}