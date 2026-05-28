use bevy::prelude::*;

pub mod components;
mod systems;

pub struct DirectivePlugin;

impl Plugin for DirectivePlugin {
    fn build(&self, app: &mut App) {}
}
