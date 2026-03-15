use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct WorldMap {
    pub name: String,
    pub id: Uuid,
    pub tileset_name: String,
    pub tiles: Vec<Vec<u32>>,
}
