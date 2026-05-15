use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct AudioRegistry {
    pub sounds: HashMap<String, Handle<AudioSource>>,
}
