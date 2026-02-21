use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2,
}
