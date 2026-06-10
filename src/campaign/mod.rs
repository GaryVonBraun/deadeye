use bevy::prelude::*;

use crate::campaign::{messages::CreateCampaignMessage, systems::create_new_campaign};

mod io;
pub mod messages;
mod resources;
mod systems;
pub struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CreateCampaignMessage>();
        app.add_systems(
            Update,
            create_new_campaign.run_if(on_message::<CreateCampaignMessage>),
        );
    }
}
