use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use uuid::Uuid;

use crate::{
    core::{components::GameEntity, io::read_ron_file},
    world::map::{components::WorldMap, io::*, messages::LoadMapMessage},
};

pub fn spawn_world_map(mut messages: MessageWriter<LoadMapMessage>) {
    // load manifest needed for map selection
    let Ok(manifest) = read_ron_file::<MapManifest>(manifest_path()) else {
        error!("failed to get manifest needed for spawning map");
        return;
    };

    let first_manifest = &manifest.maps[0];

    messages.write(LoadMapMessage {
        id: first_manifest.id,
        name: first_manifest.name.clone(),
    });
}

pub fn load_map(
    mut messages: MessageReader<LoadMapMessage>,
    map_query: Query<(Entity, &WorldMap), With<WorldMap>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    //NOTE - currently just taking the first message to prevent loading and unloading if multiple messages are present
    for message in messages.read() {
        info!("loading map: {:?}", message.name);

        // despawn all existing maps
        //NOTE - technically it should not be possible to have multiple maps, but its for safety
        for (map_entity, world_map_info) in map_query.iter() {
            info!("despawning {:?}", world_map_info.name);
            commands.entity(map_entity).despawn();
        }

        //TEMPORARY - The tilemap texture is currently hardcoded, maps might have different textures
        let texture = assets_server.load_with_settings(
            "prototype_ground_textures.png",
            |settings: &mut ImageLoaderSettings| {
                settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
            },
        );

        let Ok(world_map) = load_world_map_data(&message.id) else {
            error!("failed to load world map with id: {:?}", message.id);
            return;
        };

        commands.spawn((
            WorldMap {
                name: "test_map".to_string(),
                id: Uuid::new_v4(),
                tiles: world_map.tiles.clone(),
            },
            TilemapChunk {
                chunk_size: UVec2::new(10, 10),      // 20x20 tiles
                tile_display_size: UVec2::splat(64), // each tile is 64x64 pixels
                tileset: texture,
                ..default()
            },
            TilemapChunkTileData(convert_tiles(&world_map.tiles)),
            GameEntity,
        ));
    }
}

fn convert_tiles(tiles: &Vec<Vec<u32>>) -> Vec<Option<TileData>> {
    tiles
        .iter()
        .flatten()
        .map(|tile_id| Some(TileData::from_tileset_index(*tile_id as u16)))
        .collect()
}

//TEMPORARY - we temporary controls for testing
pub fn map_input_actions(
    keys: Res<ButtonInput<KeyCode>>,
    mut messages: MessageWriter<LoadMapMessage>,
) {
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
        let world_map = WorldMap {
            name: "test map".to_string(),
            id: Uuid::new_v4(),
            tiles: raw_matrix,
        };
        save_world_map(world_map);
    }
    if keys.just_pressed(KeyCode::KeyL) {
        let Ok(manifest) = read_ron_file::<MapManifest>(manifest_path()) else {
            error!("failed to get manifest needed for spawning map");
            return;
        };

        let first_manifest = &manifest.maps[1];

        messages.write(LoadMapMessage {
            id: first_manifest.id,
            name: first_manifest.name.clone(),
        });
    }
}
