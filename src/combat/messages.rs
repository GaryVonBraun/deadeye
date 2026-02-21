use bevy::prelude::*;

#[derive(Message)]
pub struct ShootMessage {
    pub shooter: Entity,
    pub direction: Vec2
}