use bevy::prelude::*;

use crate::navigation::{flow_field::FlowFieldPlugin, messages::*, resources::NavGrid, systems::*};

pub mod flow_field;
pub mod messages;
mod resources;
mod systems;
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BuildNavGridMessage>();

        app.add_plugins(FlowFieldPlugin);

        app.add_systems(
            Update,
            build_nav_grid.run_if(on_message::<BuildNavGridMessage>),
        );
        // app.add_systems(Update, nav_grid_gizmo.run_if(resource_exists::<NavGrid>));
    }
}
