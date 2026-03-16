use std::path::PathBuf;

use uuid::Uuid;

//TEMPORARY - public now for testing
pub fn missions_manifest_path() -> PathBuf {
    PathBuf::from("content/missions/manifest.ron")
}

pub fn missions_data_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!(
        "content/missions/mission_data/{}.ron",
        id.to_string()
    ))
}
