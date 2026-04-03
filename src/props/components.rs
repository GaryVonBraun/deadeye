use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Prop {
    pub size: Vec2,
}

impl Prop {
    pub fn default() -> Self {
        Prop {
            size: Vec2::default(),
        }
    }
    pub fn with_size(size: Vec2) -> Self {
        Prop { size }
    }
}
