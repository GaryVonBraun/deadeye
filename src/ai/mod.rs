use bevy::prelude::*;

use crate::ai::vision::VisionPlugin;

mod components;
mod vision;
pub mod bundles;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VisionPlugin);
    }
}