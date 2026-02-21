use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Weapon {
    pub fire_rate: f32,
    pub projectile_speed: f32,
}