use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub map_id: Uuid,
}

#[derive(Debug, Resource)]
pub struct ActiveMission {
    pub mission: Mission,
}
