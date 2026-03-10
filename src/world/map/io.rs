use std::{fs, path::PathBuf};

use bevy::log::{info, tracing_subscriber::fmt::format};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::world::map::components::WorldMap;

#[derive(Serialize, Deserialize)]
pub struct MapManifestEntry {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MapManifest {
    pub maps: Vec<MapManifestEntry>,
}

fn manifest_path() -> PathBuf {
    PathBuf::from("data/maps/manifest.ron")
}
fn map_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("data/maps/data/{}.ron", id.to_string()))
}

pub fn save_map(map: WorldMap) {
    info!("saving world map");
    fs::create_dir_all("data/maps/data").unwrap();

    let mut manifest = load_or_create_manifest();

    manifest.maps.push(MapManifestEntry {
        uuid: map.uuid.clone(),
        name: map.name.clone(),
    });

    let new_manifest = ron::to_string(&manifest).unwrap();
    fs::write(manifest_path(), new_manifest).unwrap();

    let world_map_data = ron::to_string(&map).unwrap();
    fs::write(map_data_path(&map.uuid), world_map_data).unwrap();
}

fn load_or_create_manifest() -> MapManifest {
    // this function ensure that if there is no manifest we create one
    let path = manifest_path();
    if path.exists() {
        info!("manifest found, reading contents");
        let contents = fs::read_to_string(path).unwrap();
        ron::from_str(&contents).unwrap()
    } else {
        info!("manifest not found, creating new manifest");
        MapManifest { maps: vec![] }
    }
}
