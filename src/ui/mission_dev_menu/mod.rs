use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::mission_dev_menu::{
        messages::RefreshMissionDevListMessage,
        systems::{behavior::*, entries::populate_map_list, layout::*},
    },
};

mod components;
pub mod messages;
mod systems;

pub struct MissionDevMenuPlugin;

impl Plugin for MissionDevMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RefreshMissionDevListMessage>();
        app.add_systems(OnExit(AppState::MapListMenu), despawn_missions_menu);
        app.add_systems(
            OnEnter(AppState::MapListMenu),
            (spawn_missions_menu, populate_map_list).chain(),
        );
        app.add_systems(
            Update,
            (mission_menu_interactions, mission_list_interactions)
                .run_if(in_state(AppState::MapListMenu)),
        );
        app.add_systems(
            Update,
            populate_map_list.run_if(on_message::<RefreshMissionDevListMessage>),
        );
    }
}
