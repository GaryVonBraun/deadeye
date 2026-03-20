use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

use crate::{
    core::states::AppState,
    mission::{messages::SaveMissionMessage, resources::ActiveMission},
    world::map::{editor::resources::ActiveTile, resources::ActiveMap},
};

pub fn editor_tile_picker_panel(
    mut contexts: EguiContexts,
    active_map: Res<ActiveMap>,
    mut active_tile: ResMut<ActiveTile>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::right("tile_picker").show(ctx, |ui| {
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

pub fn editor_left_panel(
    mut contexts: EguiContexts,
    mut active_mission: ResMut<ActiveMission>,
    mut save_mission_writer: MessageWriter<SaveMissionMessage>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
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
            save_mission_writer.write(SaveMissionMessage);
        }
        let back_to_missions = ui.button("back to missions");
        if back_to_missions.clicked() {
            next_state.set(AppState::MissionMenu);
        }
    });

    Ok(())
}
