use bevy::prelude::*;

use crate::combat::{messages::ShootMessage, projectiles::ProjectilePlugin, weapon::WeaponPlugin};

mod projectiles;
pub mod weapon;
pub mod messages;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShootMessage>();
        app.add_plugins((WeaponPlugin, ProjectilePlugin));
    }
}
