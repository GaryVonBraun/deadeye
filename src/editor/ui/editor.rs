use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Ui},
};

use crate::{
    core::states::AppState,
    editor::{
        messages::{
            MapBoundDirectionEnum, MapBoundOperationEnum, SaveEditorChangesMessage,
            UpdateMapBoundsMessage,
        },
        resources::{ActiveTile, EditorTool, SelectedProp, ToolAction},
    },
    map::resources::ActiveMap,
    mission::resources::ActiveMission,
    props::io::operations::read_prop_definitions,
};

pub fn editor_tile_picker_panel(
    mut contexts: EguiContexts,
    active_map: Res<ActiveMap>,
    mut active_tile: ResMut<ActiveTile>,
    mut update_bounds: MessageWriter<UpdateMapBoundsMessage>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    //TEMPORARY - this is very rough ui code, when things are more certain it should be organized

    egui::SidePanel::right("tile_picker").show(ctx, |ui| {
        ui.label("MapDetails");

        //FIXME - There has to be a way to fix this:
        let mut write_message: Option<UpdateMapBoundsMessage> = None;

        if let Some(message) = size_control_buttons(MapBoundDirectionEnum::East, ui) {
            write_message = Some(message);
        };
        if let Some(message) = size_control_buttons(MapBoundDirectionEnum::West, ui) {
            write_message = Some(message);
        };
        if let Some(message) = size_control_buttons(MapBoundDirectionEnum::North, ui) {
            write_message = Some(message);
        };
        if let Some(message) = size_control_buttons(MapBoundDirectionEnum::South, ui) {
            write_message = Some(message);
        };

        if let Some(message) = write_message {
            update_bounds.write(message);
        }

        ui.label("Tiles");
        ui.separator();
        for tile in active_map.tileset.tiles.iter() {
            let tile_button = ui.add(
                egui::Button::new(tile.name.clone()).selected(tile.index == active_tile.index),
            );
            if tile_button.clicked() {
                active_tile.index = tile.index;
            }
        }
    });
    Ok(())
}

fn size_control_buttons(
    direction: MapBoundDirectionEnum,
    ui: &mut Ui,
) -> Option<UpdateMapBoundsMessage> {
    ui.label(format!("{:?}", direction));
    if ui.button("+").clicked() {
        return Some(UpdateMapBoundsMessage {
            direction: direction.clone(),
            action: MapBoundOperationEnum::Expand(1),
        });
    }
    if ui.button("-").clicked() {
        return Some(UpdateMapBoundsMessage {
            direction: direction.clone(),
            action: MapBoundOperationEnum::Shrink(1),
        });
    }
    None
}

pub fn editor_left_panel(
    mut contexts: EguiContexts,
    mut active_mission: ResMut<ActiveMission>,
    mut save_edits_writer: MessageWriter<SaveEditorChangesMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut editor_tool: ResMut<EditorTool>,
    mut selected_prop: ResMut<SelectedProp>,
) -> Result {
    //TEMPORARY - this is very rough ui code, when things are more certain it should be organized

    let ctx = contexts.ctx_mut()?;
    let mission = &mut active_mission.mission;
    egui::SidePanel::left("mission_properties").show(ctx, |ui| {
        ui.label("mission properties");
        ui.separator();
        ui.text_edit_singleline(&mut mission.name);
        ui.label(format!("mission id: {}", mission.id));
        ui.label(format!("map id: {}", mission.map_id));

        ui.separator();
        let save_button = ui.button("save changes");
        if save_button.clicked() {
            save_edits_writer.write(SaveEditorChangesMessage);
        }
        let back_to_missions = ui.button("back to missions");
        if back_to_missions.clicked() {
            next_state.set(AppState::MissionMenu);
        }

        ui.separator();
        ui.label("mission spawnables");
        let save_button = ui.button("Tile painter");
        if save_button.clicked() {
            *editor_tool = EditorTool::TilePainter;
        }
        let back_to_missions = ui.button("Player spawnpoint");
        if back_to_missions.clicked() {
            *editor_tool = EditorTool::PlayerSpawn;
        }

        ui.separator();
        ui.label("mission props");

        let Ok(definitions) = read_prop_definitions() else {
            return;
        };

        for prop_definition in definitions.props {
            if ui.button(&prop_definition.name).clicked() {
                *editor_tool = EditorTool::PropTool(ToolAction::Place);
                selected_prop.name = prop_definition.name;
            };
        }
    });

    Ok(())
}
