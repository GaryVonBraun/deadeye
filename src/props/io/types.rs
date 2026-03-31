use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlacedProp {
    definition_id: String,
    position: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinition {
    name: String,
    sprite: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropDefinitions {
    props: Vec<PropDefinition>,
}
