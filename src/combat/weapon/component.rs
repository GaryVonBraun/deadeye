use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Weapon {
    pub fire_delay: f32,
    pub cooldown: f32,
    pub speed: f32,
    pub damage: f32,
}

#[derive(Component, Debug)]
pub struct ShootingIntent {
    pub direction: Vec2,
}

impl ShootingIntent {
    pub fn default() -> Self {
        ShootingIntent {
            direction: Vec2::default(),
        }
    }
}
