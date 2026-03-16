use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{
        editor::systems::*,
        messages::{CreateMissionMessage, EditMissionMessage},
    },
};

mod systems;

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MapEditor), init_map_editor);
        app.add_systems(
            Update,
            handle_edit_map_message.run_if(on_message::<EditMissionMessage>),
        );
        app.add_systems(
            Update,
            handle_create_map_message.run_if(on_message::<CreateMissionMessage>),
        );
        app.add_systems(
            Update,
            editor_camera_controller.run_if(in_state(AppState::MapEditor)),
        );
    }
}
