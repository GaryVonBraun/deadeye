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
    pub name: String,
    pub texture: String,
    pub tiles: Vec<TileDef>,
}
