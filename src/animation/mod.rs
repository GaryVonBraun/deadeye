use bevy::prelude::*;

use crate::{
    animation::systems::{load_animation_definitions, sprite_animator},
    core::states::AppState,
};

pub mod components;
pub mod resources;
mod systems;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_animation_definitions);
        app.add_systems(Update, sprite_animator.run_if(in_state(AppState::InGame)));
    }
}
