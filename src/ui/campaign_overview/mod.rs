use bevy::prelude::*;

use crate::{
    campaign::resources::Campaign,
    core::states::AppState,
    ui::campaign_overview::systems::{behavior::*, entries::populate_squad_member_list, layout::*},
};

mod components;
mod systems;
pub struct CampaignOverviewPlugin;

impl Plugin for CampaignOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::CampaignOverview),
            (spawn_campaign_overview, populate_squad_member_list).chain(),
        );
        app.add_systems(
            OnExit(AppState::CampaignOverview),
            despawn_campaign_overview,
        );
        app.add_systems(
            Update,
            (
                campaign_overview_interactions,
                campaign_squad_interactions,
                populate_squad_member_list.run_if(resource_exists_and_changed::<Campaign>),
            )
                .run_if(in_state(AppState::CampaignOverview)),
        );
    }
}
