use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct SpawnPlayerMessage {
    pub position: Vec2,
}
