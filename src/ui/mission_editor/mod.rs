use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    core::states::AppState, map::resources::ActiveMap, mission::resources::ActiveMission,
    ui::mission_editor::editor::*,
};

mod components;
mod editor;
pub struct MissionEditorUiPlugin;

impl Plugin for MissionEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (editor_left_panel)
                .chain()
                .run_if(in_state(AppState::Editor).and(resource_exists::<ActiveMission>)),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            editor_tile_picker_panel
                .run_if(in_state(AppState::Editor).and(resource_exists::<ActiveMap>)),
        );
    }
}
