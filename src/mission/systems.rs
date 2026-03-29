use bevy::{prelude::*, transform::commands};
use rand::RngExt;
use uuid::Uuid;

use crate::{
    actor::{humanoid::factories::spawn_player_humanoid, messages::SpawnPlayerMessage},
    core::states::AppState,
    editor::messages::LoadEditorMessage,
    map::{
        components::{MapBounds, MissionMap},
        io::operations::write_map,
        messages::LoadMapMessage,
    },
    mission::{
        io::operations::{
            read_mission_data, remove_mission_file, update_mission_data, write_mission,
        },
        messages::*,
        resources::{ActiveMission, Mission},
    },
    ui::missions_menu::messages::RefreshMissionListMessage,
};

pub fn load_mission(
    mut load_mission_reader: MessageReader<LoadMissionMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
    mut spawn_player_writer: MessageWriter<SpawnPlayerMessage>,
) {
    for message in load_mission_reader.read() {
        info!("load mission: {:?}", message.id);
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };

        info!("found mission called: {:?}", mission.name);

        // set state to game
        next_state.set(AppState::InGame);

        //set active map
        load_map_writer.write(LoadMapMessage { id: mission.map_id });
        spawn_player_writer.write(SpawnPlayerMessage {
            position: mission.player_spawn,
        });
    }
}

// crud systems
pub fn save_mission(active_mission: Res<ActiveMission>) {
    info!("{:?}", active_mission);
    update_mission_data(&active_mission.mission);
}

pub fn create_new_mission(mut load_editor_writer: MessageWriter<LoadEditorMessage>) {
    info!("creating mission");

    //NOTE - i am not really a fan of this but this is the simplest way for now
    let map = create_map_for_mission();

    let mut rng = rand::rng();

    let mission = Mission {
        id: Uuid::new_v4(),
        name: format!("test mission {:?}", rng.random_range(1..1000)).to_string(),
        map_id: map.id,
        player_spawn: Vec2::splat(0.),
    };

    // write the new mission to drive
    write_mission(&mission);

    load_editor_writer.write(LoadEditorMessage { id: mission.id });
}

fn create_map_for_mission() -> MissionMap {
    let raw_matrix: Vec<Vec<u32>> = vec![vec![2, 2], vec![2, 2]];

    let mut rng = rand::rng();

    let map = MissionMap {
        name: format!("test map {:?}", rng.random_range(1..1000)).to_string(),
        id: Uuid::new_v4(),
        tiles: raw_matrix,
        tileset_name: "base".to_string(),
        bounds: MapBounds::default(),
    };
    write_map(&map);
    map
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
