use std::{fs, path::PathBuf};

use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    core::io::{read_ron_file, remove_ron_file, write_ron_file},
    map::{
        components::MissionMap,
        io::{
            paths::{manifest_path, map_data_path},
            types::{MapManifest, MapManifestEntry, TileSet},
        },
    },
};

pub fn write_map(map: &MissionMap) {
    info!("saving world map");
    fs::create_dir_all("content/maps/map_data").unwrap();

    let mut manifest = read_or_write_map_manifest();

    manifest.maps.push(MapManifestEntry {
        id: map.id.clone(),
        name: map.name.clone(),
    });

    if let Err(_) = write_ron_file(&manifest, manifest_path()) {
        error!("failed to save updated entry");
        return;
    }
    if let Err(_) = write_ron_file(&map, map_data_path(&map.id)) {
        error!("failed to store world map data");
        return;
    };
}

pub fn update_map_data(map: &MissionMap) {
    if let Err(_) = write_ron_file(&map, map_data_path(&map.id)) {
        error!("failed to store world map data");
        return;
    };
}

pub fn read_map_data(id: &Uuid) -> Result<MissionMap, ()> {
    read_ron_file(map_data_path(id))
}

pub fn read_map_manifest() -> Result<MapManifest, ()> {
    match read_ron_file(manifest_path()) {
        Ok(manifest) => Ok(manifest),
        Err(()) => Err(()),
    }
}

fn read_or_write_map_manifest() -> MapManifest {
    // this function ensure that if there is no manifest we create one

    match read_ron_file(manifest_path()) {
        Ok(manifest) => manifest,
        Err(_) => {
            warn!("manifest not found, creating new manifest");
            MapManifest { maps: vec![] }
        }
    }
}

pub fn read_tileset(path: PathBuf) -> Result<TileSet, ()> {
    let Ok(tileset) = read_ron_file::<TileSet>(path.clone()) else {
        error!("tileset not found: {:?}", path);
        return Err(());
    };
    Ok(tileset)
}

pub fn remove_map(id: Uuid) {
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
