use bevy::prelude::*;

use crate::world::map::{components::WorldMap, io::types::TileSet};

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub mission_map: WorldMap,
    pub tileset: TileSet,
}
