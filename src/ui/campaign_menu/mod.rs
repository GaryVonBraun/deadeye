use bevy::prelude::*;

use crate::{
    core::states::AppState,
    ui::campaign_menu::systems::{behavior::campaign_menu_interactions, layout::*},
};

mod components;
mod systems;
pub struct CampaignMenuPlugin;

impl Plugin for CampaignMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CampaignMenu), spawn_campaign_menu);
        app.add_systems(OnExit(AppState::CampaignMenu), despawn_campaign_menu);
        app.add_systems(Update, campaign_menu_interactions);
    }
}
