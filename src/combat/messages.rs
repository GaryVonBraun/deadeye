use bevy::prelude::*;

#[derive(Message)]
pub struct ShootMessage {
    pub owner: Entity,
    pub direction: Vec2
}