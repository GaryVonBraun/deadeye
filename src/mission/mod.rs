use bevy::prelude::*;

use crate::mission::{messages::*, systems::*};

pub mod io;
pub mod messages;
pub mod resources;
mod systems;
pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMissionMessage>();
        app.add_message::<SaveMissionMessage>();
        app.add_message::<CreateMissionMessage>();
        app.add_message::<DeleteMissionMessage>();
        app.add_message::<EditMissionMessage>();

        app.add_systems(
            Update,
            load_mission.run_if(on_message::<LoadMissionMessage>),
        );

        // crud systems
        app.add_systems(
            Update,
            save_mission.run_if(on_message::<SaveMissionMessage>),
        );
        app.add_systems(
            Update,
            create_new_mission.run_if(on_message::<CreateMissionMessage>),
        );
        app.add_systems(
            Update,
            delete_mission.run_if(on_message::<DeleteMissionMessage>),
        );
        app.add_systems(
            Update,
            edit_mission.run_if(on_message::<EditMissionMessage>),
        );
    }
}
