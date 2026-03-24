use bevy::prelude::*;

use crate::mission::{io::operations::read_mission_data, messages::LoadMissionMessage};

pub fn load_mission(mut load_mission_reader: MessageReader<LoadMissionMessage>) {
    for message in load_mission_reader.read() {
        info!("load mission: {:?}", message.id);
        let Ok(mission) = read_mission_data(&message.id) else {
            return;
        };

        info!("found mission called: {:?}", mission.name);
    }
}
