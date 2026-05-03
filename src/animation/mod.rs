use bevy::prelude::*;

use crate::{animation::systems::*, core::states::AppState};

pub mod components;
pub mod resources;
pub mod systems;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_animation_definitions);
        app.add_systems(Update, sprite_animator.run_if(in_state(AppState::InGame)));
        app.add_systems(
            Update,
            (
                zombie_animation_state,
                player_animation_state,
                // weapon_animation_state,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
