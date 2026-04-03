use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message)]
pub struct LoadPropsMessage {
    pub id: Uuid,
}

#[derive(Debug, Message)]
pub struct SpawnPropMessage {
    pub name: String,
    pub position: Vec2,
}

#[derive(Debug, Message)]
pub struct RemovePropMessage {
    pub position: Vec2,
}

#[derive(Debug, Message)]
pub struct UnloadPropsMessage;
