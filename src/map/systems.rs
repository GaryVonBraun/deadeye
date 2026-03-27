use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::{
    core::{components::GameEntity, io::read_ron_file},
    map::{
        components::MissionMap,
        io::{operations::*, paths::*, types::MapManifest},
        messages::{DeleteMapMessage, LoadMapMessage},
        resources::ActiveMap,
    },
};

pub fn spawn_map(mut message_writer: MessageWriter<LoadMapMessage>) {
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

    message_writer.write(LoadMapMessage {
        id: first_manifest.id,
    });
}

pub fn delete_map_message(mut message_reader: MessageReader<DeleteMapMessage>) {
    for message in message_reader.read() {
        remove_map(message.id);
    }
}

pub fn load_map_data(
    mut message_reader: MessageReader<LoadMapMessage>,
    map_query: Query<(Entity, &MissionMap), With<MissionMap>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    //NOTE - currently just taking the first message to prevent loading and unloading if multiple messages are present
    for message in message_reader.read() {
        // despawn all existing maps
        //NOTE - technically it should not be possible to have multiple maps, but its for safety
        for (map_entity, map_info) in map_query.iter() {
            info!("despawning {:?}", map_info.name);
            commands.entity(map_entity).despawn();
        }

        let Ok(map) = read_map_data(&message.id) else {
            error!("failed to load world map with id: {:?}", message.id);
            return;
        };

        let Ok(tileset) = read_tileset(tileset_path(map.tileset_name.clone())) else {
            info!("failed to load tileset needed for loading map");
            return;
        };

        //TEMPORARY - The tilemap texture is currently hardcoded, maps might have different textures
        // let texture = assets_server.load_with_settings(
        //     tileset.texture.clone(),
        //     |settings: &mut ImageLoaderSettings| {
        //         settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
        //     },
        // );

        // commands.spawn((
        //     MissionMap {
        //         name: map.name.clone(),
        //         id: map.id,
        //         tiles: map.tiles.clone(),
        //         tileset_name: map.tileset_name.clone(),
        //         bounds: map.bounds.clone(),
        //     },
        //     TilemapChunk {
        //         chunk_size: UVec2::new(map.tiles[0].len() as u32, map.tiles.len() as u32),
        //         tile_display_size: UVec2::splat(64), // each tile is 64x64 pixels
        //         tileset: texture,
        //         ..default()
        //     },
        //     TilemapChunkTileData(convert_tiles(&map.tiles)),
        //     GameEntity,
        // ));
        commands.insert_resource(ActiveMap { map, tileset });
    }
}

//FIXME - this system is almost the same as load_map, should see to re-use the code
pub fn load_map_from_resource(
    active_map: Res<ActiveMap>,
    assets_server: Res<AssetServer>,
    map_query: Query<(Entity, &MissionMap), With<MissionMap>>,
    mut commands: Commands,
) {
    // despawn all existing maps
    //NOTE - technically it should not be possible to have multiple maps, but its for safety
    for (map_entity, map_info) in map_query.iter() {
        info!("despawning {:?}", map_info.name);
        commands.entity(map_entity).despawn();
    }

    //TEMPORARY - The tilemap texture is currently hardcoded, maps might have different textures
    let texture = assets_server.load_with_settings(
        active_map.tileset.texture.clone(),
        |settings: &mut ImageLoaderSettings| {
            settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
        },
    );

    let map = &active_map.map;

    commands.spawn((
        MissionMap {
            name: map.name.clone(),
            id: map.id,
            tiles: map.tiles.clone(),
            tileset_name: map.tileset_name.clone(),
            bounds: map.bounds.clone(),
        },
        TilemapChunk {
            chunk_size: UVec2::new(map.tiles[0].len() as u32, map.tiles.len() as u32),
            tile_display_size: UVec2::splat(64), // each tile is 64x64 pixels
            tileset: texture,
            ..default()
        },
        TilemapChunkTileData(convert_tiles(&map.tiles)),
        GameEntity,
    ));
}

pub fn convert_tiles(tiles: &Vec<Vec<u32>>) -> Vec<Option<TileData>> {
    tiles
        .iter()
        .rev() // flip row order
        .flatten()
        .map(|tile_id| Some(TileData::from_tileset_index(*tile_id as u16)))
        .collect()
}
