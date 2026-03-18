use bevy::prelude::*;

use crate::{
    core::states::AppState,
    mission::editor::{messages::*, systems::*},
};

pub mod messages;
mod systems;

pub struct MissionEditorPlugin;

impl Plugin for MissionEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CreateMissionMessage>();
        app.add_message::<DeleteMissionMessage>();
        app.add_message::<EditMissionMessage>();
        app.add_message::<LoadEditorMessage>();
        app.add_systems(
            Update,
            create_mission.run_if(on_message::<CreateMissionMessage>),
        );
        app.add_systems(
            Update,
            delete_mission.run_if(on_message::<DeleteMissionMessage>),
        );
        app.add_systems(
            Update,
            edit_mission.run_if(on_message::<EditMissionMessage>),
        );
        app.add_systems(Update, load_editor.run_if(on_message::<LoadEditorMessage>));
        app.add_systems(OnEnter(AppState::Editor), enter_editor);
    }
}
