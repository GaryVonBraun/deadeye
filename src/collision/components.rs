use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollisionShape {
    Circle { radius: f32 },
    Rect { width: f32, height: f32 },
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Collision {
    pub shape: CollisionShape,
    pub offset: Vec2,
}

impl Collision {
    pub fn default() -> Self {
        Collision {
            shape: CollisionShape::Rect {
                width: 64.,
                height: 64.,
            },
            offset: Vec2::default(),
        }
    }
    pub fn from_radius(radius: f32) -> Self {
        Collision {
            shape: CollisionShape::Circle { radius },
            offset: Vec2::default(),
        }
    }
    pub fn from_rect(width: f32, height: f32) -> Self {
        Collision {
            shape: CollisionShape::Rect { width, height },
            offset: Vec2::default(),
        }
    }
    pub fn from_collision(collision: Collision) -> Self {
        collision
    }
}

impl CollisionShape2d for Collision {
    fn shape(&self) -> &CollisionShape {
        &self.shape
    }
    fn offset(&self) -> Vec2 {
        self.offset
    }
}

pub trait CollisionShape2d {
    fn shape(&self) -> &CollisionShape;
    fn offset(&self) -> Vec2;
}
