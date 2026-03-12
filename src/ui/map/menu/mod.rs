use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::map::menu::systems::{entries::populate_map_list, layout::*},
};

mod components;
mod systems;

pub struct MapsMenuUiPlugin;

impl Plugin for MapsMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MapListMenu),
            (spawn_map_list, populate_map_list).chain(),
        );
        app.add_systems(OnExit(AppState::MapListMenu), despawn_map_list);
    }
}
