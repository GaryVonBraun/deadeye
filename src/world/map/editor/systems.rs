use bevy::{prelude::*, sprite_render::TilemapChunkTileData};
use bevy_egui::EguiContexts;

use crate::world::map::{
    components::WorldMap,
    editor::resources::ActiveTile,
    io::operations::{update_map_data, write_map},
    resources::ActiveMap,
    systems::convert_tiles,
};

pub fn tile_paint_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    active_tile: Res<ActiveTile>,
    mut map_query: Query<(&mut WorldMap, &mut TilemapChunkTileData)>,
    mut contexts: EguiContexts,
) {
    // dont paint if hovering egui
    //FIXME - we dont check if mouse pointer is over ui
    // if contexts.ctx_mut().is_pointer_over_area() {
    //     return;
    // }
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera_query
        .0
        .viewport_to_world_2d(camera_query.1, cursor_pos)
    else {
        return;
    };

    let Ok((mut world_map, mut tile_data)) = map_query.single_mut() else {
        return;
    };

    let tile_size = 64.0;
    let map_width = world_map.tiles[0].len();
    let map_height = world_map.tiles.len();
    let map_half_width = (map_width as f32 * tile_size) / 2.0;
    let map_half_height = (map_height as f32 * tile_size) / 2.0;

    let tile_x = ((world_pos.x + map_half_width) / tile_size) as i32;
    let tile_y = (map_height as i32 - 1) - ((world_pos.y + map_half_height) / tile_size) as i32;

    // bounds check
    if tile_x < 0 || tile_y < 0 || tile_x >= map_width as i32 || tile_y >= map_height as i32 {
        return;
    }

    // update tile
    world_map.tiles[tile_y as usize][tile_x as usize] = active_tile.index as u32;

    // rebuild chunk data
    *tile_data = TilemapChunkTileData(convert_tiles(&world_map.tiles));
}

pub fn save_mission_map(map_query: Query<&WorldMap>) {
    for map in map_query.iter() {
        info!("saving map with id: {}", map.id);
        update_map_data(&map);
    }
}

pub fn exit_editor(mut commands: Commands, map_query: Query<Entity, With<WorldMap>>) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn();
    }
}
