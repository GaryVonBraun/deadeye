use bevy::prelude::*;
use rand::RngExt;
use uuid::Uuid;

use crate::{
    core::states::AppState,
    mission::{
        io::operations::{read_mission_data, remove_mission_file, write_mission},
        messages::LoadMissionMessage,
        resources::Mission,
    },
    ui::missions_menu::messages::RefreshMissionListMessage,
    world::map::messages::CreateMapMessage,
};

pub fn load_mission(mut load_mission_reader: MessageReader<LoadMissionMessage>) {
    for message in load_mission_reader.read() {
        info!("load mission: {:?}", message.id);
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };

        info!("found mission called: {:?}", mission.name);
    }
}
