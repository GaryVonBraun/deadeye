use bevy::prelude::*;
use uuid::Uuid;

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

#[derive(Component, Debug, Clone, Copy)]
pub enum CampaignMissionInteractions {
    SelectMission(Uuid),
    StartMission(Uuid),
}

#[derive(Component, Debug)]
pub struct MissionEntry {
    pub id: Uuid,
}

#[derive(Component, Debug)]
pub struct SquadMemberList;

#[derive(Component, Debug)]
pub struct UiMissionList;

#[derive(Component, Debug)]
pub struct UiMissionBriefing;
