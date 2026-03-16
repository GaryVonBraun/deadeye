use bevy::prelude::*;
use rand::RngExt;
use uuid::Uuid;

use crate::{
    mission::{
        io::operations::{remove_mission_file, write_mission},
        messages::DeleteMissionMessage,
        resources::Mission,
    },
    ui::missions_menu::messages::RefreshMissionListMessage,
    world::map::messages::{CreateMapMessage, DeleteMapMessage},
};

pub fn load_mission() {
    info!("load mission")
}

pub fn create_mission(mut map_message_writer: MessageWriter<CreateMapMessage>) {
    info!("creating mission");
    let mut rng = rand::rng();

    let mission = Mission {
        id: Uuid::new_v4(),
        name: format!("test mission {:?}", rng.random_range(1..1000)).to_string(),
        map_id: Uuid::new_v4(),
    };

    write_mission(&mission);

    map_message_writer.write(CreateMapMessage { id: mission.map_id });
}

pub fn delete_mission(
    mut mission_reader: MessageReader<DeleteMissionMessage>,
    mut mission_writer: MessageWriter<RefreshMissionListMessage>,
) {
    for message in mission_reader.read() {
        remove_mission_file(message.id);
    }
    mission_writer.write(RefreshMissionListMessage);
}

pub fn edit_mission() {
    info!("edit mission")
}
