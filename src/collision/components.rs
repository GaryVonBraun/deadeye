use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CollisionShape {
    Circle { radius: f32 },
    Rect { width: f32, height: f32 },
}

#[derive(Component, Debug)]
pub struct Collision {
    pub shape: CollisionShape,
    pub offset: Vec2,
}

impl Collision {
    pub fn from_radius(radius: f32) -> Self {
        Collision {
            shape: CollisionShape::Circle { radius },
            offset: Vec2::default(),
        }
    }
}
