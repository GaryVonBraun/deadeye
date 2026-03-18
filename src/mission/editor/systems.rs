use bevy::prelude::*;
use rand::RngExt;
use uuid::Uuid;

use crate::{
    core::states::AppState,
    mission::{
        editor::messages::*,
        io::operations::{read_mission_data, remove_mission_file, write_mission},
        resources::{ActiveMission, Mission},
    },
    ui::missions_menu::messages::RefreshMissionListMessage,
    world::map::{components::WorldMap, io::operations::write_map},
};

pub fn create_mission(mut load_editor_writer: MessageWriter<LoadEditorMessage>) {
    info!("creating mission");

    let mission_map = create_map_for_mission();

    let mut rng = rand::rng();

    let mission = Mission {
        id: Uuid::new_v4(),
        name: format!("test mission {:?}", rng.random_range(1..1000)).to_string(),
        map_id: mission_map.id,
    };

    write_mission(&mission);
    load_editor_writer.write(LoadEditorMessage { id: mission.id });
}

pub fn delete_mission(
    mut delete_mission_reader: MessageReader<DeleteMissionMessage>,
    mut refresh_mission_writer: MessageWriter<RefreshMissionListMessage>,
) {
    for message in delete_mission_reader.read() {
        remove_mission_file(message.id);
    }
    refresh_mission_writer.write(RefreshMissionListMessage);
}

pub fn edit_mission(
    mut edit_mission_reader: MessageReader<EditMissionMessage>,
    mut load_editor_writer: MessageWriter<LoadEditorMessage>,
) {
    for mission in edit_mission_reader.read() {
        load_editor_writer.write(LoadEditorMessage { id: mission.id });
    }
}

fn create_map_for_mission() -> WorldMap {
    let raw_matrix: Vec<Vec<u32>> = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 1, 2, 3],
        vec![1, 1, 1, 1, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
    ];

    let mut rng = rand::rng();

    let world_map = WorldMap {
        name: format!("test map {:?}", rng.random_range(1..1000)).to_string(),
        id: Uuid::new_v4(),
        tiles: raw_matrix,
        tileset_name: "base".to_string(),
    };
    write_map(&world_map);
    world_map
}

pub fn load_editor(
    mut load_editor_reader: MessageReader<LoadEditorMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    for message in load_editor_reader.read() {
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };

        commands.insert_resource(ActiveMission { mission });
        next_state.set(AppState::Editor);
    }
}

pub fn enter_editor(active_mission_res: Res<ActiveMission>) {
    info!("Enter editor for: {:?}", active_mission_res.mission.name)
}
