use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerMovementIntent {
    pub direction: Vec2,
}

impl PlayerMovementIntent {
    pub fn default() -> Self {
        PlayerMovementIntent {
            direction: Vec2::default(),
        }
    }
}