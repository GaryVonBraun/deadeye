use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{messages::*, resources::ActiveMap, systems::*},
};

mod components;
pub mod io;
pub mod messages;
pub mod resources;
mod systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMapMessage>();
        app.add_message::<DeleteMapMessage>();
        app.add_message::<CreateMapMessage>();
        app.add_message::<EditMapMessage>();
        app.init_resource::<ActiveMap>();
        app.add_systems(Update, load_map_data.run_if(on_message::<LoadMapMessage>));
        app.add_systems(
            Update,
            handle_create_map_message.run_if(on_message::<CreateMapMessage>),
        );
        app.add_systems(Update, load_map_data);
        app.add_systems(
            Update,
            handle_delete_map_message.run_if(on_message::<DeleteMapMessage>),
        );
        app.add_systems(
            Update,
            handle_edit_map_message.run_if(on_message::<EditMapMessage>),
        );
        app.add_systems(OnEnter(AppState::MapEditor), init_map_editor);
    }
}
