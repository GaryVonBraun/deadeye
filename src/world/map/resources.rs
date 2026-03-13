use bevy::prelude::*;
use uuid::Uuid;

#[derive(Debug, Resource)]
pub struct ActiveMap {
    pub id: Uuid,
}

impl Default for ActiveMap {
    fn default() -> Self {
        Self { id: Uuid::nil() }
    }
}
