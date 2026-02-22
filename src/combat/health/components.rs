use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 100.,
            current: 100.,
        }
    }
}
