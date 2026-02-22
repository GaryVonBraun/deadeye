use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Projectile {
    pub owner: Entity,
    pub speed: f32,
    pub direction: Vec2,
    pub lifetime: f32,
    pub damage: f32,
}
