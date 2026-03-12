use bevy::prelude::*;

use crate::world::map::MapPlugin;

pub mod map;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin);
    }
}
