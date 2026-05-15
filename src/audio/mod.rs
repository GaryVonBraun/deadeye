use bevy::prelude::*;

use crate::{audio::systems::setup_audio, core::states::AppState};

pub mod resources;
mod systems;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_audio);
    }
}
