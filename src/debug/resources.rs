use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct DebugOptions {
    pub vision: bool,
    pub visible_actors: bool,
    pub target_entity: bool,
    pub hit_box: bool,
    pub hurt_box: bool,
    pub collision: bool,
}

impl DebugOptions {
    pub fn default() -> Self {
        DebugOptions {
            vision: false,
            visible_actors: false,
            target_entity: false,
            hit_box: false,
            hurt_box: false,
            collision: false,
        }
    }
}
