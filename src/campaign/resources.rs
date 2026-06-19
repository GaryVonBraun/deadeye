use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub money: i32,
    pub squad: Vec<SquadMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SquadMember {
    pub name: String,
}
