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
        resources::{EditorSettings, EditorTool, ToolAction},
    },
    map::resources::ActiveMap,
    mission::resources::{ActiveMission, WaveDefinition},
    props::io::operations::read_prop_definitions,
};

pub fn editor_tile_picker_panel(
    mut contexts: EguiContexts,
    active_map: Res<ActiveMap>,
    mut update_bounds: MessageWriter<UpdateMapBoundsMessage>,
    mut editor_tools: ResMut<EditorTool>,
    mut editor_settings: ResMut<EditorSettings>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    //TEMPORARY - this is very rough ui code, when things are more certain it should be organized

    egui::SidePanel::right("tile_picker").show(ctx, |ui| {
        ui.label("MapDetails");

        ui.horizontal(|ui| {
            ui.label("Size control amount");
            ui.add(egui::DragValue::new(&mut editor_settings.size_control_amount).speed(1.))
        });

        //FIXME - There has to be a way to fix this:
        let mut write_message: Option<UpdateMapBoundsMessage> = None;

        if let Some(message) =
            size_control_buttons(MapBoundDirectionEnum::East, ui, &editor_settings)
        {
            write_message = Some(message);
        };
        if let Some(message) =
            size_control_buttons(MapBoundDirectionEnum::West, ui, &editor_settings)
        {
            write_message = Some(message);
        };
        if let Some(message) =
            size_control_buttons(MapBoundDirectionEnum::North, ui, &editor_settings)
        {
            write_message = Some(message);
        };
        if let Some(message) =
            size_control_buttons(MapBoundDirectionEnum::South, ui, &editor_settings)
        {
            write_message = Some(message);
        };

        if let Some(message) = write_message {
            update_bounds.write(message);
        }

        ui.label("Tiles");
        ui.separator();

        // perhaps
        // active_map.tileset.name
        // for tile in active_map.tileset.tiles.iter() {
        //     let selected = matches!(*editor_tools, EditorTool::TilePainter(i) if i == tile.index);

        //     let tile_button = ui.add(egui::Button::new(tile.name.clone()).selected(selected));
        //     if tile_button.clicked() {
        //         *editor_tools = EditorTool::TilePainter(tile.index);
        //     }
        // }
    });
    Ok(())
}

fn size_control_buttons(
    direction: MapBoundDirectionEnum,
    ui: &mut Ui,
    editor_settings: &EditorSettings,
) -> Option<UpdateMapBoundsMessage> {
    ui.label(format!("{:?}", direction));
    if ui.button("-").clicked() {
        info!("ok");
        return Some(UpdateMapBoundsMessage {
            direction: direction.clone(),
            action: MapBoundOperationEnum::Shrink(editor_settings.size_control_amount),
        });
    }
    if ui.button("+").clicked() {
        return Some(UpdateMapBoundsMessage {
            direction: direction.clone(),
            action: MapBoundOperationEnum::Expand(editor_settings.size_control_amount),
        });
    }
    None
}

pub fn editor_left_panel(
    mut contexts: EguiContexts,
    mut active_mission: ResMut<ActiveMission>,
    mut save_edits_writer: MessageWriter<SaveEditorChangesMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut editor_tools: ResMut<EditorTool>,
    mut editor_settings: ResMut<EditorSettings>,
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
        ui.label("mission waves");
        if ui.button("add wave").clicked() {
            mission.waves.push(WaveDefinition::default());
        };

        for wave in mission.waves.iter_mut() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Zombie count");
                ui.add(egui::DragValue::new(&mut wave.zombie_count).speed(1.))
            });
            ui.horizontal(|ui| {
                ui.label("Spawn rate");
                ui.add(egui::DragValue::new(&mut wave.spawn_per_second).speed(1.))
            });
        }

        ui.separator();
        ui.label("mission spawnables");

        let back_to_missions = ui.button("Player spawnpoint");
        if back_to_missions.clicked() {
            *editor_tools = EditorTool::PlayerSpawn;
        }

        ui.separator();
        let toggle_grid =
            ui.add(egui::Button::new("Snap to grid").selected(editor_settings.snap_to_grid));

        if toggle_grid.clicked() {
            editor_settings.snap_to_grid = !editor_settings.snap_to_grid;
        }
        ui.label("mission props");

        let remove_prop_button = ui.button("Remove Props");
        if remove_prop_button.clicked() {
            *editor_tools = EditorTool::PropTool(ToolAction::Remove)
        }

        let Ok(definitions) = read_prop_definitions() else {
            return;
        };

        for prop_definition in definitions.props {
            if ui.button(&prop_definition.name).clicked() {
                *editor_tools = EditorTool::PropTool(ToolAction::Place(prop_definition.name));
            };
        }
    });

    Ok(())
}
