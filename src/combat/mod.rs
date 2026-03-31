use bevy::prelude::*;

use crate::combat::{
    health::HealthPlugin, messages::ShootMessage, projectiles::ProjectilePlugin,
    systems::tick_melee_intents, weapon::WeaponPlugin,
};

pub mod components;
pub mod health;
pub mod messages;
mod projectiles;
mod systems;
pub mod weapon;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShootMessage>();
        app.add_plugins((WeaponPlugin, ProjectilePlugin, HealthPlugin));
        app.add_systems(Update, tick_melee_intents);
    }
}
