use bevy::prelude::*;

use crate::world::map::{components::WorldMap, io::types::TileSet};

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub tiles: WorldMap,
    pub tileset: TileSet,
}
