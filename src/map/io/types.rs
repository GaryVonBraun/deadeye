use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifestEntry {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifest {
    pub maps: Vec<MapManifestEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileDef {
    pub index: u16,
    pub name: String,
    pub uv_coordinate: [u32; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileSet {
    pub tile_size: f32,
    pub chunk_size: f32,
    pub name: String,
    pub texture: String,
    pub tiles: Vec<TileDef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
