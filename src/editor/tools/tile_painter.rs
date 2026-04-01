use bevy::prelude::*;

use crate::map::resources::ActiveMap;

pub fn tile_paint_system(tile_index: &u16, mut active_map: ResMut<ActiveMap>, tile_position: Vec2) {
    // check if tile is the same already
    if active_map.map.tiles[tile_position.y as usize][tile_position.x as usize]
        == *tile_index as u32
    {
        return;
    }

    // update tile
    active_map.map.tiles[tile_position.y as usize][tile_position.x as usize] = *tile_index as u32;
}
