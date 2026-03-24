use bevy::prelude::*;
use rand::RngExt;
use uuid::Uuid;

use crate::{
    core::states::AppState,
    map::{
        components::{MapBounds, MissionMap},
        editor::resources::ActiveTile,
        io::operations::write_map,
        messages::LoadMapMessage,
    },
    mission::{editor::messages::*, io::operations::*, resources::*},
    ui::missions_menu::messages::RefreshMissionListMessage,
};

pub fn create_mission(mut load_editor_writer: MessageWriter<LoadEditorMessage>) {
    info!("creating mission");

    let map = create_map_for_mission();

    let mut rng = rand::rng();

    let mission = Mission {
        id: Uuid::new_v4(),
        name: format!("test mission {:?}", rng.random_range(1..1000)).to_string(),
        map_id: map.id,
    };

    //FIXME - Better if we create a save_mission system
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

fn create_map_for_mission() -> MissionMap {
    let raw_matrix: Vec<Vec<u32>> = vec![vec![0]];

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

        //TEMPORARY - maybe its better if we insert this resource in the map plugin
        commands.insert_resource(ActiveTile { index: 0 });

        next_state.set(AppState::Editor);
    }
}

pub fn setup_editor(
    active_mission_res: Res<ActiveMission>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
) {
    info!("Enter editor for: {:?}", active_mission_res.mission.name);
    load_map_writer.write(LoadMapMessage {
        id: active_mission_res.mission.map_id,
    });
}

const CAMERA_SPEED: f32 = 100.;

pub fn editor_camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        error!("no camera found");
        return;
    };
    let mut direction = Vec2::default();
    let mut speed_multiplier: f32 = 1.;

    if keys.pressed(KeyCode::KeyA) {
        direction.x += -1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.y += -1.;
    }

    //TEMPORARY - for now we just keep it simple and can increase speed with shift
    if keys.pressed(KeyCode::ShiftLeft) {
        speed_multiplier = 2.
    }

    let displacement = direction * (CAMERA_SPEED * speed_multiplier) * time.delta_secs();

    camera_transform.translation += displacement.extend(0.0)
}
