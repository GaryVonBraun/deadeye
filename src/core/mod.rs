use bevy::prelude::*;

use crate::core::camera::setup_camera;

mod camera;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}