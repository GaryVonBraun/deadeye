use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::map::{
    components::MissionMapChunk,
    editor::{
        messages::{MapBoundDirectionEnum, MapBoundOperationEnum, UpdateMapBoundsMessage},
        resources::ActiveTile,
    },
    io::operations::update_map_data,
    resources::ActiveMap,
};

pub fn tile_paint_system(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    active_tile: Res<ActiveTile>,
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

    let tile_size = 64.0;
    let map_width = active_map.map.tiles[0].len();
    let map_height = active_map.map.tiles.len();

    let tile_x =
        ((world_pos.x + active_map.map.bounds.west as f32 * tile_size) / tile_size).floor() as i32;
    let tile_y =
        ((active_map.map.bounds.north as f32 * tile_size - world_pos.y) / tile_size).floor() as i32;

    // bounds check
    if tile_x < 0 || tile_y < 0 || tile_x >= map_width as i32 || tile_y >= map_height as i32 {
        return;
    }

    // check if tile is the same already
    if active_map.map.tiles[tile_y as usize][tile_x as usize] == active_tile.index as u32 {
        return;
    }

    // update tile
    active_map.map.tiles[tile_y as usize][tile_x as usize] = active_tile.index as u32;
}

const TILE_INDEX: u32 = 1;

pub fn update_map_bounds(
    mut active_map: ResMut<ActiveMap>,
    mut map_bounds_reader: MessageReader<UpdateMapBoundsMessage>,
) {
    for message in map_bounds_reader.read() {
        let map_info = active_map.map.clone();
        match message.action {
            MapBoundOperationEnum::Shrink(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map.map.tiles.remove(0);
                    active_map.map.bounds.north -= amount
                }
                MapBoundDirectionEnum::East => {
                    active_map.map.tiles =
                        row_operations(RowOperation::EastShrink, active_map.map.tiles.clone());
                    active_map.map.bounds.east -= amount
                }
                MapBoundDirectionEnum::South => {
                    active_map.map.tiles.pop();
                    active_map.map.bounds.south -= amount
                }
                MapBoundDirectionEnum::West => {
                    active_map.map.tiles =
                        row_operations(RowOperation::WestShrink, active_map.map.tiles.clone());
                    active_map.map.bounds.west -= amount
                }
            },
            MapBoundOperationEnum::Expand(amount) => match message.direction {
                MapBoundDirectionEnum::North => {
                    active_map
                        .map
                        .tiles
                        .insert(0, vec![TILE_INDEX; map_info.tiles[0].len()]);
                    active_map.map.bounds.north += amount
                }
                MapBoundDirectionEnum::East => {
                    active_map.map.tiles =
                        row_operations(RowOperation::EastExpand, active_map.map.tiles.clone());
                    active_map.map.bounds.east += amount
                }
                MapBoundDirectionEnum::South => {
                    active_map
                        .map
                        .tiles
                        .push(vec![TILE_INDEX; map_info.tiles[0].len()]);
                    active_map.map.bounds.south += amount
                }
                MapBoundDirectionEnum::West => {
                    active_map.map.tiles =
                        row_operations(RowOperation::WestExpand, active_map.map.tiles.clone());
                    active_map.map.bounds.west += amount
                }
            },
        }
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
            RowOperation::WestExpand => row.insert(0, TILE_INDEX),
            RowOperation::EastExpand => row.push(TILE_INDEX),
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

pub fn save_map(active_map: Res<ActiveMap>) {
    info!("saving map with id: {}", active_map.map.id);
    update_map_data(&active_map.map);
}

pub fn exit_editor(mut commands: Commands, map_query: Query<Entity, With<MissionMapChunk>>) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn();
    }
}
