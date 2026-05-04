use bevy::prelude::*;

use crate::map::{messages::*, rendering::MapRenderingPlugin, systems::*};

pub mod components;
pub mod io;
pub mod messages;
pub mod rendering;
pub mod resources;
mod systems;
pub mod utility;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMapMessage>();
        app.add_message::<DeleteMapMessage>();
        app.add_message::<CreateMapMessage>();
        app.add_message::<EditMapMessage>();
        app.add_message::<SaveMapMessage>();
        app.add_plugins(MapRenderingPlugin);
        app.add_systems(Update, load_map_data.run_if(on_message::<LoadMapMessage>));
        app.add_systems(
            Update,
            delete_map_message.run_if(on_message::<DeleteMapMessage>),
        );
        app.add_systems(Update, save_map.run_if(on_message::<SaveMapMessage>));
    }
}
