use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Ui, load::SizedTexture, scroll_area::ScrollSource},
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
    let texture_handle = active_map.texture.clone();

    let texture_id = contexts.add_image(bevy_egui::EguiTextureHandle::Strong(texture_handle));
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

        let tile_size = active_map.tileset.tile_size;
        let tileset_width = active_map.tileset.width as f32;
        let tileset_height = active_map.tileset.height as f32;

        // total image size (no padding here, just display)
        let base_size = egui::Vec2::new(tileset_width * tile_size, tileset_height * tile_size);
        let image_size = base_size * editor_settings.tile_picker_zoom;

        egui::ScrollArea::both()
            .scroll_source(ScrollSource::NONE)
            .show(ui, |ui| {
                // allocate space for image
                let (rect, response) =
                    ui.allocate_exact_size(image_size, egui::Sense::click_and_drag());

                // apply panning offset
                let rect = rect.translate(editor_settings.tile_picker_offset);

                // draw image manually
                ui.painter().image(
                    texture_id,
                    rect,
                    egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)),
                    egui::Color32::WHITE,
                );

                // Zooming
                if response.hovered() {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);

                    if scroll != 0.0 {
                        let zoom_before = editor_settings.tile_picker_zoom;

                        // mouse position in screen space
                        if let Some(mouse_pos) = response.hover_pos() {
                            // position relative to image before zoom
                            let local_before = (mouse_pos - rect.min) / zoom_before;

                            // apply zoom
                            editor_settings.tile_picker_zoom *= (1.0 + scroll * 0.001);
                            editor_settings.tile_picker_zoom =
                                editor_settings.tile_picker_zoom.clamp(0.2, 8.0);

                            let zoom_after = editor_settings.tile_picker_zoom;

                            let local_after = local_before * zoom_after;

                            editor_settings.tile_picker_offset +=
                                (local_before * zoom_before) - local_after;
                        }
                    }
                }

                // Right mouse panning
                if ui.input(|i| i.pointer.secondary_down()) && response.hovered() {
                    let delta = ui.input(|i| i.pointer.delta());
                    editor_settings.tile_picker_offset += delta;
                }

                // Tile selection
                if let Some(pos) = response.hover_pos() {
                    let local = pos - rect.min;

                    let corrected = local / editor_settings.tile_picker_zoom;

                    let tile_x = (corrected.x / tile_size) as u32;
                    let tile_y = (corrected.y / tile_size) as u32;

                    let max_x = active_map.tileset.width as u32;
                    let max_y = active_map.tileset.height as u32;

                    if tile_x < max_x && tile_y < max_y {
                        if response.clicked_by(egui::PointerButton::Primary) {
                            *editor_tools = EditorTool::TilePainter {
                                x: tile_x,
                                y: tile_y,
                            };
                        }

                        ui.label(format!("Tile: ({}, {})", tile_x, tile_y));
                    }
                }

                let zoom = editor_settings.tile_picker_zoom;
                let tile_size = active_map.tileset.tile_size;

                let tiles_x = active_map.tileset.width as u32;
                let tiles_y = active_map.tileset.height as u32;

                let step = tile_size * zoom;

                // vertical lines
                for x in 0..=tiles_x {
                    let x_pos = rect.min.x + x as f32 * step;

                    ui.painter().line_segment(
                        [egui::pos2(x_pos, rect.min.y), egui::pos2(x_pos, rect.max.y)],
                        egui::Stroke::new(2.0, egui::Color32::from_white_alpha(100)),
                    );
                }

                // horizontal lines
                for y in 0..=tiles_y {
                    let y_pos = rect.min.y + y as f32 * step;

                    ui.painter().line_segment(
                        [egui::pos2(rect.min.x, y_pos), egui::pos2(rect.max.x, y_pos)],
                        egui::Stroke::new(2.0, egui::Color32::from_white_alpha(100)),
                    );
                }

                // Highlighted tile
                if let EditorTool::TilePainter { x, y } = *editor_tools {
                    let highlight_rect = egui::Rect::from_min_size(
                        rect.min
                            + egui::vec2(
                                x as f32 * tile_size * editor_settings.tile_picker_zoom,
                                y as f32 * tile_size * editor_settings.tile_picker_zoom,
                            ),
                        egui::vec2(
                            tile_size * editor_settings.tile_picker_zoom,
                            tile_size * editor_settings.tile_picker_zoom,
                        ),
                    );

                    ui.painter().rect_stroke(
                        highlight_rect,
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::GREEN),
                        egui::StrokeKind::Outside,
                    );
                }
            });
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
            grow_tile: editor_settings.grow_tile,
        });
    }
    if ui.button("+").clicked() {
        return Some(UpdateMapBoundsMessage {
            direction: direction.clone(),
            action: MapBoundOperationEnum::Expand(editor_settings.size_control_amount),
            grow_tile: editor_settings.grow_tile,
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
