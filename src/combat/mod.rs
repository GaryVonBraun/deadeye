use bevy::prelude::*;

use crate::combat::{projectiles::ProjectilePlugin, weapon::WeaponPlugin};

mod projectiles;
pub mod weapon;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WeaponPlugin, ProjectilePlugin));
    }
}
