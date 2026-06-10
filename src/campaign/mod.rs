use bevy::prelude::*;

use crate::campaign::{
    messages::{CreateCampaignMessage, LoadCampaignMessage},
    systems::{create_new_campaign, load_campaign},
};

mod io;
pub mod messages;
mod resources;
pub mod systems;
pub struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CreateCampaignMessage>();
        app.add_message::<LoadCampaignMessage>();
        app.add_systems(
            Update,
            (
                create_new_campaign.run_if(on_message::<CreateCampaignMessage>),
                load_campaign.run_if(on_message::<LoadCampaignMessage>),
            ),
        );
    }
}
