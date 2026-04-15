use bevy::prelude::*;

use crate::collision::components::{CollisionShape, CollisionShape2d};

#[derive(Component, Debug)]
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

#[derive(Component)]
pub struct Hurtbox {
    pub shape: CollisionShape,
    pub offset: Vec2,
}

impl CollisionShape2d for Hurtbox {
    fn shape(&self) -> &CollisionShape {
        &self.shape
    }
    fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[derive(Component)]
pub struct Hitbox {
    pub shape: CollisionShape,
    pub offset: Vec2,
}

impl CollisionShape2d for Hitbox {
    fn shape(&self) -> &CollisionShape {
        &self.shape
    }
    fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[derive(Component, PartialEq)]
pub struct Dead;
