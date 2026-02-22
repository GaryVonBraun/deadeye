use bevy::prelude::*;

use crate::combat::{
    health::HealthPlugin, messages::ShootMessage, projectiles::ProjectilePlugin,
    weapon::WeaponPlugin,
};

pub mod health;
pub mod messages;
mod projectiles;
pub mod weapon;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShootMessage>();
        app.add_plugins((WeaponPlugin, ProjectilePlugin, HealthPlugin));
    }
}
