use bevy::prelude::*;

pub mod components;
pub mod systems;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {}
}
