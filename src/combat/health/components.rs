use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    max: f32,
    current: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 100.,
            current: 0.,
        }
    }
}
