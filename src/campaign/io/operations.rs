use std::{fs, path::PathBuf};

use bevy::prelude::*;

use crate::{
    campaign::{io::paths::*, resources::Campaign},
    core::io::{read_ron_file, write_ron_file},
};

pub fn write_campaign(campaign: &Campaign) {
    info!("saving campaign");
    fs::create_dir_all("content/campaigns").unwrap();

    if let Err(_) = write_ron_file(&campaign, campaign_data_path(&campaign.id)) {
        error!("failed to save campaign data");
        return;
    };
}

pub fn get_campaign_files() -> Vec<PathBuf> {
    let mut campaigns = Vec::new();

    if let Ok(entries) = std::fs::read_dir(campaigns_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "ron") {
                campaigns.push(path);
            }
        }
    }

    campaigns
}

pub fn read_campaign_data(path: PathBuf) -> Result<Campaign, ()> {
    let Ok(campaign) = read_ron_file::<Campaign>(path.clone()) else {
        error!("campaign not found: {:?}", path);
        return Err(());
    };
    Ok(campaign)
}

pub fn list_all_campaign_data() -> Vec<Campaign> {
    let file_paths = get_campaign_files();

    let mut campaigns: Vec<Campaign> = vec![];

    for path in file_paths {
        let Ok(campaign) = read_campaign_data(path) else {
            error!("failed loading campaign data");
            continue;
        };

        campaigns.push(campaign);
    }
    campaigns
}
