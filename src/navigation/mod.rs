use bevy::prelude::*;

use crate::{
    map::resources::ActiveMap,
    navigation::{flow_field::FlowFieldPlugin, messages::*, resources::NavGrid, systems::*},
};

pub mod flow_field;
pub mod messages;
pub mod resources;
mod systems;
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BuildNavGridMessage>();

        app.add_plugins(FlowFieldPlugin);

        app.add_systems(Update, build_nav_grid.run_if(resource_added::<ActiveMap>));
        // app.add_systems(Update, nav_grid_gizmo.run_if(resource_exists::<NavGrid>));
    }
}
