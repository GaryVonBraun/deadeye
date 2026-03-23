use bevy::prelude::*;

use crate::{
    core::states::AppState,
    mission::messages::SaveMissionMessage,
    world::map::{
        editor::{messages::UpdateMapBoundsMessage, systems::*},
        resources::ActiveMap,
    },
};

pub mod messages;
pub mod resources;
mod systems;

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UpdateMapBoundsMessage>();
        app.add_systems(
            Update,
            tile_paint_system.run_if(in_state(AppState::Editor).and(resource_exists::<ActiveMap>)),
        );
        app.add_systems(
            Update,
            save_mission_map.run_if(on_message::<SaveMissionMessage>),
        );
        app.add_systems(
            Update,
            update_map_bounds.run_if(on_message::<UpdateMapBoundsMessage>),
        );
        app.add_systems(OnExit(AppState::Editor), exit_editor);
    }
}
