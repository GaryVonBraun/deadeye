use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{messages::LoadMapMessage, systems::*},
};

mod components;
pub mod io;
mod messages;
mod systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMapMessage>();
        app.add_systems(OnEnter(AppState::InGame), spawn_world_map);
        app.add_systems(
            Update,
            (map_input_actions, load_map).run_if(in_state(AppState::InGame)),
        );
    }
}
