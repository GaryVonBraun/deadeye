use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Locomotion {
    pub move_direction: Vec2,
    pub speed: f32,
}

impl Locomotion {
    pub fn from_speed(speed: f32) -> Self {
        Locomotion {
            move_direction: Vec2::default(),
            speed: speed,
        }
    }
}
