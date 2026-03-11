use bevy::prelude::*;

use crate::{core::states::AppState, world::map::systems::*};

mod components;
mod io;
mod systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(AppState::InGame), spawn_tilemap);
        app.add_systems(Update, map_input_actions.run_if(in_state(AppState::InGame)));
    }
}
