use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct DebugOptions {
    pub vision: bool,
    pub visible_actors: bool,
    pub nearest_visible_hostile: bool,
    pub hit_box: bool,
    pub hurt_box: bool,
    pub collision: bool,
    pub astar_paths: bool,
}

impl DebugOptions {
    pub fn default() -> Self {
        DebugOptions {
            vision: false,
            visible_actors: false,
            nearest_visible_hostile: false,
            hit_box: false,
            hurt_box: false,
            collision: false,
            astar_paths: false,
        }
    }
}
