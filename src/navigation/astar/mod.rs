use bevy::prelude::*;

use crate::{
    core::states::AppState,
    navigation::{astar::systems::*, resources::NavGrid},
};

pub mod components;
mod systems;

pub struct AstarPlugin;

impl Plugin for AstarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (calculate_astar_path, astar_gizmos, astar_navigation)
                .run_if(resource_exists::<NavGrid>)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
