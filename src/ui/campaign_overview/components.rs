use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CampaignOverview;

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignOverviewInteractions {
    MainMenuButton,
}
