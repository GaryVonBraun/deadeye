use bevy::prelude::*;

use crate::props::{messages::*, systems::*};

pub mod components;
pub mod io;
pub mod messages;
pub mod resources;
mod systems;

pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnPropMessage>();
        app.add_message::<RemovePropMessage>();
        app.add_message::<LoadPropsMessage>();
        app.add_message::<UnloadPropsMessage>();

        app.add_systems(
            Update,
            load_map_props.run_if(on_message::<LoadPropsMessage>),
        );
        app.add_systems(Update, spawn_prop.run_if(on_message::<SpawnPropMessage>));
        app.add_systems(Update, remove_prop.run_if(on_message::<RemovePropMessage>));
    }
}
