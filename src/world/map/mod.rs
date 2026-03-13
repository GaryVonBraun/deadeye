use bevy::prelude::*;

use crate::{
    core::states::AppState,
    world::map::{editor::MapEditorPlugin, messages::*, resources::ActiveMap, systems::*},
};

mod components;
mod editor;
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
        app.add_plugins(MapEditorPlugin);
        app.init_resource::<ActiveMap>();
        app.add_systems(Update, load_map_data.run_if(on_message::<LoadMapMessage>));

        app.add_systems(Update, load_map_data);
        app.add_systems(
            Update,
            handle_delete_map_message.run_if(on_message::<DeleteMapMessage>),
        );
    }
}
