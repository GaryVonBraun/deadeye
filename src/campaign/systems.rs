use bevy::prelude::*;
use uuid::Uuid;

use crate::campaign::{
    io::operations::{list_all_campaign_data, read_campaign_data_from_id, write_campaign},
    messages::LoadCampaignMessage,
    resources::Campaign,
};

pub fn create_new_campaign() {
    let campaign = Campaign {
        id: Uuid::new_v4(),
        name: "new save".to_string(),
        money: 500,
    };

    write_campaign(&campaign);
}

pub fn load_all_campaign_data() -> Vec<Campaign> {
    list_all_campaign_data()
}

pub fn load_campaign(
    mut load_campaign_reader: MessageReader<LoadCampaignMessage>,
    mut commands: Commands,
) {
    for message in load_campaign_reader.read() {
        let Ok(campaign_data) = read_campaign_data_from_id(message.id) else {
            error!(
                "Failed getting data needed for loading campaign: {:?}",
                message.id
            );
            continue;
        };

        info!("loaded campaign: {:?}", campaign_data);
        commands.insert_resource(campaign_data);
    }
}
