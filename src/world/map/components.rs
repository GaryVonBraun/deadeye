use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct WorldMap {
    pub name: String,
    pub uuid: Uuid,
    pub tiles: Vec<Vec<u32>>,
}
