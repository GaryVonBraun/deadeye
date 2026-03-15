use std::path::PathBuf;

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
    index: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileSet {
    pub name: String,
    pub texture: String,
    pub tiles: Vec<TileDef>,
}
