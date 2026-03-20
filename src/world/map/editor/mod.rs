use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{editor::systems::*, messages::EditMapMessage},
};

pub mod resources;
mod systems;

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tile_paint_system.run_if(in_state(AppState::Editor)));
    }
}
