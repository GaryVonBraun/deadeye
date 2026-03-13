use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use uuid::Uuid;

use crate::{
    core::io::{read_ron_file, write_ron_file},
    world::map::components::WorldMap,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifestEntry {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifest {
    pub maps: Vec<MapManifestEntry>,
}

//TEMPORARY - public now for testing
pub fn manifest_path() -> PathBuf {
    PathBuf::from("data/maps/manifest.ron")
}

pub fn map_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("data/maps/data/{}.ron", id.to_string()))
}

pub fn save_world_map(world_map: &WorldMap) {
    info!("saving world map");
    fs::create_dir_all("data/maps/data").unwrap();

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
