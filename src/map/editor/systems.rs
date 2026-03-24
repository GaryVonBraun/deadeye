use bevy::{prelude::*, sprite_render::TilemapChunkTileData};
use bevy_egui::EguiContexts;

use crate::map::{
    components::WorldMap,
    editor::{
        messages::{MapBoundDirectionEnum, MapBoundOperationEnum, UpdateMapBoundsMessage},
        resources::ActiveTile,
    },
    io::operations::update_map_data,
    messages::LoadMapFromResMessage,
    resources::ActiveMap,
    systems::convert_tiles,
};

pub fn tile_paint_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    active_tile: Res<ActiveTile>,
    mut map_query: Query<(&mut WorldMap, &mut TilemapChunkTileData)>,
    mut active_map: ResMut<ActiveMap>,
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

    active_map.tiles = world_map.clone();
}

pub fn update_map_bounds(
    mut active_map: ResMut<ActiveMap>,
    mut map_bounds_reader: MessageReader<UpdateMapBoundsMessage>,
    mut load_map_from_res_writer: MessageWriter<LoadMapFromResMessage>,
) {
    for message in map_bounds_reader.read() {
        let mission_map_info = active_map.tiles.clone();

        match message.action {
            MapBoundOperationEnum::Shrink(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map.tiles.tiles.remove(0);
                }
                MapBoundDirectionEnum::East => {
                    active_map.tiles.tiles =
                        row_operations(RowOperation::EastShrink, active_map.tiles.tiles.clone());
                }
                MapBoundDirectionEnum::South => {
                    active_map.tiles.tiles.pop();
                }
                MapBoundDirectionEnum::West => {
                    active_map.tiles.tiles =
                        row_operations(RowOperation::WestShrink, active_map.tiles.tiles.clone());
                }
            },
            MapBoundOperationEnum::Expand(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map
                        .tiles
                        .tiles
                        .insert(0, vec![0; mission_map_info.tiles[0].len()]);
                }
                MapBoundDirectionEnum::East => {
                    active_map.tiles.tiles =
                        row_operations(RowOperation::EastExpand, active_map.tiles.tiles.clone());
                    active_map.tiles.bounds.east += amount
                }
                MapBoundDirectionEnum::South => {
                    active_map
                        .tiles
                        .tiles
                        .push(vec![0; mission_map_info.tiles[0].len()]);
                }
                MapBoundDirectionEnum::West => {
                    active_map.tiles.tiles =
                        row_operations(RowOperation::WestExpand, active_map.tiles.tiles.clone());
                    active_map.tiles.bounds.west += amount
                }
            },
        }
        load_map_from_res_writer.write(LoadMapFromResMessage);
    }
}

enum RowOperation {
    WestExpand,
    EastExpand,
    WestShrink,
    EastShrink,
}

fn row_operations(operation: RowOperation, mut tiles: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for row in tiles.iter_mut() {
        match operation {
            RowOperation::WestExpand => row.insert(0, 0),
            RowOperation::EastExpand => row.push(0),
            RowOperation::WestShrink => {
                row.remove(0);
            }
            RowOperation::EastShrink => {
                row.pop();
            }
        }
    }
    tiles
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
