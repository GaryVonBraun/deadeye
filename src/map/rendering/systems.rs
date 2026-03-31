use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::{
    core::components::GameEntity,
    map::{components::MissionMapChunk, io::types::TileSet, resources::ActiveMap},
};

pub fn render_map(
    active_map: Res<ActiveMap>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("rendering map!");

    info!("{:?}", active_map.map.bounds);

    let y_offset = (active_map.map.bounds.north.saturating_sub(8) + 15) / 16;
    let x_offset = (active_map.map.bounds.east.saturating_sub(8) + 15) / 16;

    // ranges are used to take pieces of a chunk later
    let y_ranges = chunk_ranges(active_map.map.bounds.south, active_map.map.bounds.north);
    let x_ranges = chunk_ranges(active_map.map.bounds.west, active_map.map.bounds.east);

    for (y, y_range) in y_ranges.iter().enumerate() {
        for (x, x_range) in x_ranges.iter().enumerate() {
            // the chunk position is position on the grid, e.g x: 2 y: 0
            let chunk_y_position = y_offset as i32 - y as i32;
            let chunk_x_position = x_offset as i32 - x as i32;

            let chunk_pixel_size = active_map.tileset.chunk_size * active_map.tileset.tile_size;

            // precise world position  of the chunks for rendering tiles and chunk placement
            let chunk_world_position = Vec2 {
                x: chunk_x_position as f32 * chunk_pixel_size,
                y: chunk_y_position as f32 * chunk_pixel_size,
            };

            // 16x16 grid created filled with the tiles from defined ranges
            let grid: Vec<Vec<u32>> =
                construct_chunk_tiles(&active_map.map.tiles, y_range, x_range);

            let mut mesh = Mesh::new(
                bevy::mesh::PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );

            // constructing mesh data from sliced grid
            let (positions, uvs) = construct_mesh_data(&grid, &active_map.tileset);

            // for each grid tile we need to create 2 triangles
            let total_tiles = grid.len() * grid[0].len();
            let mut indices: Vec<u32> = vec![];
            for i in 0..total_tiles {
                let offset = i as u32 * 4;

                // first triangle
                indices.push(offset);
                indices.push(offset + 1);
                indices.push(offset + 3);

                // second triangle
                indices.push(offset + 1);
                indices.push(offset + 2);
                indices.push(offset + 3);
            }

            // adding attributes to mesh
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh.insert_indices(bevy::mesh::Indices::U32(indices));

            let mesh_handle = meshes.add(mesh);

            // creating material
            let texture = asset_server.load(&active_map.tileset.texture);
            let material_handle = materials.add(ColorMaterial {
                texture: Some(texture),
                ..default()
            });

            // spawning chunk
            commands.spawn({
                (
                    MissionMapChunk {
                        grid,
                        chunk_pos: IVec2 {
                            x: chunk_x_position,
                            y: chunk_y_position,
                        },
                    },
                    GameEntity,
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(material_handle),
                    Transform::from_xyz(chunk_world_position.x, chunk_world_position.y, 0.0),
                )
            });
        }
    }
}

fn construct_mesh_data(grid: &Vec<Vec<u32>>, tileset: &TileSet) -> (Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut positions: Vec<[f32; 3]> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];

    for (y, col) in grid.iter().enumerate() {
        for (x, tile_type) in col.iter().enumerate() {
            // x decreases as column index increases (index 0 = easternmost after row reversal).
            let tile_left = (7.5 - x as f32) * tileset.tile_size - (tileset.tile_size / 2.);
            let tile_bottom = (7.5 - y as f32) * tileset.tile_size - (tileset.tile_size / 2.);
            let Some(tile_def) = tileset.tiles.iter().find(|t| t.index == *tile_type as u16) else {
                continue;
            };

            // creating 4 positions for each corner of a tile
            positions.push([tile_left, tile_bottom, 0.]);
            positions.push([tile_left + tileset.tile_size, tile_bottom, 0.]);
            positions.push([
                tile_left + tileset.tile_size,
                tile_bottom + tileset.tile_size,
                0.,
            ]);
            positions.push([tile_left, tile_bottom + tileset.tile_size, 0.]);

            // corners of tile texture
            //TEMPORARY - currently the texture map 10x10 hardcoded but this should be based on data
            let columns = 10.0_f32;
            let rows = 10.0_f32;

            let padding = 0.0;
            let start_x = tile_def.uv_coordinate[0] as f32 / columns + padding;
            let end_x = (tile_def.uv_coordinate[0] as f32 + 1.0) / columns - padding;
            let start_y = tile_def.uv_coordinate[1] as f32 / rows + padding;
            let end_y = (tile_def.uv_coordinate[1] as f32 + 1.0) / rows - padding;

            // putting values as uv
            uvs.push([start_x, end_y]);
            uvs.push([end_x, end_y]);
            uvs.push([end_x, start_y]);
            uvs.push([start_x, start_y]);
        }
    }

    return (positions, uvs);
}

