use std::path::PathBuf;

use uuid::Uuid;

//TEMPORARY - public now for testing
pub fn manifest_path() -> PathBuf {
    PathBuf::from("content/maps/manifest.ron")
}

pub fn map_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("content/maps/map_data/{}.ron", id.to_string()))
}

pub fn tileset_path(name: String) -> PathBuf {
    PathBuf::from(format!("content/maps/tilesets/{}.ron", name))
}
