use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlacedProp {
    pub definition_name: String,
    pub position: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinition {
    pub name: String,
    pub sprite: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinitions {
    pub props: Vec<PropDefinition>,
}
