use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    campaign::{
        io::operations::{list_all_campaign_data, read_campaign_data_from_id, write_campaign},
        messages::LoadCampaignMessage,
        resources::{Campaign, SquadMember},
    },
    core::states::AppState,
};

pub fn create_new_campaign(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let campaign = Campaign {
        id: Uuid::new_v4(),
        name: "new save".to_string(),
        money: 500,
        squad: vec![SquadMember {
            name: "Zack".to_string(),
        }],
    };

    write_campaign(&campaign);
    commands.insert_resource(campaign);
    next_state.set(AppState::CampaignOverview);
}

pub fn load_all_campaign_data() -> Vec<Campaign> {
    list_all_campaign_data()
}

pub fn load_campaign(
    mut load_campaign_reader: MessageReader<LoadCampaignMessage>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for message in load_campaign_reader.read() {
        let Ok(campaign_data) = read_campaign_data_from_id(message.id) else {
            error!(
                "Failed getting data needed for loading campaign: {:?}",
                message.id
            );
            continue;
        };
        commands.insert_resource(campaign_data);
        next_state.set(AppState::CampaignOverview);
    }
}
