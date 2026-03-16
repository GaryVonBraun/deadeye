use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message)]
pub struct LoadMissionMessage(Uuid);

#[derive(Debug, Message)]
pub struct CreateMissionMessage;

#[derive(Debug, Message)]
pub struct DeleteMissionMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct EditMissionMessage {
    pub id: Uuid,
}
