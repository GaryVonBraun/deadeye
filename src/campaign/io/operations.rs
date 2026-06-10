use std::fs;

use bevy::prelude::*;

use crate::{
    campaign::{io::paths::campaign_data_path, resources::Campaign},
    core::io::write_ron_file,
};

pub fn write_campaign(campaign: &Campaign) {
    info!("saving campaign");
    fs::create_dir_all("content/campaigns").unwrap();

    if let Err(_) = write_ron_file(&campaign, campaign_data_path(&campaign.id)) {
        error!("failed to save campaign data");
        return;
    };
}
