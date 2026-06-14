use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::campaign_overview::systems::{behavior::*, layout::*},
};

mod components;
mod systems;
pub struct CampaignOverviewPlugin;

impl Plugin for CampaignOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CampaignOverview), spawn_campaign_overview);
        app.add_systems(
            OnExit(AppState::CampaignOverview),
            despawn_campaign_overview,
        );
        app.add_systems(
            Update,
            campaign_overview_interactions.run_if(in_state(AppState::CampaignOverview)),
        );
    }
}
