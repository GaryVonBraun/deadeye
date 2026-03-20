use bevy::prelude::*;

use crate::mission::{editor::MissionEditorPlugin, messages::*, systems::*};

mod components;
pub mod editor;
pub mod io;
pub mod messages;
pub mod resources;
mod systems;
pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMissionMessage>();
        app.add_message::<SaveMissionMessage>();
        app.add_plugins(MissionEditorPlugin);
        app.add_systems(
            Update,
            load_mission.run_if(on_message::<LoadMissionMessage>),
        );
    }
}
