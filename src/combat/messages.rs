use bevy::prelude::*;

#[derive(Message)]
pub struct ShootMessage {
    pub owner: Entity,
    pub direction: Vec2,
}

#[derive(Message)]
pub struct ReloadMessage {
    pub entity: Entity,
}
