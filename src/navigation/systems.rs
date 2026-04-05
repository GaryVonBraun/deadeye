use bevy::prelude::*;

use crate::{
    map::{
        io::{
            operations::{read_map_data, read_tileset},
            paths::tileset_path,
            types::MapBounds,
        },
        resources::ActiveMap,
    },
    navigation::{messages::BuildNavGridMessage, resources::NavGrid},
    props::io::operations::read_prop_definitions,
};

pub fn build_nav_grid(
    mut build_grid_reader: MessageReader<BuildNavGridMessage>,
    mut commands: Commands,
) {
    let Ok(prop_definitions) = read_prop_definitions() else {
        error!("Prop definitions not found needed to build NavGrid");
        return;
    };

    let Ok(tile_set) = read_tileset(tileset_path("base".to_string())) else {
        return;
    };
    for message in build_grid_reader.read() {
        let Ok(map_data) = read_map_data(&message.id) else {
            error!(
                "Failed to find map data needed to load props, map: {:?}",
                message.id
            );
            return;
        };

        let width = map_data.bounds.east + map_data.bounds.west;
        let height = map_data.bounds.north + map_data.bounds.south;

        // create grid filled with true
        let cells: Vec<Vec<bool>> = vec![vec![true; width as usize]; height as usize];

        let mut nav_grid = NavGrid {
            cells,
            width,
            height,
        };

        for placed_prop in map_data.placed_props {
            let Some(prop) = prop_definitions
                .props
                .iter()
                .find(|p| p.name == placed_prop.id)
            else {
                error!("Failed to find prop in definitions needed to build NavGrid");
                continue;
            };

            // we only allow grid collision with for aligned props
            if !prop.tile_aligned {
                continue;
            }

            let tiles_wide = (prop.size.x / tile_set.tile_size).ceil() as u32;
            let tiles_high = (prop.size.y / tile_set.tile_size).ceil() as u32;

            let tile_position =
                world_to_grid(placed_prop.position, tile_set.tile_size, &map_data.bounds);
            let start_tile_x = tile_position.0 - (tiles_wide / 2) as i32;
            let start_tile_y = tile_position.1 - (tiles_high / 2) as i32;

            for dx in 0..tiles_wide {
                for dy in 0..tiles_high {
                    let x = start_tile_x + dx as i32;
                    let y = start_tile_y + dy as i32;
                    if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
                        nav_grid.cells[y as usize][x as usize] = false;
                        info!("grid position set to false")
                    }
                }
            }
        }

        info!("Inserted nav grid");

        commands.insert_resource(nav_grid);
    }
}

pub fn world_to_grid(position: Vec2, tile_size: f32, bounds: &MapBounds) -> (i32, i32) {
    let tile_x = (position.x / tile_size).floor() as i32 + bounds.west as i32;
    let tile_y = (-position.y / tile_size).floor() as i32 + bounds.north as i32;
    (tile_x, tile_y)
}

pub fn nav_grid_gizmo(nav_grid: Res<NavGrid>, active_map: Res<ActiveMap>, mut gizmos: Gizmos) {
    let bounds = &active_map.map.bounds;
    let tile_size = &active_map.tileset.tile_size;

    let x_offset = bounds.west as f32 * tile_size;
    let y_offset = bounds.north as f32 * tile_size;

    for y in 0..nav_grid.cells.len() {
        for x in 0..nav_grid.cells[0].len() {
            let gizmo_color: Color;

            if nav_grid.cells[y][x] {
                gizmo_color = Color::linear_rgb(0., 1., 0.);
            } else {
                gizmo_color = Color::linear_rgb(1., 0., 0.);
            }

            gizmos.circle_2d(
                Vec2 {
                    x: x as f32 * tile_size - x_offset + tile_size / 2.,
                    y: -(y as f32 * tile_size) + y_offset - tile_size / 2.,
                },
                *tile_size / 2.,
                gizmo_color,
            );
        }
    }
}
