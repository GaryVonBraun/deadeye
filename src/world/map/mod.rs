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
        app.add_message::<LoadMissionMessage>();
        app.add_message::<DeleteMissionMessage>();
        app.add_message::<CreateMissionMessage>();
        app.add_message::<EditMissionMessage>();
        app.add_plugins(MapEditorPlugin);
        app.init_resource::<ActiveMap>();
        app.add_systems(
            Update,
            load_map_data.run_if(on_message::<LoadMissionMessage>),
        );

        app.add_systems(Update, load_map_data);
        app.add_systems(
            Update,
            handle_delete_map_message.run_if(on_message::<DeleteMissionMessage>),
        );
    }
}
