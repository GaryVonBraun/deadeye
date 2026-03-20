use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message)]
pub struct LoadMissionMessage {
    pub id: Uuid,
}

#[derive(Debug, Message)]
pub struct SaveMissionMessage;
