use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct SpawnActorMessage {
    pub id: String,
    pub position: Vec2,
    pub weapons: Vec<String>,
}
