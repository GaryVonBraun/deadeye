use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Message)]
pub struct BuildNavGridMessage {
    pub id: Uuid,
}
