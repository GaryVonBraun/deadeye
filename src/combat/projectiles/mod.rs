use bevy::prelude::*;

use crate::combat::projectiles::systems::update_projectiles;

pub mod bundles;
pub mod component;
mod systems;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles);
    }
}
