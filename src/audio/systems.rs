use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{audio::resources::AudioRegistry, core::io::read_ron_file};

#[derive(Debug, Serialize, Deserialize)]
struct AudioEntry {
    id: String,
    path: String,
}

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("setting up audio");
    let Ok(audio_entries) =
        read_ron_file::<Vec<AudioEntry>>(PathBuf::from("content/audio_registry.ron"))
    else {
        error!("failed to find audio registry");
        return;
    };
    let mut sounds: HashMap<String, Handle<AudioSource>> = HashMap::new();
    for entry in audio_entries {
        let asset: Handle<AudioSource> = asset_server.load(&entry.path);
        sounds.insert(entry.id, asset);
    }

    commands.insert_resource(AudioRegistry { sounds });
}
