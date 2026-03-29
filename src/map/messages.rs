use bevy::prelude::*;
use uuid::Uuid;

#[derive(Message)]
pub struct LoadMapMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct DeleteMapMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct CreateMapMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct EditMapMessage {
    pub id: Uuid,
}

#[derive(Message)]
pub struct SaveMapMessage;
