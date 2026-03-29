use std::fs;

use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    core::io::{read_ron_file, remove_ron_file, write_ron_file},
    mission::{
        io::{
            paths::{missions_data_path, missions_manifest_path},
            types::*,
        },
        resources::Mission,
    },
};

pub fn write_mission(mission: &Mission) {
    info!("saving mission");
    fs::create_dir_all("content/missions/mission_data").unwrap();

    let mut manifest = read_or_write_missions_manifest();

    manifest.missions.push(MissionsManifestEntry {
        id: mission.id.clone(),
        name: mission.name.clone(),
        map_id: mission.map_id.clone(),
    });

    if let Err(_) = write_ron_file(&manifest, missions_manifest_path()) {
        error!("failed to save updated entry");
        return;
    }
    if let Err(_) = write_ron_file(&mission, missions_data_path(&mission.id)) {
        error!("failed to store world map data");
        return;
    };
}

pub fn read_mission_data(id: &Uuid) -> Result<Mission, ()> {
    read_ron_file(missions_data_path(id))
}

pub fn read_missions_manifest() -> Result<MissionsManifest, ()> {
    match read_ron_file(missions_manifest_path()) {
        Ok(manifest) => Ok(manifest),
        Err(()) => Err(()),
    }
}

fn read_or_write_missions_manifest() -> MissionsManifest {
    // this function ensure that if there is no manifest we create one

    match read_ron_file(missions_manifest_path()) {
        Ok(manifest) => manifest,
        Err(_) => {
            warn!("manifest not found, creating new manifest");
            MissionsManifest { missions: vec![] }
        }
    }
}

pub fn remove_mission_file(id: Uuid) {
    info!("deleting: {:?}", id);

    let Ok(mut manifest) = read_missions_manifest() else {
        error!("cannot find manifest");
        return;
    };

    if let Some(index) = manifest.missions.iter().position(|map| map.id == id) {
        let deleted_entry = manifest.missions.swap_remove(index);
        if let Err(_) = write_ron_file(&manifest, missions_manifest_path()) {
            error!("failed to save updated entry");
            return;
        }
        info!("removed entry from manifest: {:?}", deleted_entry.id);
    }

    if let Err(err) = remove_ron_file(missions_data_path(&id)) {
        error!("failed to remove map data file for: {:?}{:?}", err, id);
    }
}
