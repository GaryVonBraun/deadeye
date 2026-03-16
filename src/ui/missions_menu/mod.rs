use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::missions_menu::{
        messages::RefreshMissionListMessage,
        systems::{behavior::*, entries::populate_mission_list, layout::*},
    },
};

mod components;
pub mod messages;
mod systems;

pub struct MissionDevMenuPlugin;

impl Plugin for MissionDevMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RefreshMissionListMessage>();
        app.add_systems(OnExit(AppState::MissionMenu), despawn_missions_menu);
        app.add_systems(
            OnEnter(AppState::MissionMenu),
            (spawn_missions_menu, populate_mission_list).chain(),
        );
        app.add_systems(
            Update,
            (mission_menu_interactions, mission_list_interactions)
                .run_if(in_state(AppState::MissionMenu)),
        );
        app.add_systems(
            Update,
            populate_mission_list.run_if(on_message::<RefreshMissionListMessage>),
        );
    }
}
