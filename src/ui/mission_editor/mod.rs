use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{core::states::AppState, ui::mission_editor::editor::editor_ui};

mod components;
mod editor;
pub struct MissionEditorUiPlugin;

impl Plugin for MissionEditorUiPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(AppState::Editor), spawn_mission_editor);
        // app.add_systems(OnExit(AppState::Editor), despawn_mission_editor);
        // app.add_systems(Update, mission_editor_interactions);
        app.add_systems(
            EguiPrimaryContextPass,
            (editor_ui).chain().run_if(in_state(AppState::Editor)),
        );
    }
}
