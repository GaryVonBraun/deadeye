use bevy::prelude::*;

use crate::{editor::resources::ActiveTile, map::resources::ActiveMap};

pub fn tile_paint_system(
    active_tile: Res<ActiveTile>,
    mut active_map: ResMut<ActiveMap>,
    tile_position: IVec2,
) {
    // check if tile is the same already
    if active_map.map.tiles[tile_position.y as usize][tile_position.x as usize]
        == active_tile.index as u32
    {
        return;
    }

    // update tile
    active_map.map.tiles[tile_position.y as usize][tile_position.x as usize] =
        active_tile.index as u32;
}
