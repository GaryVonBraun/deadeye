use bevy::prelude::*;
use uuid::Uuid;

#[derive(Message)]
pub struct LoadMissionMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct DeleteMissionMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct CreateMissionMessage;

#[derive(Message)]
pub struct EditMissionMessage {
    pub id: Uuid,
}
