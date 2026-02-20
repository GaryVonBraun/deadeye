use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerMovementIntent {
    pub direction: Vec2
}

#[derive(Component, Debug)]
pub struct PlayerShootingIntent {
    pub direction: Vec2
}
    