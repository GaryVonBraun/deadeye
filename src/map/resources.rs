use bevy::prelude::*;

use crate::map::io::types::{MissionMap, TileSet};

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub map: MissionMap,
    pub tileset: TileSet,
}
