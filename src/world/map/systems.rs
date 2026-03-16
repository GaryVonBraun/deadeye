use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use rand::RngExt;
use uuid::Uuid;

use crate::{
    core::{
        components::GameEntity,
        io::{read_ron_file, remove_ron_file, write_ron_file},
    },
    ui::mission_dev_menu::messages::RefreshMissionDevListMessage,
    world::map::{
        components::WorldMap,
        io::{
            operations::{read_map_data, read_map_manifest, read_tileset, write_map},
            paths::{manifest_path, map_data_path, tileset_path},
            types::MapManifest,
            *,
        },
        messages::{DeleteMissionMessage, LoadMissionMessage},
    },
};

pub fn spawn_world_map(mut message_writer: MessageWriter<LoadMissionMessage>) {
    // load manifest needed for map selection
    let Ok(manifest) = read_ron_file::<MapManifest>(manifest_path()) else {
        error!("failed to get manifest needed for spawning map");
        return;
    };

    if manifest.maps.is_empty() {
        error!("no maps found in manifest");
        return;
    }

    let first_manifest = &manifest.maps[0];

    message_writer.write(LoadMissionMessage {
        id: first_manifest.id,
    });
}

fn remove_map_file(id: Uuid) {
    info!("deleting: {:?}", id);

    let Ok(mut manifest) = read_map_manifest() else {
        error!("cannot find manifest");
        return;
    };

    if let Some(index) = manifest.maps.iter().position(|map| map.id == id) {
        let deleted_entry = manifest.maps.swap_remove(index);
        if let Err(_) = write_ron_file(&manifest, manifest_path()) {
            error!("failed to save updated entry");
            return;
        }
        info!("removed entry from manifest: {:?}", deleted_entry.id);
    }

    if let Err(err) = remove_ron_file(map_data_path(&id)) {
        error!("failed to remove map data file for: {:?}{:?}", err, id);
    }
}

pub fn handle_delete_map_message(
    mut message_reader: MessageReader<DeleteMissionMessage>,
    mut resfresh_map_message_writer: MessageWriter<RefreshMissionDevListMessage>,
) {
    for message in message_reader.read() {
        remove_map_file(message.id);
    }
    resfresh_map_message_writer.write(RefreshMissionDevListMessage);
}

pub fn load_map_data(
    mut message_reader: MessageReader<LoadMissionMessage>,
    map_query: Query<(Entity, &WorldMap), With<WorldMap>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    //NOTE - currently just taking the first message to prevent loading and unloading if multiple messages are present
    for message in message_reader.read() {
        // despawn all existing maps
        //NOTE - technically it should not be possible to have multiple maps, but its for safety
        for (map_entity, world_map_info) in map_query.iter() {
            info!("despawning {:?}", world_map_info.name);
            commands.entity(map_entity).despawn();
        }

        let Ok(world_map) = read_map_data(&message.id) else {
            error!("failed to load world map with id: {:?}", message.id);
            return;
        };

        let Ok(tileset) = read_tileset(tileset_path(world_map.tileset_name.clone())) else {
            return;
        };

        //TEMPORARY - The tilemap texture is currently hardcoded, maps might have different textures
        let texture = assets_server.load_with_settings(
            tileset.texture,
            |settings: &mut ImageLoaderSettings| {
                settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
            },
        );

        commands.spawn((
            WorldMap {
                name: "test_map".to_string(),
                id: Uuid::new_v4(),
                tiles: world_map.tiles.clone(),
                tileset_name: world_map.tileset_name,
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

pub fn create_new_map() -> WorldMap {
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

    let mut rng = rand::rng();

    let world_map = WorldMap {
        name: format!("test map {:?}", rng.random_range(1..1000)).to_string(),
        id: Uuid::new_v4(),
        tiles: raw_matrix,
        tileset_name: "base".to_string(),
    };
    write_map(&world_map);
    world_map
}

fn convert_tiles(tiles: &Vec<Vec<u32>>) -> Vec<Option<TileData>> {
    tiles
        .iter()
        .flatten()
        .map(|tile_id| Some(TileData::from_tileset_index(*tile_id as u16)))
        .collect()
}
