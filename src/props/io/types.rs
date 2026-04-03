use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlacedProp {
    pub id: String,
    pub position: Vec2,
    #[serde(skip)]
    pub entity: Option<Entity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinition {
    pub name: String,
    pub sprite: String,
    pub size: Vec2,
    pub tile_aligned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinitions {
    pub props: Vec<PropDefinition>,
}
