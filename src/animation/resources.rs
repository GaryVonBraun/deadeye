use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct AnimationDefinitions {
    pub defs: HashMap<String, AnimationDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationDef {
    pub default: String,
    pub clips: HashMap<String, AnimationClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationClip {
    pub texture: String,
    pub frame_size: (u32, u32),
    pub columns: u32,
    pub rows: u32,
    pub fps: u16,
    pub looping: bool,
    pub freeze: bool,
}
