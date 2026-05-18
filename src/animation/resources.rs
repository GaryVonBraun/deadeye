use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Resource)]
pub struct AnimationRegistry {
    pub entries: HashMap<String, LoadedAnimation>,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct AnimationDefinition {
    pub default: String,
    pub name: String,
    pub clips: Vec<ClipDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipDefinition {
    pub name: String,
    pub texture: String,
    pub frame_size: (u32, u32),
    pub columns: u32,
    pub rows: u32,
    pub fps: f32,
    pub looping: bool,
    pub freeze: bool,
}

#[derive(Resource, Debug)]
pub struct LoadedAnimation {
    pub default: String,
    pub clips: HashMap<String, LoadedAnimationClip>,
}

#[derive(Debug)]
pub struct LoadedAnimationClip {
    pub image_handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub frame_size: (u32, u32),
    pub columns: u32,
    pub rows: u32,
    pub fps: f32,
    pub looping: bool,
    pub freeze: bool,
}
