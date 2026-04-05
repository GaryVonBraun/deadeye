use bevy::prelude::*;

use crate::map::io::types::MapBounds;

pub fn tile_world_position(position: Vec2, tile_size: f32) -> Vec2 {
    let floored = (position / tile_size).floor();
    floored * tile_size + tile_size
}

pub fn world_to_grid(position: Vec2, tile_size: f32, bounds: &MapBounds) -> (i32, i32) {
    let tile_x = (position.x / tile_size).floor() as i32 + bounds.west as i32;
    let tile_y = (-position.y / tile_size).floor() as i32 + bounds.north as i32;
    (tile_x, tile_y)
}

pub fn grid_to_world(grid_x: i32, grid_y: i32, tile_size: f32, bounds: &MapBounds) -> Vec2 {
    Vec2::new(
        (grid_x - bounds.west as i32) as f32 * tile_size + tile_size / 2.,
        -(grid_y - bounds.north as i32) as f32 * tile_size - tile_size / 2.,
    )
}
