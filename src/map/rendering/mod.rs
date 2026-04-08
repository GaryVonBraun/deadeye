use bevy::prelude::*;

use crate::map::{rendering::systems::*, resources::ActiveMap};

mod systems;
pub struct MapRenderingPlugin;

impl Plugin for MapRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_map.run_if(resource_added::<ActiveMap>));
        app.add_systems(
            Update,
            rerender_map.run_if(resource_exists_and_changed::<ActiveMap>),
        );
        // app.add_systems(
        //     Update,
        //     chunk_rendering_gizmos.run_if(resource_exists::<ActiveMap>),
        // );
    }
}
