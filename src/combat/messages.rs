use bevy::prelude::*;

#[derive(Message)]
pub struct ShootMessage {
    pub owner: Entity,
    pub direction: Vec2,
    pub just_pressed: bool,
}

#[derive(Message)]
pub struct ReloadMessage {
    pub entity: Entity,
}
