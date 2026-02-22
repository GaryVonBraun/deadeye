use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collision {
    pub radius: f32,
}

impl Collision {
    pub fn from_radius(radius: f32) -> Self {
        Collision {radius: radius}
    }
}