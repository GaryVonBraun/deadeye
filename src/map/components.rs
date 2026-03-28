use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MissionMap {
    pub name: String,
    pub id: Uuid,
    pub tileset_name: String,
    pub tiles: Vec<Vec<u32>>,
    pub bounds: MapBounds,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapBounds {
    pub north: u32,
    pub south: u32,
    pub east: u32,
    pub west: u32,
}

impl MapBounds {
    pub fn default() -> Self {
        MapBounds {
            north: 1,
            south: 1,
            east: 1,
            west: 1,
        }
    }
}

#[derive(Component, Debug)]
pub struct MissionMapChunk {
    pub grid: Vec<Vec<u32>>,
    pub chunk_pos: IVec2,
}
