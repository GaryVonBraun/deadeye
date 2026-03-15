use std::fs;

use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    core::io::{read_ron_file, write_ron_file},
    world::map::{
        components::WorldMap,
        io::{
            paths::{manifest_path, map_data_path},
            types::{MapManifest, MapManifestEntry},
        },
    },
};

pub fn save_world_map(world_map: &WorldMap) {
    info!("saving world map");
    fs::create_dir_all("content/maps/map_data").unwrap();

    let mut manifest = load_or_create_manifest();

    manifest.maps.push(MapManifestEntry {
        id: world_map.id.clone(),
        name: world_map.name.clone(),
    });

    if let Err(_) = write_ron_file(&manifest, manifest_path()) {
        error!("failed to save updated entry");
        return;
    }
    if let Err(_) = write_ron_file(&world_map, map_data_path(&world_map.id)) {
        error!("failed to store world map data");
        return;
    };
}

pub fn load_world_map_data(id: &Uuid) -> Result<WorldMap, ()> {
    match read_world_map_data(id) {
        Ok(map_data) => Ok(map_data),
        Err(()) => Err(()),
    }
}

fn read_world_map_data(id: &Uuid) -> Result<WorldMap, ()> {
    read_ron_file(map_data_path(id))
}

pub fn read_manifest() -> Result<MapManifest, ()> {
    match read_ron_file(manifest_path()) {
        Ok(manifest) => Ok(manifest),
        Err(()) => Err(()),
    }
}

fn load_or_create_manifest() -> MapManifest {
    // this function ensure that if there is no manifest we create one

    match read_ron_file(manifest_path()) {
        Ok(manifest) => manifest,
        Err(_) => {
            warn!("manifest not found, creating new manifest");
            MapManifest { maps: vec![] }
        }
    }
}
