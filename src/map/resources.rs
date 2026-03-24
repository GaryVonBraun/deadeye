use bevy::prelude::*;

use crate::map::{components::MissionMap, io::types::TileSet};

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub map: MissionMap,
    pub tileset: TileSet,
}
