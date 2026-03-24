use bevy::prelude::*;

use crate::map::{components::WorldMap, io::types::TileSet};

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub tiles: WorldMap,
    pub tileset: TileSet,
}
