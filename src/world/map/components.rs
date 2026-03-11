use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct WorldMap {
    pub name: String,
    pub id: Uuid,
    pub tiles: Vec<Vec<u32>>,
}
