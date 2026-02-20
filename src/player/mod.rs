use bevy::prelude::*;

use crate::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_movement_controller, player_shooting_controller));
    }
}