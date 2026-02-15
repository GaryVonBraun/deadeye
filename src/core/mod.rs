use bevy::prelude::*;

use crate::core::{camera::setup_camera, states::*, systems::*};

mod camera;
pub mod states;
mod systems;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.init_state::<SimulationState>();
        app.add_systems(Startup, load_app);
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, log_app_state_changes);
    }
}