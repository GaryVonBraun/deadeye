use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{
        messages::{DeleteMapMessage, LoadMapMessage},
        systems::*,
    },
};

mod components;
pub mod io;
pub mod messages;
mod systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMapMessage>();
        app.add_message::<DeleteMapMessage>();
        app.add_systems(OnEnter(AppState::InGame), spawn_world_map);
        app.add_systems(
            Update,
            (map_input_actions).run_if(in_state(AppState::InGame)),
        );
        app.add_systems(Update, load_map_data);
        app.add_systems(
            Update,
            handle_delete_map_message.run_if(on_message::<DeleteMapMessage>),
        );
    }
}
