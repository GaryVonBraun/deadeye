use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CampaignOverview;

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignOverviewInteractions {
    MainMenuButton,
    ShopMenuButton,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignSquadInteractions {
    AddMemberButton,
    RemoveMemberButton(usize),
}

#[derive(Component, Debug)]
pub struct SquadMemberList;
