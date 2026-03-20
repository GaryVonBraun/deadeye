use bevy::prelude::*;

use crate::world::map::{editor::MapEditorPlugin, messages::*, resources::ActiveMap, systems::*};

pub mod components;
pub mod editor;
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
        app.add_systems(Update, load_map_data.run_if(on_message::<LoadMapMessage>));
        app.add_systems(
            Update,
            create_new_map.run_if(on_message::<CreateMapMessage>),
        );
        app.add_systems(
            Update,
            delete_map_message.run_if(on_message::<DeleteMapMessage>),
        );
    }
}
