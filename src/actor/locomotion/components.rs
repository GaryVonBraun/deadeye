use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Locomotion {
    pub move_direction: Vec2,
    pub speed: f32
}
