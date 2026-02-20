use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Weapon {
    pub fire_rate: f32,
    pub projectile_speed: f32,
}

#[derive(Component, Debug)]
pub struct PendingShot {
    direction: Vec2
}
