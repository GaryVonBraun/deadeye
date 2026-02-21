use bevy::prelude::*;

use crate::combat::weapon::systems::shoot_weapon;

mod bundles;
pub mod component;
pub mod factories;
mod systems;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_weapon);
    }
}
