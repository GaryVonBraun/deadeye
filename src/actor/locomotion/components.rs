use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Locomotion {
    pub movement_intent: Vec2,
    pub movement_speed: f32
}
