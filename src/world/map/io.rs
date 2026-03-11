use std::{fs, path::PathBuf};

use bevy::log::{error_span, info, tracing_subscriber::fmt::format};
use ron::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{core::io::read_ron_file, world::map::components::WorldMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifestEntry {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapManifest {
    pub maps: Vec<MapManifestEntry>,
}

fn manifest_path() -> PathBuf {
    PathBuf::from("data/maps/manifest.ron")
}
fn map_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("data/maps/data/{}.ron", id.to_string()))
}

pub fn save_world_map(world_map: WorldMap) {
    info!("saving world map");
    fs::create_dir_all("data/maps/data").unwrap();

    let mut manifest = load_or_create_manifest();

    manifest.maps.push(MapManifestEntry {
        id: world_map.uuid.clone(),
        name: world_map.name.clone(),
    });

    write_manifest_data(manifest);

    write_world_map_data(world_map);
}

pub fn load_world_map_data(id: &Uuid) {
    let map_data = read_world_map_data(id);
    info!("{:?}", map_data);
}

fn write_world_map_data(world_map: WorldMap) {
    let world_map_data = ron::to_string(&world_map).unwrap();
    fs::write(map_data_path(&world_map.uuid), world_map_data).unwrap();
}

fn write_manifest_data(manifest: MapManifest) {
    let new_manifest = ron::to_string(&manifest).unwrap();
    fs::write(manifest_path(), new_manifest).unwrap();
}

fn read_world_map_data(id: &Uuid) -> Result<WorldMap, String> {
    read_ron_file(map_data_path(id))
}

pub fn read_manifest() -> Result<MapManifest, String> {
    read_ron_file(manifest_path())
}

fn load_or_create_manifest() -> MapManifest {
    // this function ensure that if there is no manifest we create one

    if let Ok(manifest) = read_manifest() {
        manifest
    } else {
        info!("manifest not found, creating new manifest");
        MapManifest { maps: vec![] }
    }
}
