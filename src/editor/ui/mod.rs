use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    core::states::AppState, editor::ui::editor::*, map::resources::ActiveMap,
    mission::resources::ActiveMission,
};

mod components;
mod editor;
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
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
