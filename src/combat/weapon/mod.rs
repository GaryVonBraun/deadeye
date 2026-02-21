use bevy::prelude::*;

use crate::combat::weapon::system::shoot_weapon;

mod bundles;
mod component;
pub mod factories;
mod system;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_weapon);
    }
}
