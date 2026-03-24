use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct WorldMap {
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
            north: 0,
            south: 0,
            east: 0,
            west: 0,
        }
    }
}
