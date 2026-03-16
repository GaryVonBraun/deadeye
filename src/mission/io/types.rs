use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MissionsManifestEntry {
    pub id: Uuid,
    pub name: String,
    pub map_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MissionsManifest {
    pub missions: Vec<MissionsManifestEntry>,
}
