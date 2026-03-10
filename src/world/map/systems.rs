use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use uuid::Uuid;

use crate::{
    core::components::GameEntity,
    world::map::{components::WorldMap, io::save_map},
};

pub fn spawn_tilemap(mut commands: Commands, assets_server: Res<AssetServer>) {
    //TEMPORARY - Everything you see here is hard coded and will be data driven later, this is to check whether everything works
    let texture = assets_server.load_with_settings(
        "prototype_ground_textures.png",
        |settings: &mut ImageLoaderSettings| {
            settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
        },
    );

    let raw_matrix: Vec<Vec<u32>> = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 1, 2, 3],
        vec![1, 1, 1, 1, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
    ];

    commands.spawn((
        WorldMap {
            name: "test_map".to_string(),
            uuid: Uuid::new_v4(),
            tiles: raw_matrix.clone(),
        },
        TilemapChunk {
            chunk_size: UVec2::new(10, 10),      // 20x20 tiles
            tile_display_size: UVec2::splat(64), // each tile is 64x64 pixels
            tileset: texture,
            ..default()
        },
        TilemapChunkTileData(convert_tiles(&raw_matrix)),
        GameEntity,
    ));
}

fn convert_tiles(tiles: &Vec<Vec<u32>>) -> Vec<Option<TileData>> {
    tiles
        .iter()
        .flatten()
        .map(|tile_id| Some(TileData::from_tileset_index(*tile_id as u16)))
        .collect()
}

//TEMPORARY - we create a map on M input just so we can test if everything works
pub fn create_map_on_input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyM) {
        info!("user creating new world map entry");

        let raw_matrix: Vec<Vec<u32>> = vec![
            vec![0, 0, 0, 0, 1, 0, 0, 1, 2, 3],
            vec![1, 1, 1, 1, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 2, 3],
        ];
        let map = WorldMap {
            name: "test map".to_string(),
            uuid: Uuid::new_v4(),
            tiles: raw_matrix,
        };
        save_map(map);
    }
}
