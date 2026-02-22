use bevy::prelude::*;

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

#[derive(Component, Debug)]
pub struct PlayerShootingIntent {
    pub direction: Vec2,
}

impl PlayerShootingIntent {
    pub fn default() -> Self {
        PlayerShootingIntent {
            direction: Vec2::default(),
        }
    }
}
