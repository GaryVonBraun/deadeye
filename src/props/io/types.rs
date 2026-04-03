use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::collision::components::Collision;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlacedProp {
    pub id: String,
    pub position: Vec2,
    #[serde(skip)]
    pub entity: Option<Entity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropDefinition {
    pub name: String,
    pub sprite: String,
    pub size: Vec2,
    pub tile_aligned: bool,
    pub collision: Collision,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinitions {
    pub props: Vec<PropDefinition>,
}
