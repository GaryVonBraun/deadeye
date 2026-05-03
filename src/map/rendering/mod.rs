use bevy::prelude::*;

use crate::map::{
    rendering::{resources::TilesetRenderState, systems::*},
    resources::ActiveMap,
};

mod resources;
mod systems;
pub struct MapRenderingPlugin;

impl Plugin for MapRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_tileset.run_if(resource_added::<ActiveMap>));
        // app.add_systems(
        //     Update,
        //     rerender_map.run_if(resource_exists_and_changed::<ActiveMap>),
        // );
        app.add_systems(
            Update,
            (
                generate_padding,
                render_map,
                rerender_map.run_if(resource_exists_and_changed::<ActiveMap>),
            )
                .run_if(resource_exists::<TilesetRenderState>),
        );
        // app.add_systems(
        //     Update,
        //     chunk_rendering_gizmos.run_if(resource_exists::<ActiveMap>),
        // );
    }
}
