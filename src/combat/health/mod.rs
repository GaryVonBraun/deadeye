use bevy::prelude::*;

use crate::combat::health::{messages::DamageMessage, systems::apply_damage};

pub mod components;
pub mod messages;
mod systems;
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageMessage>();
        app.add_systems(Update, apply_damage);
    }
}
