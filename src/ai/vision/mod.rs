use bevy::prelude::*;

use crate::{ai::vision::systems::*, core::states::AppState};

pub mod components;
mod systems;
pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (vision_detection_system, vision_debug_system).run_if(in_state(AppState::InGame)),
        );
    }
}