fn construct_chunk_tiles(
    tiles: &Vec<Vec<u32>>,
    y_range: &(usize, usize, ChunkSide),
    x_range: &(usize, usize, ChunkSide),
) -> Vec<Vec<u32>> {
    // destructuring ranges
    let (y_start, y_end, y_side) = y_range;
    let (x_start, x_end, x_side) = x_range;

    // getting offset based on what side the range is on
    let y_offset = calculate_offset(y_start, y_end, y_side);
    let x_offset = calculate_offset(x_start, x_end, x_side);

    // extracting tiles from grid
    let sliced_grid: Vec<Vec<u32>> = tiles[y_range.0..y_range.1]
        .iter()
        .map(|row| {
            let reversed: Vec<u32> = row.iter().rev().cloned().collect();
            reversed[x_range.0..x_range.1].to_vec()
        })
        .collect();

    // creating 16x16 base grid so sliced grid can be copied on it
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 16]; 16];

    // using offsets to decide where tiles are placed
    for (i, row) in sliced_grid.iter().enumerate() {
        grid[y_offset + i][x_offset..x_offset + row.len()].copy_from_slice(row);
    }

    return grid;
}

fn calculate_offset(start: &usize, end: &usize, side: &ChunkSide) -> usize {
    match side {
        ChunkSide::Positive => {
            let slice_height = end - start;
            16 - slice_height
        }
        ChunkSide::Border(positive) => {
            // border represents the center grid
            // if it has less than 0 positive tiles it needs to be offset
            if *positive > 8 { 0 } else { 8 - positive }
        }
        ChunkSide::Negative => 0,
    }
}

#[derive(Debug)]
enum ChunkSide {
    Positive,
    Border(usize),
    Negative,
}

fn chunk_ranges(negative: u32, positive: u32) -> Vec<(usize, usize, ChunkSide)> {
    // we subtract 8 to see what is left over
    let positive_leftover = positive.saturating_sub(8);
    let negative_leftover = negative.saturating_sub(8);

    let mut ranges: Vec<(usize, usize, ChunkSide)> = vec![];

    // border start is the beginning tile of the center chunk and end is the last
    let border_start = positive_leftover as usize;
    let border_end = (positive + negative) as usize - negative_leftover as usize;

    // we get all negative ranges
    if positive_leftover > 0 {
        let odd_tiles = positive_leftover % 16;
        let full_chunks = positive_leftover / 16;

        // the odd tiles are the left overs of chunks and are put at the beginning of the range
        if odd_tiles > 0 {
            ranges.push((0, odd_tiles as usize, ChunkSide::Positive));
        }

        // full chunks are put after the odd tiles if there is any
        for i in 0..full_chunks {
            ranges.push((
                (odd_tiles + 16 * i) as usize,
                (odd_tiles + 16 * (i + 1)) as usize,
                ChunkSide::Positive,
            ))
        }
    }

    // border/origin chunk are at start and end.
    ranges.push((
        border_start,
        border_end,
        ChunkSide::Border(positive as usize),
    ));

    if negative_leftover > 0 {
        let full_chunks = negative_leftover / 16;

        // create a full range each full chunk
        for i in 0..full_chunks {
            ranges.push((
                (border_end + (16 * i) as usize),
                (border_end + 16 * (i + 1) as usize),
                ChunkSide::Negative,
            ))
        }

        // the "odd" tiles here are at the end of the full chunks and the total end
        if (positive + negative) > border_end as u32 + 16 * full_chunks {
            ranges.push((
                border_end + 16 * full_chunks as usize,
                (positive + negative) as usize,
                ChunkSide::Negative,
            ));
        }
    }

    return ranges;
}

pub fn rerender_map(
    active_map: Res<ActiveMap>,
    chunk_query: Query<Entity, With<MissionMapChunk>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in chunk_query.iter() {
        commands.entity(entity).despawn();
    }
    render_map(active_map, commands, meshes, asset_server, materials);
}
pub fn chunk_rendering_gizmos(
    active_map: Res<ActiveMap>,
    chunk_query: Query<(&Transform, &MissionMapChunk), With<MissionMapChunk>>,
    mut gizmos: Gizmos,
) {
    for (transform, chunk) in chunk_query.iter() {
        let chunk_pixel_size = active_map.tileset.chunk_size * active_map.tileset.tile_size;
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2 {
                x: chunk_pixel_size,
                y: chunk_pixel_size,
            },
            Color::linear_rgb(0.7, 0., 0.7),
        );
    }
}
