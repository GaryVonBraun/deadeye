use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Weapon {
    pub fire_delay: f32,
    pub cooldown: f32,
    pub projectile_speed: f32,
}
