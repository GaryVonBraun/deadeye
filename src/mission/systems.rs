use bevy::prelude::*;

use crate::{
    core::states::AppState,
    map::messages::LoadMapMessage,
    mission::{io::operations::read_mission_data, messages::LoadMissionMessage},
};

pub fn load_mission(
    mut load_mission_reader: MessageReader<LoadMissionMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut load_map_writer: MessageWriter<LoadMapMessage>,
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
    }
}
