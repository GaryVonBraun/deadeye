use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug)]
pub struct MissionMapChunk {
    pub grid: Vec<Vec<u32>>,
    pub chunk_pos: IVec2,
}
