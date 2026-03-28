use bevy::{asset::RenderAssetUsages, mesh::VertexFormat, prelude::*, state::commands};

use crate::map::{io::types::TileSet, rendering::components::Chunk, resources::ActiveMap};

const CHUNK_SIZE: f32 = 16. * 64.;

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

    let y_ranges = chunk_ranges(active_map.map.bounds.south, active_map.map.bounds.north);
    let x_ranges = chunk_ranges(active_map.map.bounds.west, active_map.map.bounds.east);

    for (y, y_range) in y_ranges.iter().enumerate() {
        for (x, x_range) in x_ranges.iter().enumerate() {
            // chunk

            let chunk_y_position = y_offset as i32 - y as i32;
            let chunk_x_position = x_offset as i32 - x as i32;
            let grid: Vec<Vec<u32>> =
                construct_chunk_tiles(&active_map.map.tiles, y_range, x_range);
            //okay so we have the grid of tiles for each chunk
            // info!(" grid: y{}, x:{} = {:?}", y, x, grid);

            let mut mesh = Mesh::new(
                bevy::mesh::PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );

            let chunk_position = Vec2 {
                x: chunk_x_position as f32 * CHUNK_SIZE,
                y: chunk_y_position as f32 * CHUNK_SIZE,
            };

            let (positions, uvs) = construct_mesh_data(&grid, &active_map.tileset);
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

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh.insert_indices(bevy::mesh::Indices::U32(indices));

            let mesh_handle = meshes.add(mesh);

            let texture = asset_server.load(&active_map.tileset.texture);

            let material_handle = materials.add(ColorMaterial {
                texture: Some(texture),
                ..default()
            });

            commands.spawn({
                (
                    Chunk {
                        grid,
                        chunk_pos: IVec2 {
                            x: chunk_x_position,
                            y: chunk_y_position,
                        },
                    },
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(material_handle),
                    Transform::from_xyz(chunk_position.x, chunk_position.y, 0.0),
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
            // Local space, centered at entity origin to match gizmo rendering.
            // X decreases as column index increases (index 0 = easternmost after row reversal).
            let tile_left = (7.5 - x as f32) * 64. - 32.;
            let tile_bottom = (7.5 - y as f32) * 64. - 32.;
            let Some(tile_def) = tileset.tiles.iter().find(|t| t.index == *tile_type as u16) else {
                continue;
            };

            positions.push([tile_left, tile_bottom, 0.]);
            positions.push([tile_left + 64., tile_bottom, 0.]);
            positions.push([tile_left + 64., tile_bottom + 64., 0.]);
            positions.push([tile_left, tile_bottom + 64., 0.]);

            let start_x = tile_def.uv_coordinate[0] as f32 / 10.;
            let end_x = tile_def.uv_coordinate[0] as f32 / 10. + 0.1;

            let start_y = tile_def.uv_coordinate[1] as f32 / 10.;
            let end_y = tile_def.uv_coordinate[1] as f32 / 10. + 0.1;

            uvs.push([start_x, end_y]);
            uvs.push([end_x, end_y]);
            uvs.push([end_x, start_y]);
            uvs.push([start_x, start_y]);
        }
    }

    (positions, uvs)
}

fn construct_chunk_tiles(
    tiles: &Vec<Vec<u32>>,
    y_range: &(usize, usize, ChunkSide),
    x_range: &(usize, usize, ChunkSide),
) -> Vec<Vec<u32>> {
    let (y_start, y_end, y_side) = y_range;
    let (x_start, x_end, x_side) = x_range;

    let y_offset = calculate_offset(y_start, y_end, y_side);
    let x_offset = calculate_offset(x_start, x_end, x_side);
    //FIXME - somewhere here is the issue or something
    // info!("chunk ranges x: {:?}, y: {:?}", x_range, y_range);
    // info!("offsets  x: {:?}, y: {:?}", x_offset, y_offset);

    let sliced_grid: Vec<Vec<u32>> = tiles[y_range.0..y_range.1]
        .iter()
        .map(|row| {
            let reversed: Vec<u32> = row.iter().rev().cloned().collect();
            reversed[x_range.0..x_range.1].to_vec()
        })
        .collect();

    // info!("sliced grid {:?}", sliced_grid);

    let mut grid: Vec<Vec<u32>> = vec![vec![0; 16]; 16];

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
        ChunkSide::Border(negative) => {
            if *negative > 8 {
                0
            } else {
                8 - negative
            }
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
    let positive_leftover = positive.saturating_sub(8);
    let negative_leftover = negative.saturating_sub(8);

    let mut ranges: Vec<(usize, usize, ChunkSide)> = vec![];

    let border_start = positive_leftover as usize;
    let border_end = (positive + negative) as usize - negative_leftover as usize;
    if positive_leftover > 0 {
        let odd_tiles = positive_leftover % 16;
        let full_chunks = positive_leftover / 16;

        if odd_tiles > 0 {
            ranges.push((0, odd_tiles as usize, ChunkSide::Positive));
        }

        for i in 0..full_chunks {
            ranges.push((
                (odd_tiles + 16 * i) as usize,
                (odd_tiles + 16 * (i + 1)) as usize,
                ChunkSide::Positive,
            ))
        }
    }

    ranges.push((
        border_start,
        border_end,
        ChunkSide::Border(positive as usize),
    ));

    if negative_leftover > 0 {
        let full_chunks = negative_leftover / 16;

        for i in 0..full_chunks {
            ranges.push((
                (border_end + (16 * i) as usize),
                (border_end + 16 * (i + 1) as usize),
                ChunkSide::Negative,
            ))
        }

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
    chunk_query: Query<Entity, With<Chunk>>,
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
    chunk_query: Query<(&Transform, &Chunk), With<Chunk>>,
    mut gizmos: Gizmos,
) {
    for (transform, chunk) in chunk_query.iter() {
        // for (y, col) in chunk.grid.iter().enumerate() {
        //     for row in 0..col.len() {
        //         gizmos.rect_2d(
        //             Isometry2d::from_xy(
        //                 transform.translation.x - row as f32 * 64. + 7.5 * 64.,
        //                 transform.translation.y - y as f32 * 64. + 7.5 * 64.,
        //             ),
        //             Vec2 { x: 64., y: 64. },
        //             Color::linear_rgb(0., 0., 0.2 * chunk.grid[y][row] as f32),
        //         );
        //     }
        // }
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2 {
                x: CHUNK_SIZE,
                y: CHUNK_SIZE,
            },
            Color::linear_rgb(0.7, 0., 0.7),
        );
    }
}
