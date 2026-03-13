use bevy::prelude::*;
use uuid::Uuid;

#[derive(Message)]
pub struct LoadMapMessage {
    pub id: Uuid,
    pub name: String,
}

#[derive(Message)]
pub struct DeleteMapMessage {
    pub id: Uuid,
}
