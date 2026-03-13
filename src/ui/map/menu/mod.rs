use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::map::menu::{
        messages::RefreshMapListMessage,
        systems::{behavior::*, entries::populate_map_list, layout::*},
    },
};

mod components;
pub mod messages;
mod systems;

pub struct MapsMenuUiPlugin;

impl Plugin for MapsMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RefreshMapListMessage>();
        app.add_systems(OnExit(AppState::MapListMenu), despawn_map_menu);
        app.add_systems(
            OnEnter(AppState::MapListMenu),
            (spawn_map_menu, populate_map_list).chain(),
        );
        app.add_systems(
            Update,
            (map_menu_interactions, map_list_interactions).run_if(in_state(AppState::MapListMenu)),
        );
        app.add_systems(
            Update,
            populate_map_list.run_if(on_message::<RefreshMapListMessage>),
        );
    }
}
