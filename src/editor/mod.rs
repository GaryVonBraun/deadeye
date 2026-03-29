use bevy::prelude::*;

use crate::{
    core::states::AppState,
    editor::{
        messages::*, resources::EditorTool, systems::*, tools::editor_click_system,
        ui::EditorUiPlugin,
    },
    map::resources::ActiveMap,
};

pub mod messages;
mod resources;
mod systems;
mod tools;
mod ui;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorTool>();
        app.add_plugins(EditorUiPlugin);
        app.add_message::<LoadEditorMessage>();
        app.add_message::<SaveEditorChangesMessage>();

        app.add_systems(
            Update,
            editor_click_system.run_if(resource_exists::<ActiveMap>),
        );
        app.add_message::<UpdateMapBoundsMessage>();
        app.add_systems(
            Update,
            save_editor_changes.run_if(on_message::<SaveEditorChangesMessage>),
        );

        app.add_systems(
            Update,
            update_map_bounds.run_if(on_message::<UpdateMapBoundsMessage>),
        );

        app.add_systems(OnExit(AppState::Editor), exit_editor);

        app.add_systems(Update, load_editor.run_if(on_message::<LoadEditorMessage>));

        app.add_systems(
            Update,
            (editor_camera_controller).run_if(in_state(AppState::Editor)),
        );
    }
}
