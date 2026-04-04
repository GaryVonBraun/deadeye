use bevy::prelude::*;

pub fn tile_world_position(position: Vec2, tile_size: f32) -> Vec2 {
    let floored = (position / tile_size).floor();
    floored * tile_size + tile_size
}
